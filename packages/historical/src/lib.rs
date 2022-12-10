#[derive(Debug,Clone,PartialEq,Eq,Default,serde::Serialize,serde::Deserialize)]
pub struct HistoricalSignature {
    pub index: usize,
    pub branch: usize,
}

pub fn next_signature<T: HistricalItem>(items: &Vec<T>,current: Option<usize>) -> HistoricalSignature {
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
        HistoricalSignature {
            index: items.len(),
            branch: 0,
        }
    }
}

pub fn calculate_latest<T: HistricalItem>(mut items: Vec<T>) -> T::Collected {
    items.sort_by_key(|item| (item.signature().index,-(item.signature().branch as i32)));
    items.into_iter().fold((0 as usize,T::Collected::default()), |(branch,mut acc),current| {
        if branch > current.signature().branch {
            (branch,acc)
        } else {
            let branch = current.signature().branch;
            current.apply(&mut acc);
            (branch,acc)
        }
    }).1
}

pub fn calculate<T: HistricalItem>(items: Vec<T>,current: Option<usize>) -> T::Collected {
    current.map(|current| calculate_latest(items.into_iter().filter(|item| item.signature().index <= current).collect())).unwrap_or_default()
}

pub trait HistricalItem : Sized{
    type Collected : Default;
    fn signature(&self) -> HistoricalSignature;
    fn apply(self,acc: &mut Self::Collected);
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
        type Collected = TestHistrical;
        fn signature(&self) -> HistoricalSignature {
            HistoricalSignature {
                index: self.index,
                branch: self.branch,
            }
        }
        fn apply(self,acc: &mut Self::Collected) {
            acc.text.push_str(self.text.as_str());
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
        assert_eq!(calculate(items.clone(),2.into()),TestHistrical {
            text: "abz".to_owned(),
        });
        assert_eq!(calculate(items.clone(),3.into()),TestHistrical {
            text: "abzy".to_owned(),
        });
        assert_eq!(calculate_latest(items.clone()),TestHistrical {
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
        assert_eq!(next_signature(&items,0.into()),HistoricalSignature {
            index: 1,
            branch: 1,
        });
        assert_eq!(next_signature(&items,1.into()),HistoricalSignature {
            index: 2,
            branch: 2,
        });
        assert_eq!(next_signature(&items,2.into()),HistoricalSignature {
            index: 3,
            branch: 2,
        });
        assert_eq!(next_signature(&items,3.into()),HistoricalSignature {
            index: 4,
            branch: 1,
        });
    }


    #[test]
    fn test_historical_e2e() {
        let mut history = Vec::new();
        let historical = next_signature(&history,None);
        history.push(TestHistricalItem {
            index: historical.index,
            branch: historical.branch,
            text: "a".to_owned(),
        });
        assert_eq!(calculate_latest(history.clone()),TestHistrical {
            text: "a".to_owned(),
        });
        let historical = next_signature(&history,0.into());
        history.push(TestHistricalItem {
            index: historical.index,
            branch: historical.branch,
            text: "b".to_owned(),
        });
        assert_eq!(calculate_latest(history.clone()),TestHistrical {
            text: "ab".to_owned(),
        });
        assert_eq!(calculate(history.clone(),0.into()),TestHistrical {
            text: "a".to_owned(),
        });
        let historical = next_signature(&history,1.into());
        history.push(TestHistricalItem {
            index: historical.index,
            branch: historical.branch,
            text: "c".to_owned(),
        });
        assert_eq!(calculate_latest(history.clone()),TestHistrical {
            text: "abc".to_owned(),
        });
        let historical = next_signature(&history,0.into());
        history.push(TestHistricalItem {
            index: historical.index,
            branch: historical.branch,
            text: "d".to_owned(),
        });
        assert_eq!(calculate_latest(history.clone()),TestHistrical {
            text: "ad".to_owned(),
        });
        let historical = next_signature(&history,1.into());
        history.push(TestHistricalItem {
            index: historical.index,
            branch: historical.branch,
            text: "e".to_owned(),
        });
        assert_eq!(calculate_latest(history.clone()),TestHistrical {
            text: "ade".to_owned(),
        });
        let historical = next_signature(&history,1.into());
        history.push(TestHistricalItem {
            index: historical.index,
            branch: historical.branch,
            text: "f".to_owned(),
        });
        assert_eq!(calculate_latest(history.clone()),TestHistrical {
            text: "adf".to_owned(),
        });
    }
}