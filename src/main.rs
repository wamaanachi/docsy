mod app;
mod cli;
mod docs;
mod models;
mod parser;
mod search;
mod split;
mod ui;

use app::App;
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // If someone explicitly runs subcommands (Amaan's area), route to CLI
    if args.len() > 1 {
        cli::parse::parse();
        return Ok(());
    }

    // Default `cargo run`: launch the TUI!
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                if app.handle_key(key.code) {
                    break;
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}
