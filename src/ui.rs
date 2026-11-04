use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;
use crate::search;

// will use https://ratatui.rs/ for studying stuff for this project 

pub fn draw(frame:&mut ratatui::Frame<'_>, scroll: u16){
let block = Block::default()
    .title("docsy")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::Cyan));

    // ok so this is just to print out some random text
let some_random_text = (1..=60)
    .map(|i| format!("line {i}: stuff from documentation"))
    .collect::<Vec<_>>()
    .join("\n");

    let the_stuff = Paragraph::new(some_random_text)
        .left_aligned()
        .scroll((scroll, 0));

    frame.render_widget(the_stuff, frame.area());
}
