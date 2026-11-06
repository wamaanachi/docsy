use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, List, Paragraph};
use ratatui::Frame;

use crate::app::{App, Focus};

fn border_style(focused: bool) -> Style {
    if focused {
        Style::new().fg(Color::Cyan)
    }
    else {
        Style::new().fg(Color::DarkGray)
    }
}

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
    .block(
        Block::bordered()
        .title("Results")
        .title_bottom("Tab focus | s view")
        .border_style(border_style(app.focus == Focus::Results)),
    );

    frame.render_widget(results, areas[0]);

    // this is the thing i had in the pager too, i could i have imported and stuff ig
    //but that'd make things unnecesarily complicated

    let text = (1..=20)
        .map(|i| format!("line {i}: stuff from documentation"))
        .collect::<Vec<_>>()
        .join("\n");

    let document = Paragraph::new(text) 
    .block(
        Block::bordered()
            .title("docs")
            .title_bottom("j/k scroll | Tab focus | / search | s view | q quit")
            .border_style(border_style(app.focus == Focus::Document)),)
            .scroll((app.scroll, 0));

    frame.render_widget(document, areas[1]);

}