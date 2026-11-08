use std::process::{exit, Command, ExitStatus};
use clap::{Parser};
use crate::cli::{CliArgs, SubCommand};


pub(crate) fn parse() {
    let args = CliArgs::parse();

    // Parse flags first
    parse_flags(args.clone());


    // Parse commands next
    let command = args.command;
    if command.is_some() {
        parse_subcommand(command.unwrap());
    }
}


fn parse_subcommand(command: SubCommand) {
    match command {
        SubCommand::Get { all, item, name, package } => {
            if all {
                todo!(
                    "Print all instances of item from doc [Args received: All = {all}, ItemKind = {:?}, crate = {:?}", item, package
                )
            } else {
                todo!(
                    "Print all instances of item from doc [Args received: All = {all}, ItemKind = {:?}, name = {}, crate = {}",
                    item,
                    name.unwrap().to_lowercase(),
                    package.unwrap().to_lowercase()
                )
            }
        }
        SubCommand::List => {
            todo!(
                "Wire this to print every instance of every thing in the doc"
            )

            // Like this:
            //
            // Struct:
            //   - SearchResult
            //   - DocItem
            // Enum:
            //   - ItemKind
        }
        SubCommand::Search { query } => {
            todo!(
                "Parse this query or do smth else, change or remove this whole thing.\n Query: {query}"
            )
        }
        SubCommand::Generate { path: dir } => {
            generate_docs(dir)
        }
    }
}


fn parse_flags(args: CliArgs) {
    if args.path.is_some() {
        todo!(
            "To path tweakery with this"
        )
    }
}


fn generate_docs(dir: Option<String>) {
    let status: ExitStatus;
    if dir.is_some() {
        status = Command::new("cargo")
            .arg("doc")
            .current_dir(dir.unwrap())
            .status()
            .expect("Something went wrong");
    } else {
        status = Command::new("cargo")
            .arg("doc")
            .status()
            .expect("Something went wrong");
    }
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}
