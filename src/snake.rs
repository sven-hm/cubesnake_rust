pub mod chain {
    use std::vec::Vec;

    #[derive(Copy, Clone, Debug)]
    pub enum Form {
        Straight,
        Turn,
    }

    pub struct Chain {
        pub dirs: Vec<Form>,
    }

    impl Chain {
        pub fn new() -> Chain {
            Chain { dirs: Vec::new() }
        }

        pub fn copy(&self) -> Chain {
            Chain {
                dirs: self.dirs.to_vec(),
            }
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
    use std::ops;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Position {
        pub x: i8,
        pub y: i8,
        pub z: i8,
    }

    impl ops::Add<Orientation> for Position {
        type Output = Position;

        fn add(self, rhs: Orientation) -> Position {
            match rhs {
                Orientation::North => Position::new(self.x + 1, self.y, self.z),
                Orientation::South => Position::new(self.x - 1, self.y, self.z),
                Orientation::East => Position::new(self.x, self.y + 1, self.z),
                Orientation::West => Position::new(self.x, self.y - 1, self.z),
                Orientation::Up => Position::new(self.x, self.y, self.z + 1),
                Orientation::Down => Position::new(self.x, self.y, self.z - 1),
            }
        }
    }

    impl Position {
        pub fn new(x: i8, y: i8, z: i8) -> Position {
            Position { x: x, y: y, z: z }
        }

        pub fn neighbours(self) -> [Position; 6] {
            [
                self + Orientation::North,
                self + Orientation::South,
                self + Orientation::East,
                self + Orientation::West,
                self + Orientation::Up,
                self + Orientation::Down,
            ]
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Orientation {
        North,
        South,
        East,
        West,
        Up,
        Down,
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
                form: frm,
            }
        }

        pub fn next_straight(&self) -> Brick {
            Brick {
                orientation: self.orientation.clone(),
                coordinates: self.coordinates.clone() + self.orientation,
                form: Form::Straight,
            }
        }

        pub fn next_turn_orientation(&self, ori: &Orientation) -> Brick {
            Brick {
                orientation: ori.clone(),
                coordinates: self.coordinates.clone() + self.orientation,
                form: Form::Turn,
            }
        }

        pub fn next_turn(&self) -> [Brick; 4] {
            // FIXME: use next_turn_orientation in here!
            let coord = self.coordinates.clone() + self.orientation;

            let (or0, or1, or2, or3) = match self.orientation {
                Orientation::North | Orientation::South => (
                    Orientation::East,
                    Orientation::West,
                    Orientation::Up,
                    Orientation::Down,
                ),
                Orientation::East | Orientation::West => (
                    Orientation::North,
                    Orientation::South,
                    Orientation::Up,
                    Orientation::Down,
                ),
                Orientation::Up | Orientation::Down => (
                    Orientation::North,
                    Orientation::South,
                    Orientation::East,
                    Orientation::West,
                ),
            };

            [
                Brick {
                    orientation: or0,
                    coordinates: coord,
                    form: Form::Turn,
                },
                Brick {
                    orientation: or1,
                    coordinates: coord,
                    form: Form::Turn,
                },
                Brick {
                    orientation: or2,
                    coordinates: coord,
                    form: Form::Turn,
                },
                Brick {
                    orientation: or3,
                    coordinates: coord,
                    form: Form::Turn,
                },
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
        let _brk = Brick::new(Position::new(0, 0, 0), Orientation::North, Form::Straight);
        let _brk = Brick::new(Position::new(0, 0, 0), Orientation::South, Form::Straight);
        let _brk = Brick::new(Position::new(0, 0, 0), Orientation::East, Form::Straight);
        let _brk = Brick::new(Position::new(0, 0, 0), Orientation::West, Form::Straight);
        let _brk = Brick::new(Position::new(0, 0, 0), Orientation::Up, Form::Straight);
        let brk = Brick::new(Position::new(0, 0, 0), Orientation::Down, Form::Straight);
        assert_eq!(0, brk.coordinates.x);
    }
}
