use std::io::stderr;

use anyhow::Result;
use crossterm::event::{self, poll, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

use crate::editor_mode::EditorMode;
use crate::editor_state::State;
use crate::ui::render_ui;

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
fn handle_events(state: &mut State, editor_mode: &mut EditorMode) -> Result<()> {
    if poll(std::time::Duration::from_millis(250))? {
        let event = crossterm::event::read()?;
        if let event::Event::Key(k) = event {
            // keyboard events
            if k.kind == event::KeyEventKind::Press {
                // if app is in idle mode then editor_state can be changed
                if let EditorMode::Idle(_) = editor_mode {
                    if k.code == KeyCode::Char(':') {
                        editor_mode.enter_command_mode();
                    } else if k.code == KeyCode::Char('i') {
                        editor_mode.enter_edit_mode(state.file.clone());
                    }
                    // navigation in idle mode
                    else if k.code == KeyCode::Char('l') {
                        state.move_cursor_ahead();
                    } else if k.code == KeyCode::Char('h') {
                        state.move_cursor_behind();
                    } else if k.code == KeyCode::Char('k') {
                        state.move_cursor_up();
                    } else if k.code == KeyCode::Char('j') {
                        state.move_cursor_down();
                    } else if k.code == KeyCode::Char('w') {
                        state.move_by_a_word();
                    }
                // app is not in idle mode
                } else if k.code == KeyCode::Esc {
                    editor_mode.enter_idle_mode(None);
                // Some input in Edit or Command mode
                } else {
                    // command mode
                    if let EditorMode::Command(_) = editor_mode {
                        match k.code {
                            KeyCode::Char(value) => {
                                editor_mode.update_command(value);
                            }
                            KeyCode::Backspace => {
                                editor_mode.remove_from_command();
                            }
                            KeyCode::Enter => match editor_mode.apply_command(state) {
                                Ok(message) => editor_mode.enter_idle_mode(Some(message)),
                                Err(m) => editor_mode.enter_idle_mode(Some(m.to_string())),
                            },
                            _ => {}
                        }
                    // editor mode
                    } else {
                        match k.code {
                            KeyCode::Char(value) => {
                                state.update_edit(value);
                            }
                            KeyCode::Backspace => {
                                state.remove_from_edit();
                            }
                            KeyCode::Enter => state.add_newline_edit(),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn run_event_loop(
    mut state: State,
    mut terminal: Terminal<CrosstermBackend<std::io::Stderr>>,
    mut editor_state: EditorMode,
) -> Result<()> {
    loop {
        if !state.running {
            break;
        }
        // event management
        handle_events(&mut state, &mut editor_state)?;

        // user interface
        render_ui(&mut terminal, &mut editor_state, &state)?;
    }
    Ok(())
}
