use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, List, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn draw_split(frame: &mut Frame<'_>, app: &App){
    let areas = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(70),
    ])
    .split(frame.area());

    let results = List::new([
        "SearchResult",
        "DocItem",
        "ItemKind",
    ])
    .block(Block::bordered().title("Results"));

    frame.render_widget(results, areas[0]);

    // this is the thing i had in the pager too, i could i have imported and stuff ig
    //but that'd make things unnecesarily complicated

    let text = (1..=20)
        .map(|i| format!("line {i}: stuff from documentation"))
        .collect::<Vec<_>>()
        .join("\n");

    let document = Paragraph::new(text) 
    .block(Block::bordered().title("docs"))
    .scroll((app.scroll, 0));

    frame.render_widget(document, areas[1]);

}