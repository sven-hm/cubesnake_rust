pub mod parser {
    use std::result::Result::{Ok, Err};
    use std::vec::Vec;
    use crate::snake::chain::{Chain, Form};
    use crate::snake::brick::{Brick, Orientation};
    use crate::area::area::{Area, Position};
    use crate::path::path::{Path};

    fn in_cube3(pos: Position) -> bool {
        let dim = 3;
        pos[0] >= 0 && pos[1] >= 0 && pos[2] >= 0 &&
            pos[0] < dim  && pos[1] < dim  && pos[2] < dim
    }

    pub struct Parser {
        inputstring: String,
        pub path: Path
    }

    impl Parser {
        pub fn new(input: &String) -> Parser {
            let mut area = Area::new();
            let mut chain = Chain::new();
            let mut orientations: Vec<Orientation> = Vec::new();
            let mut startbrick: [i8; 3] = [0, 0, 0];

            for line in input.lines() {
                let lv: Vec<&str> = line.trim_start().split_whitespace().collect();

                match lv[0] {
                    "area" => {
                        // build area
                        // FIXME: only accept cube for now
                        if lv[1] != "cube" {
                            continue;
                        }
                        // FIXME: check dim
                        let _dim: i8 = lv[2].parse().expect("parse error");
                        area.conditions.push(in_cube3);
                    },
                    "chain" => {
                        // build chain
                        for ff in lv[2].chars() {
                            match ff {
                                'S' => chain.add(Form::Straight),
                                'T' => chain.add(Form::Turn),
                                _ => {}
                            }
                        }
                    },
                    "path" => {
                        // build paths
                        for ff in lv[1].chars() {
                            match ff {
                                'N' => orientations.push(Orientation::North),
                                'S' => orientations.push(Orientation::South),
                                'E' => orientations.push(Orientation::East),
                                'W' => orientations.push(Orientation::West),
                                'U' => orientations.push(Orientation::Up),
                                'D' => orientations.push(Orientation::Down),
                                _ => {}
                            }
                        }
                    },
                    "start" => {
                        startbrick = [
                            lv[1].parse().expect("parser error"),
                            lv[2].parse().expect("parser error"),
                            lv[3].parse().expect("parser error")
                        ];
                    },
                    _ => {
                        // ignore for now
                    }
                }
            }

            for line in input.lines() {
                // FIXME
                let lv: Vec<&str> = line.trim_start().split_whitespace().collect();
                match lv[0].parse::<i8>() {
                    Ok(_) => {},
                    Err(_) => continue
                }

                // check for numeric value
                // FIXME: read into array?
                let coord: Vec<i8> = line
                    .split_whitespace()
                    .map(|s| s.parse().expect("parse error"))
                    .collect();
                if coord.len() != 3 {
                    continue;
                }
                startbrick = [coord[0], coord[1], coord[2]];
            }

            // keep copy of chain to be able to iterate on forms later
            // ... probably  not the best way XXX
            let chain2 = chain.copy();
            let mut path = Path::new(area, chain);

            // build first bricks
            let mut ot = orientations.iter();
            // first must be straight -> check for it XXX!
            //let _fo = ot.next();

            let mut newbrick: Option<Brick> = None;

            for frm in chain2.dirs.iter() {
                newbrick = match newbrick {
                    None => {
                        Some(Brick::new(startbrick, ot.next().unwrap().clone(),
                            path.chain.get(0).unwrap()))
                    },
                    Some(nb) => {
                        match frm {
                            Form::Straight => {
                                Some(nb.next_straight())
                            },
                            Form::Turn => {
                                // XXX
                                match ot.next() {
                                    None => {
                                        break;
                                    },
                                    Some(ori) => {
                                        Some(nb.next_turn_orientation(ori))
                                    }
                                }
                            }
                        }
                    }
                };
                let brk = match newbrick {
                    None => break,
                    Some(brk) => brk
                };
                path.add_brick(&brk);
            }
            Parser { path: path, inputstring: input.to_string() }
        }

        pub fn output(&self) -> String {
            let mut outputstring = self.inputstring.to_string();
            outputstring.push_str(&"=========================\n".to_string());
            outputstring.push_str(&self.path.solution_string_long());
            outputstring.push_str(&"-------------------------\n".to_string());
            outputstring.push_str(&self.path.solution_string_short());
            outputstring.push_str(&"-------------------------\n".to_string());
            outputstring.push_str(&self.path.solution_string_statistics());
            outputstring
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parser::*;

    #[test]
    fn test_parser() {
        let input = "area cube 3
chain 27 SSTTTSTTSTTTSTSTTTTSTSTSTSS
path NE
start 0 0 0
".to_string();
        let mut parser = Parser::new(&input);
        // inspect...
        parser.path.print_solution();
        assert_eq!(1, parser.path.fold());
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        println!("{}", parser.output());
    }
}
