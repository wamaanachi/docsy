use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph, Wrap};

use crate::app::App;
use crate::search;
use crate::split;

fn draw_pager(frame: &mut Frame<'_>, app: &mut App) {
    let rows = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(frame.area());

    // 1. Extract what we need and drop all borrows of `app` before moving on
    let (text, title) = {
        let hits = app.hits();
        if hits.is_empty() {
            let empty_msg = Paragraph::new("\n  No documentation loaded.\n  Press '/' to search, or verify local target/doc directory.")
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::bordered().title(" rustyman "));
            frame.render_widget(empty_msg, rows[0]);
            return;
        }

        let selected_idx = app.selected.min(hits.len().saturating_sub(1));
        let doc = hits[selected_idx];

        let mut text = format!(
            "NAME\n    {} ({:?})\n\nSYNOPSIS\n    {}\n\nDESCRIPTION\n{}\n",
            doc.path,
            doc.kind,
            doc.declaration,
            crate::docs::text(&doc.description)
        );

        if !doc.methods.is_empty() {
            text.push_str("\nMETHODS\n");
            for method in &doc.methods {
                text.push_str(&format!(
                    "  • {}\n{}\n\n",
                    method.signature,
                    crate::docs::text(&method.description)
                ));
            }
        }

        let title = format!(
            " {} ({:?}) [{}/{}] ",
            doc.name,
            doc.kind,
            selected_idx + 1,
            hits.len()
        );

        (text, title)
    };

    // 2. Safe to mutate `app.max_scroll` now
    let width = rows[0].width.max(1) as usize;
    let total_lines: usize = text.lines().map(|line| (line.len() / width) + 1).sum();

    let viewport_height = rows[0].height.saturating_sub(2) as usize;
    app.max_scroll = total_lines.saturating_sub(viewport_height) as u16;
    app.scroll = app.scroll.min(app.max_scroll);

    let page = Paragraph::new(text)
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0))
        .block(
            Block::bordered()
                .title(title)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    frame.render_widget(page, rows[0]);

    let footer = Paragraph::new(
        " j/k: scroll  g/G: top/bottom  n/p: next/prev doc  s: split  /: search  q: quit ",
    )
    .style(Style::default().fg(Color::Black).bg(Color::Cyan));
    frame.render_widget(footer, rows[1]);
}

pub fn draw(frame: &mut Frame<'_>, app: &mut App) {
    if app.split_view {
        split::draw_split(frame, app);
    } else {
        draw_pager(frame, app);
    }

    search::draw_search(frame, app);
}
