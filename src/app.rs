use crossterm::event::KeyCode;
use crate::models::DocItem;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Results,
    Document,
}

pub struct App {
    pub scroll: u16,
    pub max_scroll: u16,
    pub searching: bool,
    pub input: String,
    pub split_view: bool,
    pub focus: Focus,
    pub selected: usize,
    pub docs: Vec<DocItem>,
}

impl App {
    pub fn new() -> Self {
        Self {
            scroll: 0,
            max_scroll: 0,
            searching: false,
            input: String::new(),
            split_view: false,
            focus: Focus::Results,
            selected: 0,
            docs: crate::docs::load(std::path::Path::new("target/doc/manr")),
        }
    }

    pub fn hits(&self) -> Vec<&DocItem> {
        let query = self.input.to_lowercase();

        self.docs
            .iter()
            .filter(|doc| doc.name.to_lowercase().contains(&query))
            .collect()
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

        let len = self.hits().len();

        match key {
            KeyCode::Char('/') => {
                self.searching = true;
                self.input.clear();
            }
        KeyCode::Up | KeyCode::Char('k') => {
            if self.split_view && self.focus == Focus::Results {
                self.selected = self.selected.saturating_sub(1);
                self.scroll = 0;
            }
            else {
                self.scroll = self.scroll.saturating_sub(1);
            }
        }
        
            KeyCode::Down | KeyCode::Char('j') => {
                if self.split_view && self.focus == Focus::Results {
                    self.selected = (self.selected + 1).min(len.saturating_sub(1));
                    self.scroll = 0;
                }
                else {
                    self.scroll = self.scroll.saturating_add(1);
                }
            }
            //  this is to go to next/previous topic's documentations
        KeyCode::Char('n') => {
            self.selected = (self.selected + 1).min(len.saturating_sub(1));
            self.scroll = 0;
        }

        KeyCode::Char('p') => {
            self.selected = self.selected.saturating_sub(1);
            self.scroll  = 0;
        }
            // so this is to like go to the end and go to the top
            // using g and G because thats how rusty-man do it /i think
        KeyCode::Home | KeyCode::Char('g') => self.scroll = 0,
        KeyCode::End | KeyCode::Char('G') => self.scroll = self.max_scroll,

        KeyCode::Char('s') => self.split_view = !self.split_view,
        KeyCode::Tab => {
            self.focus = match self.focus {
                Focus::Results => Focus::Document,
                Focus::Document => Focus::Results,
            };
        }

        KeyCode::Enter => {
            if self.split_view && self.focus == Focus::Results {
                self.focus = Focus::Document;
            }
        }
        KeyCode::Char('q') => return true,
            _ => {}
        }
        false
    }
}
