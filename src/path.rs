pub mod path {
    use std::vec::Vec;
    use std::mem::swap;
    pub use std::rc::Rc;
    pub use crate::snake::chain::{Chain, Form};
    pub use crate::snake::brick::{Brick, Orientation};
    pub use crate::area::area::{Area, Position};
    use crate::tree::{Node, TreeIterator};

    pub struct Path {
        pub area: Area,
        pub chain: Chain,

        last_layer: Vec<Rc<Node<Brick>>>,
        last_layer_index: usize,

        pub statistics: Vec<(usize, usize)>
    }

    impl Path {
        pub fn new(area: Area, chain: Chain) -> Path {
            Path {
                area: area,
                chain: chain,
                last_layer: Vec::new(),
                last_layer_index: 0,
                statistics: Vec::new(),
            }
        }

        pub fn add_brick(&mut self, brick: &Brick) {
            if self.last_layer.len() == 0 {
                self.last_layer.push(Rc::new(Node::<Brick> {
                    father: None,
                    value: *brick
                }));
            } else {
                self.last_layer[0] = Rc::new(Node::<Brick> {
                    father: Some(Rc::clone(&self.last_layer[0])),
                    value: *brick
                });
            }

            self.statistics.push((self.last_layer_index, 1));
            self.last_layer_index += 1;
        }

        pub fn fold(&mut self) -> usize {
            let mut lsize = 0;
            for ii in self.last_layer_index..self.chain.len() {
                match self.chain.get(ii) {
                    Some(frm) => {
                        lsize = self.build_next_layer(frm);
                        self.statistics.push((ii, lsize));
                    },
                    None => {
                        self.statistics.push((ii, 0));
                        break;
                    }
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
                        if self.area.is_in(new_brick.coordinates)
                                && !Path::self_intersect(new_brick, Rc::clone(nr)) {
                            new_layer.push(Rc::new(
                                    Node::<Brick> {
                                        father: Some(Rc::clone(nr)),
                                        value: new_brick
                                    }));
                        }
                    },
                    Form::Turn => {
                        for new_brick in &(*nr).value.next_turn() {
                            if self.area.is_in(new_brick.coordinates)
                                    && !Path::self_intersect(*new_brick, Rc::clone(nr)) {
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

            //println!("==================");
            //self.print_layer();

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

        pub fn solution_string_long(&self) -> String {
            let mut output = "coords   form orientation\n".to_string();
            for nr in &self.last_layer {
                for rr in TreeIterator::new(Rc::clone(&nr)) {
                    let mut line = format!("{:?}  ", rr.value.coordinates);
                    line.push_str(match rr.value.form {
                        Form::Straight => "S",
                        Form::Turn => "T"
                    });
                    line.push_str("  ");
                    line.push_str(match rr.value.orientation {
                        Orientation::North => "N",
                        Orientation::South => "S",
                        Orientation::East  => "E",
                        Orientation::West  => "W",
                        Orientation::Up    => "U",
                        Orientation::Down  => "D",
                    });
                    line.push_str("\n");
                    output.push_str(&line);
                }
            }
            output
        }

        pub fn solution_string_short(&self) -> String {
            let mut output = "orientation_switches\n".to_string();
            for nr in &self.last_layer {
                let mut last_orientation: Option<Orientation> = None;
                for rr in TreeIterator::new(Rc::clone(&nr)) {

                    if last_orientation.is_none() ||
                            last_orientation.unwrap() != rr.value.orientation {
                        output.push_str(match rr.value.orientation {
                            Orientation::North => "N",
                            Orientation::South => "S",
                            Orientation::East  => "E",
                            Orientation::West  => "W",
                            Orientation::Up    => "U",
                            Orientation::Down  => "D",
                        });
                        output.push_str("\n")
                    }
                    last_orientation = Some(rr.value.orientation);
                }
            }
            output
        }

        pub fn solution_string_statistics(&self) -> String {
            let mut output = "statistics\n".to_string();
            output.push_str("step number_paths\n");
            for tt in &self.statistics {
                output.push_str(&format!("{}\t{}\n", tt.0 + 1, tt.1));
            }
            output
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
        path.add_brick(&root_brick);
        println!("==================");
        //path.print_layer();
        path.fold();
        //assert_eq!(2, path.fold(4));
        println!("==================");
        path.print_solution();
    }

    #[test]
    fn test_cubesnake_small() {
        // build area
        let mut area = Area::new();
        area.conditions.push(|pos| { pos[0] >= 0 && pos[1] >= 0 && pos[2] >= 0 });
        area.conditions.push(|pos| { pos[0] < 3  && pos[1] < 3  && pos[2] < 3 });

        // build snake
        let mut chain = Chain::new();
        chain.add(Form::Straight);
        chain.add(Form::Straight);
        chain.add(Form::Turn);
        chain.add(Form::Straight);
        chain.add(Form::Turn);
        chain.add(Form::Straight);
        chain.add(Form::Turn);
        chain.add(Form::Straight);
        chain.add(Form::Turn);
        chain.add(Form::Turn);
        chain.add(Form::Turn);
        chain.add(Form::Turn);
        chain.add(Form::Straight);
        chain.add(Form::Turn);
        chain.add(Form::Straight);
        chain.add(Form::Turn);
        chain.add(Form::Turn);
        chain.add(Form::Turn);
        chain.add(Form::Straight);
        chain.add(Form::Turn);
        chain.add(Form::Turn);
        chain.add(Form::Straight);
        chain.add(Form::Turn);
        chain.add(Form::Turn);
        chain.add(Form::Turn);
        chain.add(Form::Straight);
        chain.add(Form::Straight);
        assert_eq!(27, chain.len());

        // init Path
        let mut path = Path::new(area, chain);

        // add first bricks
        path.add_brick(&Brick::new([2,2,2], Orientation::South, Form::Straight));
        path.add_brick(&Brick::new([1,2,2], Orientation::South, Form::Straight));
        path.add_brick(&Brick::new([0,2,2], Orientation::Down, Form::Turn));

        println!("==================");
        path.print_layer();
        assert_eq!(1, path.fold());
        println!("==================");
        path.print_solution();
        println!("==================");
        println!("statistics");
        for tt in path.statistics {
            println!("{}\t{}", tt.0, tt.1)
        }
    }
}
