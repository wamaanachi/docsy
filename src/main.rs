use crate::cli::parse::parse;

mod ui;
mod app;
mod search;
pub mod cli;
pub mod models;

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