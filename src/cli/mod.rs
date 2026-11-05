use clap::{Parser, Subcommand};

/// Args for the Command Line
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {
    /// Basic subcommands
    #[clap(subcommand)]
    command: SubCommand
}

/// Basic subcommands
#[derive(Subcommand, Debug)]
pub(crate) enum SubCommand {
    Html,
}
