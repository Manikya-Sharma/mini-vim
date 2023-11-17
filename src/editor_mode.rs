use anyhow::Result;
use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

use crate::{args::CustomArgs, editor_state::State};

pub enum EditorMode {
    Command(String),
    Edit(Option<PathBuf>),
    Idle(Option<String>),
}

impl EditorMode {
    pub fn new(args: CustomArgs) -> Self {
        if args.file.is_some() {
            Self::Idle(Some(String::from("Opened a file")))
        } else {
            Self::Idle(Some(String::from("Opened untitiled file")))
        }
    }

    pub fn enter_command_mode(&mut self) {
        *self = Self::Command(String::new());
    }

    pub fn enter_idle_mode(&mut self, message: Option<String>) {
        *self = Self::Idle(message);
    }

    pub fn enter_edit_mode(&mut self, file: Option<PathBuf>) {
        *self = Self::Edit(file);
    }

    pub fn update_command(&mut self, ch: char) {
        if let Self::Command(c) = self {
            c.push(ch);
        }
    }

    pub fn remove_from_command(&mut self) {
        if let Self::Command(c) = self {
            c.pop();
        }
    }

    pub fn apply_command(&mut self, state: &mut State) -> Result<String> {
        if let EditorMode::Command(c) = self {
            if c == "q" || c == "quit" {
                state.end_program();
                Ok(String::from("exiting mini-vim"))
            } else if c == "w" || c == "write" {
                match state.flush_file() {
                    Ok(_) => Ok(String::from("File written successfully")),
                    Err(e) => Err(anyhow::Error::from(Error::new(
                        ErrorKind::Other,
                        e.to_string(),
                    ))),
                }
            } else if c == "wq" {
                match state.flush_file() {
                    Ok(_) => Ok(String::from("File written successfully")),
                    Err(e) => Err(anyhow::Error::from(Error::new(
                        ErrorKind::Other,
                        e.to_string(),
                    ))),
                }?;
                state.end_program();
                Ok(String::from("exiting mini-vim"))
            } else {
                Err(anyhow::Error::from(Error::new(
                    ErrorKind::InvalidData,
                    "no such command found",
                )))
            }
        } else {
            Err(anyhow::Error::from(Error::new(
                ErrorKind::InvalidData,
                "invalid location to evaluate a command",
            )))
        }
    }

    pub fn get_file_name(&self) -> Option<&str> {
        if let Self::Edit(Some(file)) = self {
            Some(file.file_name().expect("Invalid file").to_str().unwrap())
        } else {
            None
        }
    }
    pub fn display_mode(&self) -> String {
        match self {
            Self::Command(c) => {
                let mut display = String::from("=> ");
                display.push_str(c);
                display
            }
            Self::Edit(_) => String::from("Edit"),
            Self::Idle(message) => {
                if let Some(message) = message {
                    message.to_owned()
                } else {
                    String::from("Idle")
                }
            }
        }
    }
}
