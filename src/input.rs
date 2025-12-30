use crate::text::TextBuffer;
use gpui::{
  Context, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent, ParentElement, Render,
  Styled, Window, div, white,
};

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
    let alt = event.keystroke.modifiers.alt;

    match event.keystroke.key.as_str() {
      "left" => {
        if alt {
          self.input_buffer.move_left_word();
        } else {
          self.input_buffer.move_left_char();
        }

        cx.notify();
      }
      "right" => {
        if alt {
          self.input_buffer.move_right_word();
        } else {
          self.input_buffer.move_right_char();
        }

        cx.notify();
      }
      "space" => {
        self.input_buffer.insert_char(' ');
        cx.notify();
      }
      "backspace" => {
        self.input_buffer.delete_char();
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
    let cursor_col = self.input_buffer.get_cursor_index();

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
