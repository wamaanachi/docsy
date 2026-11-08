use clap::{ Parser };
use crate::cli::{ commands, CliArgs, SubCommand };
use crate::cli::commands::find_all;

pub(crate) fn parse() -> std::io::Result<()> {
    let args:CliArgs = CliArgs::parse();

    // Parse commands no need to parse flags just yet
    let command = args.command;
    
    if command.is_some() {
        parse_subcommand(command.unwrap()).expect("picnicking");
    }

    Ok(())
}


fn parse_subcommand(command: SubCommand) -> Result<(), Box<dyn std::error::Error>> {
    let _package: String;

    match command {
        SubCommand::Get { all, item, package, name } => {
            if package.is_none() {
                _package = crate::check::CrateCheck::new().check_crate_name().unwrap();
            } else {
                _package = package.unwrap();
            }

            if all {
                commands::show_all_instances(item, _package).expect("picnic");
                Ok(())
            } else {
                let true_name = name.unwrap();

                let thingies: Vec<(String, String)> = find_all(_package.clone());

                let item_href = thingies
                    .into_iter()
                    .find(|(id, _)| id == &true_name)
                    .map(|(_, href)| href)
                    .ok_or_else(|| {
                        eprintln!("Not found: {}", true_name);
                        std::io::Error::new(std::io::ErrorKind::NotFound, "Name incorrect")
                    })?;

                commands::show_one(_package, item_href)
            }
        }
        SubCommand::List => {
            _package = crate::check::CrateCheck::new().check_crate_name().unwrap();

            commands::classify(commands::find_all(_package)).expect("picnic");
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
