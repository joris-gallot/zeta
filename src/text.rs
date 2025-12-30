use ropey::Rope;

use crate::cursor::Cursor;

pub struct TextBuffer {
  text: Rope,
  pub cursor: Cursor,
}

impl TextBuffer {
  pub fn new() -> Self {
    Self {
      text: Rope::new(),
      cursor: Cursor::new(),
    }
  }

  pub fn insert_char(&mut self, ch: char) {
    self.text.insert_char(self.cursor.index, ch);
    self.cursor.move_right(self.text.len_chars());
  }

  pub fn as_str(&self) -> String {
    self.text.to_string()
  }

  pub fn len(&self) -> usize {
    self.text.len_chars()
  }
}
