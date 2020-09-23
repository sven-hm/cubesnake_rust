pub mod path {
    use std::vec::Vec;
    use std::mem::swap;
    use std::cmp::{min};
    pub use std::rc::Rc;
    pub use crate::snake::chain::{Chain, Form};
    pub use crate::snake::brick::{Brick, Orientation};
    pub use crate::area::area::Area;
    use crate::tree::{Node, TreeIterator};

    pub struct Path {
        pub area: Area,
        pub chain: Chain,

        last_layer: Vec<Rc<Node<Brick>>>,
        last_layer_index: usize
    }

    impl Path {
        pub fn new(area: Area, chain: Chain) -> Path {
            Path {
                area: area,
                chain: chain,
                last_layer: Vec::new(),
                last_layer_index: 0
            }
        }

        pub fn add_brick(&mut self, brick: Brick, father: Option<Rc<Node<Brick>>>) {
            self.last_layer.push(Rc::new(Node::<Brick> {
                father: father,
                value: brick
            }));
            self.last_layer_index = 1;
        }

        pub fn fold(&mut self) -> usize {
            let mut lsize = 0;
            for ii in 0..self.chain.len() {
                match self.chain.get(self.last_layer_index + ii) {
                    Some(frm) => {
                        lsize = self.build_next_layer(frm);
                    },
                    None => break
                }
            }
            lsize
        }

        fn self_intersect(newbrick: Brick, node: Rc<Node<Brick>>) -> bool {
            for rr in TreeIterator::new(Rc::clone(&node)) {
                if rr.value.coordinates == newbrick.coordinates {
                    return true;
                }
            }
            false
        }

        fn build_next_layer(&mut self, frm: Form) -> usize {
            let mut new_layer: Vec<Rc<Node<Brick>>> = Vec::new();

            // iterate on last_layer
            for nr in &self.last_layer {
                match frm {
                    Form::Straight => {
                        let new_brick = (*nr).value.next_straight();
                        if self.area.is_in(new_brick.coordinates) && !Path::self_intersect(new_brick, Rc::clone(nr)) {
                            new_layer.push(Rc::new(
                                    Node::<Brick> {
                                        father: Some(Rc::clone(nr)),
                                        value: new_brick
                                    }));
                        }
                    },
                    Form::Turn => {
                        for new_brick in &(*nr).value.next_turn() {
                            if self.area.is_in(new_brick.coordinates) && !Path::self_intersect(*new_brick, Rc::clone(nr)) {
                                new_layer.push(Rc::new(
                                        Node::<Brick> {
                                            father: Some(Rc::clone(nr)),
                                            value: *new_brick
                                        }));
                            }
                        }
                    }
                }
            }
            swap(&mut self.last_layer, &mut new_layer);

            println!("==================");
            self.print_layer();

            self.last_layer.len()
        }

        pub fn print_layer(&self) {
            for nr in &self.last_layer {
                println!("{:?}", (*nr).value);
            }
        }

        pub fn print_solution(&self) {
            for nr in &self.last_layer {
                println!("++++++++++++++++++++++++++++++++++++++++++++");
                for rr in TreeIterator::new(Rc::clone(&nr)) {
                    println!("{:?}", rr.value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::path::*;

    #[test]
    fn test_path() {
        // build area
        let mut area = Area::new();
        area.conditions.push(|pos| { pos[0] >= 0 && pos[1] >= 0 && pos[2] >= 0 });
        area.conditions.push(|pos| { pos[0] < 3  && pos[1] < 2  && pos[2] < 2 });

        // build snake
        let mut chain = Chain::new();
        chain.add(Form::Straight);
        chain.add(Form::Straight);
        chain.add(Form::Turn);
        chain.add(Form::Turn);
        chain.add(Form::Straight);
        chain.add(Form::Turn);
        chain.add(Form::Turn);
        chain.add(Form::Straight);
        chain.add(Form::Turn);
        chain.add(Form::Turn);
        chain.add(Form::Straight);
        chain.add(Form::Straight);
        assert_eq!(12, chain.len());

        // init Path
        let mut path = Path::new(area, chain);
        // add first brick
        let root_brick = Brick::new([0,0,0], Orientation::North, Form::Straight);
        path.add_brick(root_brick, None);
        println!("==================");
        path.print_layer();
        path.fold();
        //assert_eq!(2, path.fold(4));
        println!("==================");
        path.print_solution();
    }
}
