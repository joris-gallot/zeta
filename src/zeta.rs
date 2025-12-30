use gpui::{
  App, Context, Entity, FocusHandle, Focusable, KeyBinding, Window, div, prelude::*, rgb,
};

use crate::text_input::{
  AltLeft, AltRight, Backspace, CmdLeft, CmdRight, Copy, Cut, Delete, End, Home, Left, Paste,
  Right, SelectAll, SelectEnd, SelectLeft, SelectRight, SelectStart, SelectWordLeft,
  SelectWordRight, ShowCharacterPalette, TextInput,
};

pub struct Zeta {
  pub text_input: Entity<TextInput>,
  pub focus_handle: FocusHandle,
}

impl Zeta {
  pub fn new(cx: &mut Context<Self>) -> Self {
    let text_input = cx.new(|cx| TextInput {
      focus_handle: cx.focus_handle(),
      content: "".into(),
      placeholder: "Type here...".into(),
      selected_range: 0..0,
      selection_reversed: false,
      marked_range: None,
      last_layout: None,
      last_bounds: None,
      is_selecting: false,
    });

    Self {
      text_input,
      focus_handle: cx.focus_handle(),
    }
  }

  pub fn register(cx: &mut App) {
    cx.bind_keys([
      KeyBinding::new("backspace", Backspace, None),
      KeyBinding::new("delete", Delete, None),
      KeyBinding::new("left", Left, None),
      KeyBinding::new("alt-left", AltLeft, None),
      KeyBinding::new("cmd-left", CmdLeft, None),
      KeyBinding::new("right", Right, None),
      KeyBinding::new("alt-right", AltRight, None),
      KeyBinding::new("cmd-right", CmdRight, None),
      KeyBinding::new("shift-left", SelectLeft, None),
      KeyBinding::new("alt-shift-left", SelectWordLeft, None),
      KeyBinding::new("cmd-shift-left", SelectStart, None),
      KeyBinding::new("shift-right", SelectRight, None),
      KeyBinding::new("alt-shift-right", SelectWordRight, None),
      KeyBinding::new("cmd-shift-right", SelectEnd, None),
      KeyBinding::new("cmd-a", SelectAll, None),
      KeyBinding::new("cmd-v", Paste, None),
      KeyBinding::new("cmd-c", Copy, None),
      KeyBinding::new("cmd-x", Cut, None),
      KeyBinding::new("home", Home, None),
      KeyBinding::new("end", End, None),
      KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, None),
    ]);
  }
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
