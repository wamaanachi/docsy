use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, List, ListState, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::search;
