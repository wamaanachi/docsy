// yoooo just found this crate
// lessss goooo
// makes the work kinda easier
// and the docs help a bit for the code 

use ratatui::style::{Color, Style, Stylize};
use ratatui::Frame;
use tui_popup::Popup;

use crate::app::App;

pub fn draw_search(frame: &mut Frame<'_>, app: &App){
    if !app.searching {
        return;
    }

    let popup = Popup::new(format!("{}_", app.input))
        .title("Search");

    frame.render_widget(popup, frame.area());
}
