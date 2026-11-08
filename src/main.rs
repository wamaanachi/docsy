use clap::Parser;
use crate::cli::parse::parse;
use crossterm::event::KeyEventKind;
use crate::cli::CliArgs;

mod ui;
mod app;
mod search;
mod split;

pub mod cli;
pub mod models;
pub mod parser;
mod check;


fn main() -> std::io::Result<()> {
    if CliArgs::parse().is_none() {
        draw_app()?
    } else {
        parse()?
    }

    Ok(())
}


fn draw_app() -> std::io::Result<()> {
    let mut app = app::App::new();

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| ui::draw(frame, &app))?;

            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }
                if app.handle_key(key.code) {
                    break Ok::<(), std::io::Error>(());
                }
            }
        }
    }).expect("picnicing");

    Ok(())
}
