use crate::text::TextBuffer;
use gpui::{
  Context, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent, ParentElement, Render,
  Styled, Window, div, white,
};

pub struct TerminalInput {
  text_buffer: TextBuffer,
  focus_handle: FocusHandle,
}

impl TerminalInput {
  pub fn new(cx: &mut Context<Self>) -> Self {
    Self {
      focus_handle: cx.focus_handle(),
      text_buffer: TextBuffer::new(),
    }
  }

  fn on_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
    let alt = event.keystroke.modifiers.alt;

    match event.keystroke.key.as_str() {
      "left" => {
        if alt {
          self.text_buffer.move_left_word();
        } else {
          self.text_buffer.move_left_char();
        }

        cx.notify();
      }
      "right" => {
        if alt {
          self.text_buffer.move_right_word();
        } else {
          self.text_buffer.move_right_char();
        }

        cx.notify();
      }
      "space" => {
        self.text_buffer.insert_char(' ');
        cx.notify();
      }
      "backspace" => {
        self.text_buffer.delete_char();
        cx.notify();
      }
      key => {
        if let Some(ch) = key.chars().next() {
          self.text_buffer.insert_char(ch);
          cx.notify();
        }
      }
    }
  }

  fn render_content(&self) -> impl IntoElement {
    let cursor_col = self.text_buffer.get_cursor_index();

    let input_content = self.text_buffer.as_str();
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
