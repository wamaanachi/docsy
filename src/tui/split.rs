use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, List, Paragraph, ListState, Wrap};
use ratatui::Frame;

use crate::tui::app::{App, Focus};
use crate::parser::models::DocItem;

// this file was so wierd, editied this like a milliojn times

fn border_style(focused: bool) -> Style {
    if focused {
        Style::new().fg(Color::Cyan)
    } else {
        Style::new().fg(Color::DarkGray)
    }
}

pub fn draw_split(frame: &mut Frame<'_>, app: &mut App) {
    let rows = Layout::vertical([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let areas = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(70),
    ])
        .split(rows[0]);

    // removed the hardcoded 3 things so i can wire up chish's parser
    // should work now

    let query = app.input.to_lowercase();

    let docs: Vec<&DocItem> = app.docs
        .iter()
        .filter(|doc| doc.name.to_lowercase().contains(query.as_str()))
        .collect();
    let selected = app.selected.min(docs.len().saturating_sub(1));

    let doc = docs.get(selected);
    let doc_title = doc
        .map(|doc| format!(" {} ", doc.path))
        .unwrap_or_else(|| " docs ".to_string());


    let results = List::new(docs.iter().map(|doc| doc.name.as_str()))
        .block(
            Block::bordered()
                .title(format!(" Index ({})", docs.len()))
                .border_style(border_style(app.focus == Focus::Results)),
        )
        .highlight_style(Style::new().bg(Color::Cyan).fg(Color::Black));

    let mut state = ListState::default().with_selected(Some(selected));


    frame.render_stateful_widget(results, areas[0], &mut state);

    // this is the thing i had in the pager too, i could i have imported and stuff ig
    //but that'd make things unnecesarily complicated

    let text = doc
        .map(|doc| {
            let mut text = format!(
                "{}\n\n{}",
                doc.declaration,
                crate::parser::docs::text(&doc.description)
            );
            for method in &doc.methods {
                text.push_str(&format!(
                    "\n\n{}\n{}",
                    method.signature,
                    crate::parser::docs::text(&method.description)
                ));
            }
            text
        })
        .unwrap_or_default();

    let mut document = Paragraph::new(text)
        .block(
            Block::bordered()
                .title(doc_title)
                .border_style(border_style(app.focus == Focus::Document)), )
        .wrap(Wrap { trim: false });

    app.max_scroll = document.line_count(areas[1].width.saturating_sub(2)).saturating_sub(areas[1].height as usize) as u16;
    app.scroll = app.scroll.min(app.max_scroll);

    document = document.scroll((app.scroll, 0));


    frame.render_widget(document, areas[1]);

    let footer = Paragraph::new(" Tab switch  / filter  j/k scroll  g/G top/bottom  s view  q quit");
    frame.render_widget(footer, rows[1]);
}
