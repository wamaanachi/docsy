use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};

use crate::app::App;
use crate::search;
use crate::split;

// will use https://ratatui.rs/ for studying stuff for this project 

fn draw_pager(frame: &mut Frame<'_>, app: &App){

    let hits = app.hits();

    let Some(doc) = hits.get(app.selected.min(hits.len().saturating_sub(1))) 
    else {
        return;
    };

    let mut text = format!(
        "{:?} {}\n\n{}\n\n{}",
        doc.kind,
        doc.path,
        doc.declaration,
         crate::docs::text(&doc.description)
    );

    for method in &doc.methods {
        text.push_str(&format!(
            "\n\n{}\n{}",
            method.signature,
            crate::docs::text(&method.description)
        ));
    }

    let rows = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(frame.area());

    frame.render_widget(
        Paragraph::new(text).wrap(Wrap {trim: false}).scroll((app.scroll, 0)),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new(" j/k scroll  n/p next/previous  s split  q quit"),
        rows[1],
    );

}

pub fn draw(frame: &mut ratatui::Frame<'_>, app: &App){

    if app.split_view {
        split::draw_split(frame, app);        
    }
    else {
        draw_pager(frame, app);
    }

    search::draw_search(frame, app);
} 
