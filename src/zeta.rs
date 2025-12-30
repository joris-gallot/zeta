use gpui::{App, Context, Entity, FocusHandle, Focusable, Window, div, prelude::*, rgb};

use crate::text_input::TextInput;

pub struct Zeta {
  pub text_input: Entity<TextInput>,
  pub focus_handle: FocusHandle,
}

impl Focusable for Zeta {
  fn focus_handle(&self, _: &App) -> FocusHandle {
    self.focus_handle.clone()
  }
}

impl Render for Zeta {
  fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    div()
      .bg(rgb(0xaaaaaa))
      .track_focus(&self.focus_handle(cx))
      .flex()
      .flex_col()
      .size_full()
      .child(self.text_input.clone())
  }
}
