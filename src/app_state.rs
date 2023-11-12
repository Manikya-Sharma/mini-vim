use std::path::PathBuf;

use crate::args::CustomArgs;

pub enum AppState {
    Command,
    Edit(Option<PathBuf>),
    Idle,
}

impl AppState {
    pub fn new(args: CustomArgs) -> Self {
        if let Some(file) = args.file {
            Self::Edit(Some(file))
        } else {
            Self::Idle
        }
    }

    pub fn enter_command_mode(&mut self) {
        *self = Self::Command;
    }

    pub fn enter_idle_mode(&mut self) {
        *self = Self::Idle;
    }

    pub fn enter_edit_mode(&mut self, file: Option<PathBuf>) {
        *self = Self::Edit(file);
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
            Self::Command => String::from("Command mode"),
            Self::Edit(_) => String::from("Edit"),
            Self::Idle => String::from("Idle"),
        }
    }
}
