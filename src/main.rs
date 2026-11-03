mod ui;
mod app;

fn main() -> std::io::Result<()> {

    let mut app = app::App::new();

    ratatui::run(|terminal|{
        loop {
        terminal.draw(|frame| ui::draw(frame, app.scroll))?;

        if let crossterm::event::Event::Key(key) = crossterm::event::read()?{
            if app.handle_key(key.code){
         break Ok(());   
            }
        }   
    }
})
}
