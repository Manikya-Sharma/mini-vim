use std::io::stderr;

use anyhow::Result;
use clap::Parser;
use mini_vim::{
    args::CustomArgs,
    editor_mode::EditorMode,
    editor_state::State,
    tui::{initialize, quit_app, run_event_loop},
};
use ratatui::{prelude::CrosstermBackend, terminal};

fn main() -> Result<()> {
    let args = CustomArgs::parse();
    let state = State::begin_from_file(args.file.clone());
    let editor_mode = EditorMode::new(args);
    let terminal = terminal::Terminal::new(CrosstermBackend::new(stderr()))?;

    initialize()?;

    run_event_loop(state, terminal, editor_mode)?;

    quit_app()?;

    Ok(())
}
