use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color, Style, Stylize};
use ratatui::Frame;

// will use https://ratatui.rs/ for studying stuff for this project 

pub fn draw(frame:&mut ratatui::Frame<'_>, scroll: u16){
let block = Block::default()
    .title("docsy")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::Cyan));

let the_stuff = Paragraph::new("the stuff from the documentations")
    .block(block)
    .left_aligned()
    .scroll((scroll, 0));

    frame.render_widget(the_stuff, frame.area());
}