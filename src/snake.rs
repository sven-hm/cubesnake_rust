pub mod chain {
    use std::vec::Vec;

    #[derive(Copy, Clone, Debug)]
    pub enum Form {
        Straight,
        Turn
    }

    pub struct Chain {
        pub dirs: Vec<Form>
    }

    impl Chain {
        pub fn new() -> Chain {
            Chain { dirs: Vec::new() }
        }

        pub fn copy(&self) -> Chain {
            Chain { dirs: self.dirs.to_vec() }
        }

        pub fn add(&mut self, form: Form) {
            self.dirs.push(form);
        }

        pub fn len(&self) -> usize {
            self.dirs.len()
        }

        pub fn get(&self, index: usize) -> Option<Form> {
            if index < self.dirs.len() {
                Some(self.dirs[index].clone())
            } else {
                None
            }
        }
    }
}

pub mod brick {
    use super::chain::Form;

    pub type Position = [i8; 3];

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Orientation {
        North,
        South,
        East,
        West,
        Up,
        Down
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Brick {
        pub coordinates: Position,
        pub form: Form,
        pub orientation: Orientation,
    }

    impl Brick {
        pub fn new(crd: Position, orient: Orientation, frm: Form) -> Brick {
            Brick {
                orientation: orient,
                coordinates: crd,
                form: frm
            }
        }

        pub fn next_straight(&self) -> Brick {
            let mut coord = self.coordinates.clone();
            match self.orientation {
                Orientation::North => coord[0] += 1,
                Orientation::South => coord[0] -= 1,
                Orientation::East  => coord[1] += 1,
                Orientation::West  => coord[1] -= 1,
                Orientation::Up    => coord[2] += 1,
                Orientation::Down  => coord[2] -= 1
            }
            Brick {
                orientation: self.orientation.clone(),
                coordinates: coord,
                form: Form::Straight
            }
        }

        pub fn next_turn_orientation(&self, ori: &Orientation) -> Brick {
            let mut coord = self.coordinates.clone();
            match self.orientation {
                Orientation::North => coord[0] += 1,
                Orientation::South => coord[0] -= 1,
                Orientation::East  => coord[1] += 1,
                Orientation::West  => coord[1] -= 1,
                Orientation::Up    => coord[2] += 1,
                Orientation::Down  => coord[2] -= 1
            }
            Brick { orientation: ori.clone(), coordinates: coord, form: Form::Turn }
        }

        pub fn next_turn(&self) -> [Brick; 4] {
            // FIXME: use next_turn_orientation in here!
            let mut coord = self.coordinates.clone();
            match self.orientation {
                Orientation::North => coord[0] += 1,
                Orientation::South => coord[0] -= 1,
                Orientation::East  => coord[1] += 1,
                Orientation::West  => coord[1] -= 1,
                Orientation::Up    => coord[2] += 1,
                Orientation::Down  => coord[2] -= 1
            }

            let (or0, or1, or2, or3) = match self.orientation {
                Orientation::North | Orientation::South => {
                    (
                        Orientation::East,
                        Orientation::West,
                        Orientation::Up,
                        Orientation::Down,
                    )
                },
                Orientation::East | Orientation::West => {
                    (
                        Orientation::North,
                        Orientation::South,
                        Orientation::Up,
                        Orientation::Down,
                    )
                },
                Orientation::Up | Orientation::Down => {
                    (
                        Orientation::North,
                        Orientation::South,
                        Orientation::East,
                        Orientation::West,
                    )
                },
            };

            [
                Brick { orientation: or0, coordinates: coord, form: Form::Turn },
                Brick { orientation: or1, coordinates: coord, form: Form::Turn },
                Brick { orientation: or2, coordinates: coord, form: Form::Turn },
                Brick { orientation: or3, coordinates: coord, form: Form::Turn }
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::brick::*;
    use super::chain::*;

    #[test]
    fn test_chain() {
        let mut cc = Chain::new();
        cc.add(Form::Straight);
        cc.add(Form::Straight);
        cc.add(Form::Turn);
        assert_eq!(3, cc.len());

    }

    #[test]
    fn test_brick() {
        let _brk = Brick::new([0,0,0], Orientation::North, Form::Straight);
        let _brk = Brick::new([0,0,0], Orientation::South, Form::Straight);
        let _brk = Brick::new([0,0,0], Orientation::East, Form::Straight);
        let _brk = Brick::new([0,0,0], Orientation::West, Form::Straight);
        let _brk = Brick::new([0,0,0], Orientation::Up, Form::Straight);
        let brk = Brick::new([0,0,0], Orientation::Down, Form::Straight);
        assert_eq!(0, brk.coordinates[0]);
    }
}
