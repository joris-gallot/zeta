use gpui::{
  App, AppContext, Application, Bounds, KeyBinding, WindowBounds, WindowOptions, px, size,
};

mod text_element;
mod text_input;
mod zeta;
use crate::{
  text_input::{
    AltLeft, AltRight, Backspace, CmdLeft, CmdRight, Copy, Cut, Delete, End, Home, Left, Paste,
    Quit, Right, SelectAll, SelectEnd, SelectLeft, SelectRight, SelectStart, SelectWordLeft,
    SelectWordRight, ShowCharacterPalette, TextInput,
  },
  zeta::Zeta,
};

fn main() {
  Application::new().run(|cx: &mut App| {
    let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);

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

    let window = cx
      .open_window(
        WindowOptions {
          window_bounds: Some(WindowBounds::Windowed(bounds)),
          ..Default::default()
        },
        |_, cx| {
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

          cx.new(|cx| Zeta {
            text_input,
            focus_handle: cx.focus_handle(),
          })
        },
      )
      .unwrap();

    window
      .update(cx, |view, window, cx| {
        window.focus(&view.text_input.as_mut(cx).focus_handle);
        cx.activate(true);
      })
      .unwrap();

    cx.on_action(|_: &Quit, cx| cx.quit());
    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
  });
}
