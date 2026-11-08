use crate::cli::parse::parse;
use crossterm::event::KeyEventKind;
use check::CrateCheck;

mod ui;
mod app;
mod search;
mod split;

pub mod cli;
pub mod models;
pub mod parser;
mod check;


fn main() -> std::io::Result<()> {
    let package_name: String;

    match CrateCheck::new().directory() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    parse();

    let mut app = app::App::new();

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| ui::draw(frame, &app))?;

            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }
                if app.handle_key(key.code) {
                    break Ok(());
                }
            }
        }
    })
}
