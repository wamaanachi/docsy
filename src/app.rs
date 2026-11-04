use crossterm::event::KeyCode;

pub struct App {
    pub scroll: u16,
}

impl App {
    pub fn new() -> Self {
        Self {scroll: 0}
    }

    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
        KeyCode::Up | KeyCode::Char('k') => self.scroll = self.scroll.saturating_sub(1),
        KeyCode::Down | KeyCode::Char('j') =>  self.scroll = self.scroll.saturating_add(1),
        KeyCode::Char('q') => return true,
            _ => {}
        }
        false
    }
}