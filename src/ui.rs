use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};

use crate::app::App;
use crate::search;
use crate::split;

// will use https://ratatui.rs/ for studying stuff for this project 

pub fn draw(frame: &mut Frame<'_>, app: &App){

    let Some(doc) = app.docs.get(app.selected) else {
        return;
    };

    let text = format!("{}\n\n{}\n\n{}", doc.path, doc.declaration);
    let rows = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(frame.area());

    frame.render_widget(
        Paragraph::new(text).wrap(Wrap {trim: true}).scroll((app.scroll, 0)),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new(" j/k scroll  s split  q quit"),
        rows[1],
    )

}

fn draw_pager(frame: &mut ratatui::Frame<'_>, app: &App){


let some_random_text = app
    .docs
    .get(app.selected)
    .map(|doc| format!("{}\n\n{}", doc.declaration, doc.description))
    .unwrap_or_default();

    let the_stuff = Paragraph::new(some_random_text)
        .left_aligned()
        .scroll((app.scroll, 0))
        .wrap(Wrap {trim: true});

    frame.render_widget(the_stuff, frame.area());

} 
