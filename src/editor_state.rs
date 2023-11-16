use anyhow::Result;
use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

use crate::cursor::Cursor;

pub struct State {
    pub running: bool,
    pub file: Option<PathBuf>,
    pub content: String,
    pub cursor: Cursor,
}

impl State {
    pub fn begin_from_file(file: Option<PathBuf>) -> Self {
        Self {
            running: true,
            file,
            content: String::new(),
            cursor: Cursor::new(),
        }
    }
    pub fn update_edit(&mut self, ch: char) {
        self.content.insert(self.cursor.location, ch);
        self.cursor.move_char();
    }

    pub fn add_newline_edit(&mut self) {
        self.content.insert(self.cursor.location, '\n');
        self.cursor.move_char();
    }

    pub fn remove_from_edit(&mut self) {
        if self.cursor.location == 0 {
            return;
        }
        self.content.remove(self.cursor.location - 1);
        self.cursor.back_char();
    }

    pub fn move_cursor_ahead(&mut self) {
        if self.cursor.location >= self.content.len() {
            return;
        }
        self.cursor.move_char();
    }

    pub fn move_cursor_behind(&mut self) {
        self.cursor.back_char();
    }

    pub fn move_cursor_up(&mut self) {
        if let Some(slice) = self.content.get(..self.cursor.location) {
            if let Some(loc) = slice.rfind('\n') {
                let first_chars = self.cursor.location - loc;
                if let Some(second_slice) = self.content.get(..loc) {
                    if let Some(last_newline) = second_slice.rfind('\n') {
                        if first_chars > loc - last_newline {
                            self.cursor.move_behind(first_chars);
                        } else {
                            self.cursor
                                .move_behind(self.cursor.location - last_newline - first_chars);
                        }
                    } else if first_chars > loc {
                        self.cursor.move_behind(first_chars);
                    } else {
                        self.cursor
                            .move_behind(self.cursor.location - first_chars + 1);
                    }
                }
            }
        }
    }

    pub fn move_cursor_down(&mut self) {
        if let Some(prev_slice) = self.content.get(..self.cursor.location) {
            let loc1 = prev_slice.rfind('\n').unwrap_or(0);
            let first_chars = self.cursor.location - loc1;
            if let Some(after_slice) = self.content.get(self.cursor.location..) {
                if let Some(after_chars) = after_slice.find('\n') {
                    // check if next line has less number of characters
                    if let Some(next_newline) = after_slice.get((after_chars + 1)..) {
                        let next_newline_at = next_newline.find('\n').unwrap_or(self.content.len());
                        if next_newline_at < first_chars {
                            self.cursor.move_ahead(after_chars + next_newline_at + 1);
                        } else if self.cursor.location + after_chars + first_chars
                            < self.content.len()
                        {
                            self.cursor.move_ahead(after_chars + first_chars);
                        } else {
                            self.move_to_end();
                        }
                    }
                }
            }
        }
    }

    pub fn move_by_a_word(&mut self) {
        if self.cursor.location == self.content.len() {
            return;
        }
        if let Some(slice) = self.content.get(self.cursor.location..) {
            let loc1 = slice.find(' ');
            let loc2 = slice.find('\n');
            if let Some(l1) = loc1 {
                if let Some(l2) = loc2 {
                    if l1 < l2 {
                        if l1 == 0 {
                            self.cursor.move_char();
                        } else {
                            self.cursor.move_ahead(l1);
                        }
                    } else if l2 == 0 {
                        self.cursor.move_char();
                    } else {
                        self.cursor.move_ahead(l2);
                    }
                } else {
                    self.cursor.move_ahead(l1);
                }
            } else if let Some(l) = loc2 {
                if l == 0 {
                    self.cursor.move_char();
                } else {
                    self.cursor.move_ahead(l);
                }
            } else {
                self.cursor.move_ahead(slice.len() - 1);
            }
        }
    }

    pub fn move_to_end(&mut self) {
        self.cursor.location = self.content.len();
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
