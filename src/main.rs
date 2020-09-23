mod tree;
mod snake;
mod area;
mod path;

use std::rc::Rc;

use crate::tree::{Node, TreeIterator};
//use crate::brick::{Chain};

fn main() {
    let mut node = Rc::new(Node::<Box<i32>> {
        father: None,
        value: Box::new(0)
    });

    for ii in 1..10 {
        let child = Rc::new(Node::<Box<i32>> {
            father: Some(Rc::clone(&node)),
            value: Box::new(ii)
        });

        node = Rc::clone(&child);
    }

    for rr in TreeIterator::new(Rc::clone(&node)) {
        println!("value = {}", *rr.value);
    }

    //let _ch = Chain::new();
}
