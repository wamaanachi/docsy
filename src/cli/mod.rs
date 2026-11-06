pub mod parse;

use clap::{Parser, Subcommand};
use crate::models::ItemKind;

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
    /// Outputs docs for whatever you entered
    Get {
        /// Gives every instance of this item type
        #[arg(long)]
        all: bool,
        #[arg(value_enum, required = true)]
        item: ItemKind,
        #[arg(required_unless_present = "all")]
        name: Option<String>,
    },

    /// Lists all things in the program
    List,

    /// Search for a specific thing
    Search {
        query: String,
    },

    /// Runs cargo doc in the current directory
    Generate {
        /// Path from terminal dir, or use full path starting from root
        /// path = "/home/user/Projects/..." or "C:\..."
        path: Option<String>,
    }
}
