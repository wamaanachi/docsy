use clap::Parser;
use crate::cli::parse::parse;
use crate::cli::CliArgs;

pub mod cli;
pub mod parser;
pub mod check;
pub mod tui;

use tui::app::App;
use crossterm::event::{self, Event};

fn main() -> std::io::Result<()> {
    if CliArgs::parse().is_none() {
        draw_app()?
    } else {
        parse()?
    }

    Ok(())
}


fn draw_app() -> std::io::Result<()> {
    // Default `cargo run`: launch the TUI!
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|frame| tui::ui::draw(frame, &mut app))?;

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
