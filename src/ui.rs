use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;
use crate::search;
use crate::split;

// will use https://ratatui.rs/ for studying stuff for this project 

pub fn draw(frame: &mut Frame<'_>, app: &App){
    if app.split_view {
        split::draw_split(frame, app);
    }
    else {
        draw_pager(frame, app);
    }

    search::draw_search(frame, app);
}

fn draw_pager(frame: &mut ratatui::Frame<'_>, app: &App){

    // this the same stuff i did with the split layout
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
