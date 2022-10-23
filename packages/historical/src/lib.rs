trait Histrical : Default{
    type Item: HistricalItem;
    fn reduce(self,current: &Self::Item) -> Self;
    fn calculate(items: Vec<Self::Item>,current: usize) -> Self {
        Self::calculate_latest(items.into_iter().filter(|item| item.signature().index <= current).collect())
    }
    fn calculate_latest(mut items: Vec<Self::Item>) -> Self {
        items.sort_by_key(|item| (item.signature().index,-(item.signature().branch as i32)));
        items.into_iter().fold((0 as usize,Self::default()), |(branch,acc),current| {
            if branch > current.signature().branch {
                (branch,acc)
            } else {
                (current.signature().branch,acc.reduce(&current))
            }

        }).1
    }

    fn next_signature(items: &Vec<Self::Item>,current: Option<usize>) -> HistoricalSignature {
        if let Some(current) = current {
            let next_index = current + 1;
            let get_max_branch_with_index = |index| {
                items.iter().filter(|item| item.signature().index == index).map(|item| item.signature().branch).max()
            };
            let current_max_branch  = get_max_branch_with_index(current).expect("current index must be exist");
            let next_max_branch = get_max_branch_with_index(next_index);
            HistoricalSignature {
                index: next_index,
                branch: next_max_branch.map(|b| b + 1).unwrap_or(current_max_branch),
            }
        } else {
            HistoricalSignature::default()
        }
        
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Default)]
pub struct HistoricalSignature {
    pub index: usize,
    pub branch: usize,
}

trait HistricalItem {
    fn signature(&self) -> HistoricalSignature;
}

#[cfg(test)]
mod test {
    use super::*;
    #[derive(Default,Clone,PartialEq,Debug)]
    struct TestHistrical {
        text: String,
    }
    #[derive(Clone,PartialEq,Debug)]
    struct TestHistricalItem {
        index: usize,
        branch: usize,
        text: String,
    }
    impl HistricalItem for TestHistricalItem {
        fn signature(&self) -> HistoricalSignature {
            HistoricalSignature {
                index: self.index,
                branch: self.branch,
            }
        }
    }
    impl Histrical for TestHistrical {
        type Item = TestHistricalItem;
        fn reduce(self,current: &Self::Item) -> Self {
            Self {
                text: self.text + current.text.as_str()
            }
        }
    }
    #[test]
    fn test_calculate() {
        let items = vec![
            TestHistricalItem {
                index: 0,
                branch: 0,
                text: "a".to_owned(),
            },
            TestHistricalItem {
                index: 1,
                branch: 0,
                text: "b".to_owned(),
            },
            TestHistricalItem {
                index: 2,
                branch: 0,
                text: "c".to_owned(),
            },
            
            TestHistricalItem {
                index: 3,
                branch: 0,
                text: "d".to_owned(),
            },
            TestHistricalItem {
                index: 2,
                branch: 1,
                text: "z".to_owned(),
            },
            TestHistricalItem {
                index: 3,
                branch: 1,
                text: "y".to_owned(),
            },
        ];
        assert_eq!(TestHistrical::calculate(items.clone(),2),TestHistrical {
            text: "abz".to_owned(),
        });
        assert_eq!(TestHistrical::calculate(items.clone(),3),TestHistrical {
            text: "abzy".to_owned(),
        });
        assert_eq!(TestHistrical::calculate_latest(items.clone()),TestHistrical {
            text: "abzy".to_owned(),
        });
    }

    #[test]
    fn test_next_signature(){
        let items = vec![
            TestHistricalItem {
                index: 0,
                branch: 0,
                text: "a".to_owned(),
            },
            TestHistricalItem {
                index: 1,
                branch: 0,
                text: "b".to_owned(),
            },
            TestHistricalItem {
                index: 2,
                branch: 0,
                text: "c".to_owned(),
            },
            
            TestHistricalItem {
                index: 3,
                branch: 0,
                text: "d".to_owned(),
            },
            TestHistricalItem {
                index: 2,
                branch: 1,
                text: "z".to_owned(),
            },
            TestHistricalItem {
                index: 3,
                branch: 1,
                text: "y".to_owned(),
            },
        ];
        assert_eq!(TestHistrical::next_signature(&items,0.into()),HistoricalSignature {
            index: 1,
            branch: 1,
        });
        assert_eq!(TestHistrical::next_signature(&items,1.into()),HistoricalSignature {
            index: 2,
            branch: 2,
        });
        assert_eq!(TestHistrical::next_signature(&items,2.into()),HistoricalSignature {
            index: 3,
            branch: 2,
        });
        assert_eq!(TestHistrical::next_signature(&items,3.into()),HistoricalSignature {
            index: 4,
            branch: 1,
        });
    }


    #[test]
    fn test_historical_e2e() {
        let mut history = Vec::new();
        let historical = TestHistrical::next_signature(&history,None);
        history.push(TestHistricalItem {
            index: historical.index,
            branch: historical.branch,
            text: "a".to_owned(),
        });
        assert_eq!(TestHistrical::calculate_latest(history.clone()),TestHistrical {
            text: "a".to_owned(),
        });
        let historical = TestHistrical::next_signature(&history,0.into());
        history.push(TestHistricalItem {
            index: historical.index,
            branch: historical.branch,
            text: "b".to_owned(),
        });
        assert_eq!(TestHistrical::calculate_latest(history.clone()),TestHistrical {
            text: "ab".to_owned(),
        });
        assert_eq!(TestHistrical::calculate(history.clone(),0),TestHistrical {
            text: "a".to_owned(),
        });
        let historical = TestHistrical::next_signature(&history,1.into());
        history.push(TestHistricalItem {
            index: historical.index,
            branch: historical.branch,
            text: "c".to_owned(),
        });
        assert_eq!(TestHistrical::calculate_latest(history.clone()),TestHistrical {
            text: "abc".to_owned(),
        });
        let historical = TestHistrical::next_signature(&history,0.into());
        history.push(TestHistricalItem {
            index: historical.index,
            branch: historical.branch,
            text: "d".to_owned(),
        });
        assert_eq!(TestHistrical::calculate_latest(history.clone()),TestHistrical {
            text: "ad".to_owned(),
        });

    }
}