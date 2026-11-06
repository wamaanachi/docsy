use crate::cli::parse::parse;

mod ui;
mod app;
mod search;
mod split;

pub mod cli;
pub mod models;
pub mod parser;

fn main() -> std::io::Result<()> {

        parse();

    let mut app = app::App::new();

    ratatui::run(|terminal|{
        loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if let crossterm::event::Event::Key(key) = crossterm::event::read()?{
            if app.handle_key(key.code){
         break Ok(());   
            }
        }   
    }
})
}