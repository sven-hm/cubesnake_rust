mod area;
mod parser;
mod path;
mod snake;
mod tree;

use crate::parser::parser::Parser;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let contents: String =
            fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

        // read file
        let mut prsr = Parser::new(&contents);

        prsr.path.fold(true);

        File::create(&args[2])?.write_all(prsr.output().as_bytes())?;
    //println!("{}", prsr.output());
    } else {
        println!("usage: cubesnake <in file> <out file>");
    }
    Ok(())
}
