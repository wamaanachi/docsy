use crossterm::event::KeyCode;

pub struct App {
    pub scroll: u16,
    pub searching: bool,
    pub input: String,
    pub split_view: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            scroll: 0,
            searching: false,
            input: String::new(),
            split_view: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        // umm so these are for the search box thingy
        if self.searching {
            match key {
                KeyCode::Esc | KeyCode::Enter => self.searching = false,
                KeyCode::Backspace => {self.input.pop();},
                KeyCode::Char(c) => self.input.push(c),
                _ => {}
            }
            return false;
        }

        match key {
            KeyCode::Char('/') => {
                self.searching = true;
                self.input.clear();
            }
        KeyCode::Up | KeyCode::Char('k') => self.scroll = self.scroll.saturating_sub(1),
        KeyCode::Down | KeyCode::Char('j') =>  self.scroll = self.scroll.saturating_add(1),
        // so this is gonna be temporary i think
        KeyCode::Char('s') => self.split_view = !self.split_view,
        KeyCode::Char('q') => return true,
            _ => {}
        }
        false
    }
}

