mod ui;
mod app;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal|{
        loop {
        terminal.draw(ui::draw)?;

        if crossterm::event::read()?.is_key_press(){
         break Ok(());   
        }   
    }
})
}
