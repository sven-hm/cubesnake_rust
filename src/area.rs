pub mod area {
    use std::vec::Vec;
    pub use crate::snake::brick::{Brick, Position};

    pub type Condition = fn(Position) -> bool;

    pub struct Area {
        pub conditions: Vec<Condition>
    }

    impl Area {
        pub fn new() -> Area {
            Area { conditions: Vec::new() }
        }

        pub fn is_in(&self, pos: Position) -> bool {
            // test all conditions
            for cond in &self.conditions {
                if !cond(pos) { return false; }
            }
            return true;
        }

        // TODO: add split condition
    }
}

#[cfg(test)]
mod tests {
    use super::area::*;

    #[test]
    fn test_area() {
        let mut area = Area::new();
        area.conditions.push(|pos| { pos[0] > 0 });
        area.conditions.push(|pos| { pos[0] < 3 });

        let a: Position = [1,2,3];
        let b: Position = [-1,2,3];
        let c: Position = [3,2,3];
        assert!(area.is_in(a));
        assert!(!area.is_in(b));
        assert!(!area.is_in(c));
    }
}