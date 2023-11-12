use std::io::stderr;

use anyhow::Result;
use clap::Parser;
use mini_vim::{
    app_state::AppState,
    args::CustomArgs,
    tui::{initialize, quit_app, run_event_loop},
    State,
};
use ratatui::{prelude::CrosstermBackend, terminal};

fn main() -> Result<()> {
    let args = CustomArgs::parse();
    let state = State {
        file: args.file.clone(),
    };
    let app_state = AppState::new(args);
    let terminal = terminal::Terminal::new(CrosstermBackend::new(stderr()))?;

    initialize()?;

    run_event_loop(state, terminal, app_state)?;

    quit_app()?;

    Ok(())
}
