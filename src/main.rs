use crate::cli::parse::parse;

pub mod cli;
pub mod models;
pub mod parser;

fn main() {
    parse();

    println!("Hello, world!");
}
