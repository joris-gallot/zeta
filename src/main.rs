use gpui::{
  App, AppContext, Application, Bounds, KeyBinding, WindowBounds, WindowOptions, px, size,
};

mod text_element;
mod text_input;
mod zeta;
use crate::{text_input::Quit, zeta::Zeta};

fn main() {
  Application::new().run(|cx: &mut App| {
    let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);

    Zeta::register(cx);

    let window = cx
      .open_window(
        WindowOptions {
          window_bounds: Some(WindowBounds::Windowed(bounds)),
          ..Default::default()
        },
        |_, cx| cx.new(Zeta::new),
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
