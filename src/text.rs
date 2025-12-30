use ropey::Rope;

use crate::cursor::Cursor;
use unicode_segmentation::UnicodeSegmentation;

pub struct TextBuffer {
  text: Rope,
  cursor: Cursor,
}

impl TextBuffer {
  pub fn new() -> Self {
    Self {
      text: Rope::new(),
      cursor: Cursor::new(),
    }
  }

  pub fn move_right_word(&mut self) {
    let content = self.text.to_string();
    let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(content.as_str(), true).collect();
    let mut index = self.cursor.index;

    while index < graphemes.len() && graphemes[index].trim().is_empty() {
      index += 1;
    }

    while index < graphemes.len() && !graphemes[index].trim().is_empty() {
      index += 1;
    }

    self.cursor.index = index.min(graphemes.len());
  }

  pub fn move_left_word(&mut self) {
    let content = self.text.to_string();
    let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(content.as_str(), true).collect();
    let mut index = self.cursor.index;

    if index == 0 {
      return;
    }

    index -= 1;

    while index > 0 && graphemes[index].trim().is_empty() {
      index -= 1;
    }

    while index > 0 && !graphemes[index].trim().is_empty() {
      index -= 1;
    }

    if graphemes[index].trim().is_empty() && index < self.cursor.index {
      index += 1;
    }

    self.cursor.index = index;
  }

  pub fn move_right_char(&mut self) {
    self.cursor.move_right(self.text.len_chars());
  }

  pub fn move_left_char(&mut self) {
    self.cursor.move_left();
  }

  pub fn delete(&mut self, index: usize, len: usize) {
    let end = (index + len).min(self.text.len_chars());
    self.text.remove(index..end);
  }

  pub fn delete_char(&mut self) {
    if self.cursor.index > 0 {
      self.cursor.move_left();
      self.delete(self.cursor.index, 1);
    }
  }

  pub fn insert_char(&mut self, ch: char) {
    self.text.insert_char(self.cursor.index, ch);
    self.cursor.move_right(self.text.len_chars());
  }

  pub fn get_cursor_index(&self) -> usize {
    self.cursor.index
  }

  pub fn as_str(&self) -> String {
    self.text.to_string()
  }

  pub fn len(&self) -> usize {
    self.text.len_chars()
  }
}
