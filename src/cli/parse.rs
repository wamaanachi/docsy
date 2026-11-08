use clap::{Parser};
use crate::cli::{commands, CliArgs, SubCommand};

pub(crate) fn parse() -> std::io::Result<()> {
    let args:CliArgs = CliArgs::parse();

    // Parse commands no need to parse flags just yet
    let command = args.command;
    if command.is_some() {
        parse_subcommand(command.unwrap()).expect("picnicking");
    }

    Ok(())
}


fn parse_subcommand(command: SubCommand) -> std::io::Result<()> {
    let _package: String;

    match command {
        SubCommand::Get { all, item, name, package } => {
            if package.is_none() {
                _package = crate::check::CrateCheck::new().check_crate_name().unwrap();
            } else {
                _package = package.unwrap();
            }

            if all {
                commands::show_all_instances(item, _package).expect("picnic");
                Ok(())
            } else {
                commands::classify(commands::find_all(_package)).expect("picnic");
                Ok(())
            }
        }
        SubCommand::List => {
            _package = crate::check::CrateCheck::new().check_crate_name().unwrap();

            let things: Vec<(String, String)> = commands::find_all(_package.clone());

            println!("{} Things found in {_package}", things.len());
            for (name, _href) in things {
                println!("- {}", name);
            };

            Ok(())
        }
        // SubCommand::Search { query } => {
        //     to do("Parse this query or do smth else, change or remove this whole thing.\n Query: {query}")
        // }
        SubCommand::Generate { path: dir } => {
            commands::generate_docs(dir);
            Ok(())
        }
    }
}
