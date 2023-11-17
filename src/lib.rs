pub mod args;
pub mod cursor;
pub mod editor_mode;
pub mod editor_state;
pub mod tui;
pub mod ui;

#[cfg(test)]
mod tests {

    use crate::{cursor::Cursor, editor_state::State};

    #[test]
    fn cursor_down_alt1() {
        let cursor = Cursor { location: 10 };
        let mut state = State {
            content: String::from("1234567\n12345\n123456789\n"),
            cursor,
            running: true,
            file: None,
            stacked_command: None,
        };

        assert_eq!(state.cursor.location, 10);
        state.move_cursor_down();
        assert_eq!(state.cursor.location, 16);
    }
    #[test]
    fn cursor_down_alt2() {
        let cursor = Cursor { location: 10 };
        let mut state = State {
            content: String::from("1234567\n1234567\n123456789\n"),
            cursor,
            running: true,
            file: None,
            stacked_command: None,
        };

        assert_eq!(state.cursor.location, 10);
        state.move_cursor_down();
        assert_eq!(state.cursor.location, 18);
    }
    #[test]
    fn cursor_down_alt3() {
        let cursor = Cursor { location: 12 };
        let mut state = State {
            content: String::from("123456789\n1234567\n123456789\n"),
            cursor,
            running: true,
            file: None,
            stacked_command: None,
        };

        assert_eq!(state.cursor.location, 12);
        state.move_cursor_down();
        assert_eq!(state.cursor.location, 20);
    }

    #[test]
    fn delete_line() {
        let cursor = Cursor { location: 12 };
        let mut state = State {
            content: String::from("12345678\n12345678\n12345678"),
            cursor,
            running: true,
            file: None,
            stacked_command: None,
        };
        state.delete_line();
        assert_eq!(state.content, String::from("12345678\n12345678"));
    }
}
