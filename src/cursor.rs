pub struct Cursor {
    pub location: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { location: 0 }
    }
    pub fn move_char(&mut self) {
        self.location += 1;
    }
    pub fn back_char(&mut self) {
        if self.location != 0 {
            self.location -= 1;
        }
    }
    pub fn move_ahead(&mut self, dist: usize) {
        self.location += dist;
    }
    pub fn move_behind(&mut self, dist: usize) {
        if self.location >= dist {
            self.location -= dist;
        }
    }
}
