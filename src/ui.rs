use ratatui::widgets::Paragraph;
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
    // ok so this is just to print out some random text
let some_random_text = (1..=60)
    .map(|i| format!("line {i}: stuff from documentation"))
    .collect::<Vec<_>>()
    .join("\n");

    let the_stuff = Paragraph::new(some_random_text)
        .left_aligned()
        .scroll((app.scroll, 0));

    frame.render_widget(the_stuff, frame.area());

}

