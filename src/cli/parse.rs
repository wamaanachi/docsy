use std::arch::x86_64::__get_cpuid_max;
use std::process::Command;
use clap::Parser;
use crate::cli::{CliArgs, SubCommand};

pub(crate) fn parse() {
    let args = CliArgs::parse();

    let command = args.command;

    match command {
        SubCommand::Get { all, item, name } => {
            if all {
                todo!(
                    "Print all instances of item from doc"
                )
            }

        }
        SubCommand::List => {
            todo!(
                "Wire this to print every instance of every thing in the doc"
            )
        }
        SubCommand::Search { query } => {
            todo!(
                "Parse this query or do smth else, change or remove this wwhole thing"
            )
        }
        SubCommand::Generate { dir } => {
            generate_docs(dir)
        }
    }
}


fn generate_docs(dir: Option<String>) {
    if dir.is_some() {
        Command::new("cargo")
            .arg("doc")
            .current_dir(dir.unwrap())
            .status()
            .expect("Something went wrong");
    } else {
        Command::new("cargo")
            .arg("doc")
            .status()
            .expect("Something went wrong");
    }
}
