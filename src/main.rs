use gpui::{
  App, AppContext, Application, Bounds, Context, Entity, ParentElement, Render, Styled,
  WindowBounds, WindowOptions, div, px, size,
};

mod cursor;
mod input;
mod text;

use crate::input::TerminalInput;

struct Zeta {
  term_input: Entity<TerminalInput>,
}

impl Zeta {
  pub fn new(cx: &mut Context<Self>) -> Self {
    Self {
      term_input: cx.new(TerminalInput::new),
    }
  }
}

impl Render for Zeta {
  fn render(
    &mut self,
    _window: &mut gpui::Window,
    _cx: &mut gpui::Context<Self>,
  ) -> impl gpui::IntoElement {
    div().size_full().child(self.term_input.clone())
  }
}

fn main() {
  Application::new().run(|cx: &mut App| {
    let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
    cx.open_window(
      WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        ..Default::default()
      },
      |_, cx| cx.new(Zeta::new),
    )
    .unwrap();
  });
}
