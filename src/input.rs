use gpui::{
  Context, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent, ParentElement, Render,
  Styled, Window, div, white,
};

use crate::text::TextBuffer;

pub struct TerminalInput {
  input_buffer: TextBuffer,
  focus_handle: FocusHandle,
}

impl TerminalInput {
  pub fn new(cx: &mut Context<Self>) -> Self {
    Self {
      focus_handle: cx.focus_handle(),
      input_buffer: TextBuffer::new(),
    }
  }

  fn on_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
    match event.keystroke.key.as_str() {
      "left" => {
        self.input_buffer.cursor.move_left();
        cx.notify();
      }
      "right" => {
        self.input_buffer.cursor.move_right(self.input_buffer.len());
        cx.notify();
      }
      key => {
        if let Some(ch) = key.chars().next() {
          self.input_buffer.insert_char(ch);
          cx.notify();
        }
      }
    }
  }

  fn render_content(&self) -> impl IntoElement {
    let cursor_col = self.input_buffer.cursor.index;

    let input_content = self.input_buffer.as_str();
    let (before_cursor, after_cursor) = input_content.split_at(cursor_col);
    let ren = format!("{}|{}", before_cursor, after_cursor);

    div().child(ren)
  }
}

impl Render for TerminalInput {
  fn render(
    &mut self,
    _window: &mut gpui::Window,
    cx: &mut gpui::Context<Self>,
  ) -> impl gpui::IntoElement {
    div()
      .track_focus(&self.focus_handle)
      .size_full()
      .on_key_down(cx.listener(Self::on_key_down))
      .text_color(white())
      .child(self.render_content())
  }
}
