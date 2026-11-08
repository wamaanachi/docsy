use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, List, Paragraph, ListState, Wrap};
use ratatui::Frame;

use crate::app::{App, Focus};
use crate::models::DocItem;

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

   // removed the hardcoded 3 things so i can wire up chish's parser
   // should work now

   let query = app.input.to_lowercase();

   let docs: Vec<&DocItem>  = app.docs
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
        .map(|doc| format!("{}\n\n{}", doc.declaration, crate::docs::text(&doc.description)))
        .unwrap_or_default();

    let document = Paragraph::new(text) 
    .block(
        Block::bordered()
            .title(doc_title)
            .border_style(border_style(app.focus == Focus::Document)),)
            .scroll((app.scroll, 0))
            .wrap(Wrap {trim: true});

    
    frame.render_widget(document, areas[1]);

    let footer = Paragraph::new(" Tab switch  / filter  j/k scroll  s view  q quit");
    frame.render_widget(footer, rows[1]);
}
