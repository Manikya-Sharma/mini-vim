use anyhow::Result;
use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

pub struct State {
    pub running: bool,
    pub file: Option<PathBuf>,
    pub content: String,
}

impl State {
    pub fn begin_from_file(file: Option<PathBuf>) -> Self {
        Self {
            running: true,
            file,
            content: String::new(),
        }
    }
    pub fn update_edit(&mut self, ch: char) {
        self.content.push(ch);
    }

    pub fn add_newline_edit(&mut self) {
        self.content.push('\n');
    }

    pub fn remove_from_edit(&mut self) {
        self.content.pop();
    }

    pub fn flush_file(&self) -> Result<()> {
        if let Some(file) = &self.file {
            std::fs::write(file, self.content.clone())?;
            Ok(())
        } else {
            Err(anyhow::Error::from(Error::new(
                ErrorKind::NotFound,
                "No file in buffer",
            )))
        }
    }
    pub fn end_program(&mut self) {
        self.running = false;
    }
}
