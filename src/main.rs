use std::process::Command;
use crate::cli::parse::parse;

pub mod cli;
pub mod models;

fn main() {
    parse();

    println!("Hello, world!");
}
