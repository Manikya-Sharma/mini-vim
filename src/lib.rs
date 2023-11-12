use std::path::PathBuf;

pub mod app_state;
pub mod args;
pub mod editor_state;
pub mod tui;
pub mod ui;

pub struct State {
    pub file: Option<PathBuf>,
}
