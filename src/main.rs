mod tree;
mod snake;
mod area;
mod path;
mod parser;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use crate::parser::parser::Parser;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let contents: String = fs::read_to_string(&args[1])
            .expect("Something went wrong reading the file");

        // read file 
        let mut prsr = Parser::new(&contents);

        prsr.path.fold();

        File::create(&args[2])?.write_all(prsr.output().as_bytes())?;
        //println!("{}", prsr.output());
    } else {
        println!("usage: cubesnake <in file> <out file>");
    }
    Ok(())
}
