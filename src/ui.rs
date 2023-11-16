use anyhow::Result;
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::{editor_mode::EditorMode, editor_state::State};

fn layout_layer(frame: &Frame) -> std::rc::Rc<[ratatui::prelude::Rect]> {
    Layout::default()
        .direction(ratatui::prelude::Direction::Vertical)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Min(1),
        ])
        .split(frame.size())
}

pub fn render_ui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stderr>>,
    editor_state: &mut EditorMode,
    state: &State,
) -> Result<()> {
    terminal.draw(|frame| {
        let layout = layout_layer(frame);

        let title = Paragraph::new(editor_state.get_file_name().unwrap_or("No file open"))
            .block(Block::default().borders(Borders::BOTTOM))
            .alignment(ratatui::prelude::Alignment::Center)
            .bg(Color::LightRed)
            .fg(Color::Magenta);

        let mut my_str = state.content.clone();
        my_str.insert(state.cursor.location, '|');
        let main_content = Paragraph::new(my_str);
        let footer = Paragraph::new(editor_state.display_mode()).block(
            Block::default()
                .borders(Borders::TOP)
                .bg(match editor_state {
                    EditorMode::Command(_) => Color::Blue,
                    EditorMode::Edit(_) => Color::LightGreen,
                    EditorMode::Idle(_) => Color::Cyan,
                })
                .fg(match editor_state {
                    EditorMode::Command(_) => Color::White,
                    EditorMode::Edit(_) => Color::Green,
                    EditorMode::Idle(_) => Color::Blue,
                }),
        );
        frame.render_widget(title, layout[0]);
        frame.render_widget(main_content, layout[1]);
        frame.render_widget(footer, layout[2]);
    })?;
    Ok(())
}
