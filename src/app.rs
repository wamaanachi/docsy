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
            KeyCode::Up = if self.scroll = self.scroll.saturating_sub(1),
            KeyCode::Down =  if self.scroll = self.scroll.saturating_add(1),
            KeyCode::Esc = true,
            _ => {}
        }
        false
    }
}

