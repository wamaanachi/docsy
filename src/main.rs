use clap::Parser;
use crate::cli::parse::parse;
use crossterm::event::KeyEventKind;
use crate::cli::CliArgs;

mod ui;
mod app;
mod cli;
mod docs;
mod models;
mod parser;
mod search;
mod split;
mod ui;

pub mod cli;
pub mod models;
pub mod parser;
mod check;

use app::App;
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::io;

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
