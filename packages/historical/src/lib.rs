trait Histrical : Default{
    type Item: HistricalItem;
    fn reduce(self,current: &Self::Item) -> Self;
    fn calculate(items: Vec<Self::Item>,current: usize) -> Self {
        Self::calculate_latest(items.into_iter().filter(|item| item.index() <= current).collect())
    }
    fn calculate_latest(mut items: Vec<Self::Item>) -> Self {
        items.sort_by_key(|item| (item.index(),-(item.branch() as i32)));
        items.into_iter().fold((0 as usize,Self::default()), |(branch,acc),current| {
            if branch > current.branch() {
                (branch,acc)
            } else {
                (current.branch(),acc.reduce(&current))
            }

        }).1
    }
}

trait HistricalItem {
    fn index(&self) -> usize;
    fn branch(&self) -> usize;
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
        fn index(&self) -> usize {
            self.index
        }
        fn branch(&self) -> usize {
            self.branch
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
    fn test_histrical() {
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
}