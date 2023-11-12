use std::io::stderr;

use anyhow::Result;
use crossterm::event::{self, poll, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

use crate::app_state::AppState;
use crate::ui::render_ui;
use crate::State;

pub fn initialize() -> Result<()> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;
    Ok(())
}

pub fn quit_app() -> Result<()> {
    execute!(stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// returns result with bool True to quit false to keep running
fn handle_events(state: &mut State, app_state: &mut AppState) -> Result<bool> {
    if poll(std::time::Duration::from_millis(250))? {
        let event = crossterm::event::read()?;
        match event {
            event::Event::Key(k) => {
                // keyboard events
                if k.kind == event::KeyEventKind::Press {
                    // if app is in idle mode then app_state can be changed
                    if let AppState::Idle = app_state {
                        if k.code == KeyCode::Char(':') {
                            app_state.enter_command_mode();
                        } else if k.code == KeyCode::Char('q') {
                            return Ok(true);
                        } else if k.code == KeyCode::Char('e') {
                            app_state.enter_edit_mode(state.file.clone());
                        }
                    // app is not in idle mode
                    } else if k.code == KeyCode::Esc {
                        app_state.enter_idle_mode();
                    }
                }
            }
            _ => {}
        }
    }
    Ok(false)
}

pub fn run_event_loop(
    mut state: State,
    mut terminal: Terminal<CrosstermBackend<std::io::Stderr>>,
    mut app_state: AppState,
) -> Result<()> {
    loop {
        // event management
        if handle_events(&mut state, &mut app_state)? {
            break;
        }
        // user interface
        render_ui(&mut terminal, &mut app_state)?;
    }
    Ok(())
}
