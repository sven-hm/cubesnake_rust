use std::rc::Rc;

pub struct Node<T> {
    pub father: Option<Rc<Node<T>>>,
    pub value: T,
}

pub struct TreeIterator<T> {
    curr: Option<Rc<Node<T>>>,
}

impl<T> Iterator for TreeIterator<T> {
    type Item = Rc<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        // set return value
        let retval = match &self.curr {
            None => None,
            Some(cc) => Some(Rc::clone(&cc)),
        };

        // update current value
        self.curr = match &self.curr {
            None => None,
            Some(cc) => match &cc.father {
                None => None,
                Some(ff) => Some(Rc::clone(&ff)),
            },
        };

        retval
    }
}

impl<T> TreeIterator<T> {
    pub fn new(startnode: Rc<Node<T>>) -> TreeIterator<T> {
        TreeIterator::<T> {
            curr: Some(Rc::clone(&startnode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_tree() {
        let a = Rc::new(Node::<i32> {
            father: None,
            value: 0,
        });
        let b = Rc::new(Node::<i32> {
            father: Some(Rc::clone(&a)),
            value: 1,
        });
        let c = Rc::new(Node::<i32> {
            father: Some(Rc::clone(&a)),
            value: 2,
        });
        let d = Rc::new(Node::<i32> {
            father: Some(Rc::clone(&b)),
            value: 3,
        });
        let e = Rc::new(Node::<i32> {
            father: Some(Rc::clone(&c)),
            value: 4,
        });
        let f = Rc::new(Node::<i32> {
            father: Some(Rc::clone(&c)),
            value: 5,
        });
        let g = Rc::new(Node::<i32> {
            father: Some(Rc::clone(&f)),
            value: 6,
        });

        let mut r = TreeIterator::new(Rc::clone(&g));
        assert_eq!(r.next().unwrap().value, 6);
        assert_eq!(r.next().unwrap().value, 5);
        assert_eq!(r.next().unwrap().value, 2);
        assert_eq!(r.next().unwrap().value, 0);

        r = TreeIterator::new(Rc::clone(&e));
        assert_eq!(r.next().unwrap().value, 4);
        assert_eq!(r.next().unwrap().value, 2);
        assert_eq!(r.next().unwrap().value, 0);

        r = TreeIterator::new(Rc::clone(&d));
        assert_eq!(r.next().unwrap().value, 3);
        assert_eq!(r.next().unwrap().value, 1);
        assert_eq!(r.next().unwrap().value, 0);
    }

    #[test]
    fn test_string_tree() {
        let a = Rc::new(Node::<String> {
            father: None,
            value: "a".to_string(),
        });
        let b = Rc::new(Node::<String> {
            father: Some(Rc::clone(&a)),
            value: "b".to_string(),
        });

        let mut r = TreeIterator::new(Rc::clone(&b));
        assert_eq!(r.next().unwrap().value, "b");
        assert_eq!(r.next().unwrap().value, "a");
    }

    #[test]
    fn test_box_tree() {
        let mut node = Rc::new(Node::<Box<i32>> {
            father: None,
            value: Box::new(0),
        });

        for ii in 1..10 {
            let child = Rc::new(Node::<Box<i32>> {
                father: Some(Rc::clone(&node)),
                value: Box::new(ii),
            });

            node = Rc::clone(&child);
        }

        let mut expected_value = 10;
        for rr in TreeIterator::new(Rc::clone(&node)) {
            expected_value -= 1;
            assert_eq!(expected_value, *rr.value);
        }
    }
}
