use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, List, Paragraph, ListState};
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

    let rows = Layout::vertical([Constraint::Min(0), Constraint::Length(1)])
    .split(frame.area());

    let areas = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(70),
    ])
    .split(rows[0]);

    let query = app.input.to_lowercase();

    // turned into a vector cause it is easy like this
    // and this is so that the search thing actually works
    // and umm used some docs for this code
    let items: Vec<&str> = ["SearchResults", "DocItem", "ItemKind"]
        .into_iter()
        .filter(|item| item.to_lowercase().contains(query.as_str()))
        .collect();
    let selected = app.selected.min(items.len().saturating_sub(1));

    let results = List::new(items)
        .block(
            Block::bordered()
                .title("Results")
                .border_style(border_style(app.focus == Focus::Results)),
        )
        .highlight_style(Style::new().bg(Color::Cyan).fg(Color::Black));

    let mut state = ListState::default().with_selected(Some(selected));
    

    frame.render_stateful_widget(results, areas[0], &mut state);

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
            .border_style(border_style(app.focus == Focus::Document)),)
            .scroll((app.scroll, 0));

    
    frame.render_widget(document, areas[1]);

    let footer = Paragraph::new(" Tab switch  / filter  j/k scroll  s view  q quit");
    frame.render_widget(footer, rows[1]);
}

