#[derive(Debug,Clone,PartialEq,Eq,Default,serde::Serialize,serde::Deserialize)]
pub struct HistoricalSignature {
    pub index: usize,
}

pub fn next_signature<T: HistricalItem>(items: &Vec<T>) -> HistoricalSignature {
    HistoricalSignature {
        index: items.len(),
    }
}

pub fn calculate_latest<T: HistricalItem>(mut items: Vec<T>) -> T::Collected {
    items.sort_by_key(|item| item.signature().index);
    items.into_iter().fold((0 as usize,T::Collected::default()), |(branch,mut acc),current| {
        current.apply(&mut acc);
        (branch,acc)
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
        text: String,
    }
    impl HistricalItem for TestHistricalItem {
        type Collected = TestHistrical;
        fn signature(&self) -> HistoricalSignature {
            HistoricalSignature {
                index: self.index,
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
                text: "a".to_owned(),
            },
            TestHistricalItem {
                index: 1,
                text: "b".to_owned(),
            },
            TestHistricalItem {
                index: 2,
                text: "c".to_owned(),
            },
            
            TestHistricalItem {
                index: 3,
                text: "d".to_owned(),
            },
            TestHistricalItem {
                index: 2,
                text: "z".to_owned(),
            },
            TestHistricalItem {
                index: 3,
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
                text: "a".to_owned(),
            },
            TestHistricalItem {
                index: 1,
                text: "b".to_owned(),
            },
            TestHistricalItem {
                index: 2,
                text: "c".to_owned(),
            },
            
            TestHistricalItem {
                index: 3,
                text: "d".to_owned(),
            },
            TestHistricalItem {
                index: 2,
                text: "z".to_owned(),
            },
            TestHistricalItem {
                index: 3,
                text: "y".to_owned(),
            },
        ];
        assert_eq!(next_signature(&items),HistoricalSignature {
            index: 1,
        });
    }

}