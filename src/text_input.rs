use std::ops::Range;

use gpui::{
  App, Bounds, ClipboardItem, Context, CursorStyle, EntityInputHandler, FocusHandle, Focusable,
  MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, ShapedLine,
  SharedString, UTF16Selection, Window, actions, div, point, prelude::*, px, rgb, white,
};
use unicode_segmentation::*;

use crate::text_element::TextElement;

actions!(
  text_input,
  [
    Backspace,
    Delete,
    Left,
    AltLeft,
    CmdLeft,
    Right,
    AltRight,
    CmdRight,
    SelectLeft,
    SelectWordLeft,
    SelectStart,
    SelectRight,
    SelectWordRight,
    SelectEnd,
    SelectAll,
    Home,
    End,
    ShowCharacterPalette,
    Paste,
    Cut,
    Copy,
    Quit,
  ]
);

pub struct TextInput {
  pub focus_handle: FocusHandle,
  pub content: SharedString,
  pub placeholder: SharedString,
  pub selected_range: Range<usize>,
  pub selection_reversed: bool,
  pub marked_range: Option<Range<usize>>,
  pub last_layout: Option<ShapedLine>,
  pub last_bounds: Option<Bounds<Pixels>>,
  pub is_selecting: bool,
}

impl TextInput {
  fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
    if self.selected_range.is_empty() {
      self.move_to(self.previous_boundary(self.cursor_offset()), cx);
    } else {
      self.move_to(self.selected_range.start, cx)
    }
  }

  fn alt_left(&mut self, _: &AltLeft, _: &mut Window, cx: &mut Context<Self>) {
    self.move_to(self.previous_word_boundary(self.selected_range.start), cx);
  }

  fn cmd_left(&mut self, _: &CmdLeft, _: &mut Window, cx: &mut Context<Self>) {
    self.move_to(0, cx);
  }

  fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
    if self.selected_range.is_empty() {
      self.move_to(self.next_boundary(self.selected_range.end), cx);
    } else {
      self.move_to(self.selected_range.end, cx)
    }
  }

  fn alt_right(&mut self, _: &AltRight, _: &mut Window, cx: &mut Context<Self>) {
    self.move_to(self.next_word_boundary(self.selected_range.end), cx);
  }

  fn cmd_right(&mut self, _: &CmdRight, _: &mut Window, cx: &mut Context<Self>) {
    self.move_to(self.content.len(), cx);
  }

  fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
    self.select_to(self.previous_boundary(self.cursor_offset()), cx);
  }

  fn select_word_left(&mut self, _: &SelectWordLeft, _: &mut Window, cx: &mut Context<Self>) {
    self.select_to(self.previous_word_boundary(self.cursor_offset()), cx);
  }

  fn select_start(&mut self, _: &SelectStart, _: &mut Window, cx: &mut Context<Self>) {
    self.select_to(0, cx);
  }

  fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
    self.select_to(self.next_boundary(self.cursor_offset()), cx);
  }

  fn select_word_right(&mut self, _: &SelectWordRight, _: &mut Window, cx: &mut Context<Self>) {
    self.select_to(self.next_word_boundary(self.cursor_offset()), cx);
  }

  fn select_end(&mut self, _: &SelectEnd, _: &mut Window, cx: &mut Context<Self>) {
    self.select_to(self.content.len(), cx);
  }

  fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
    self.move_to(0, cx);
    self.select_to(self.content.len(), cx)
  }

  fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
    self.move_to(0, cx);
  }

  fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
    self.move_to(self.content.len(), cx);
  }

  fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
    if self.selected_range.is_empty() {
      self.select_to(self.previous_boundary(self.cursor_offset()), cx)
    }
    self.replace_text_in_range(None, "", window, cx)
  }

  fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
    if self.selected_range.is_empty() {
      self.select_to(self.next_boundary(self.cursor_offset()), cx)
    }
    self.replace_text_in_range(None, "", window, cx)
  }

  fn on_mouse_down(
    &mut self,
    event: &MouseDownEvent,
    _window: &mut Window,
    cx: &mut Context<Self>,
  ) {
    self.is_selecting = true;

    if event.modifiers.shift {
      self.select_to(self.index_for_mouse_position(event.position), cx);
    } else {
      self.move_to(self.index_for_mouse_position(event.position), cx)
    }
  }

  fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
    self.is_selecting = false;
  }

  fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
    if self.is_selecting {
      self.select_to(self.index_for_mouse_position(event.position), cx);
    }
  }

  fn show_character_palette(
    &mut self,
    _: &ShowCharacterPalette,
    window: &mut Window,
    _: &mut Context<Self>,
  ) {
    window.show_character_palette();
  }

  fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
    if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
      self.replace_text_in_range(None, &text.replace("\n", " "), window, cx);
    }
  }

  fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
    if !self.selected_range.is_empty() {
      cx.write_to_clipboard(ClipboardItem::new_string(
        self.content[self.selected_range.clone()].to_string(),
      ));
    }
  }
  fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
    if !self.selected_range.is_empty() {
      cx.write_to_clipboard(ClipboardItem::new_string(
        self.content[self.selected_range.clone()].to_string(),
      ));
      self.replace_text_in_range(None, "", window, cx)
    }
  }

  fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
    self.selected_range = offset..offset;
    cx.notify()
  }

  pub fn cursor_offset(&self) -> usize {
    if self.selection_reversed {
      self.selected_range.start
    } else {
      self.selected_range.end
    }
  }

  fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
    if self.content.is_empty() {
      return 0;
    }

    let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref()) else {
      return 0;
    };
    if position.y < bounds.top() {
      return 0;
    }
    if position.y > bounds.bottom() {
      return self.content.len();
    }
    line.closest_index_for_x(position.x - bounds.left())
  }

  fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
    if self.selection_reversed {
      self.selected_range.start = offset
    } else {
      self.selected_range.end = offset
    };
    if self.selected_range.end < self.selected_range.start {
      self.selection_reversed = !self.selection_reversed;
      self.selected_range = self.selected_range.end..self.selected_range.start;
    }
    cx.notify()
  }

  fn offset_from_utf16(&self, offset: usize) -> usize {
    let mut utf8_offset = 0;
    let mut utf16_count = 0;

    for ch in self.content.chars() {
      if utf16_count >= offset {
        break;
      }
      utf16_count += ch.len_utf16();
      utf8_offset += ch.len_utf8();
    }

    utf8_offset
  }

  fn offset_to_utf16(&self, offset: usize) -> usize {
    let mut utf16_offset = 0;
    let mut utf8_count = 0;

    for ch in self.content.chars() {
      if utf8_count >= offset {
        break;
      }
      utf8_count += ch.len_utf8();
      utf16_offset += ch.len_utf16();
    }

    utf16_offset
  }

  fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
    self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
  }

  fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
    self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
  }

  fn previous_boundary(&self, offset: usize) -> usize {
    self
      .content
      .grapheme_indices(true)
      .rev()
      .find_map(|(idx, _)| (idx < offset).then_some(idx))
      .unwrap_or(0)
  }

  fn next_boundary(&self, offset: usize) -> usize {
    self
      .content
      .grapheme_indices(true)
      .find_map(|(idx, _)| (idx > offset).then_some(idx))
      .unwrap_or(self.content.len())
  }

  fn next_word_boundary(&self, offset: usize) -> usize {
    let bytes = self.content.as_bytes();
    let mut idx = offset;

    // Skip any non-word characters
    while idx < bytes.len() && !bytes[idx].is_ascii_alphanumeric() {
      idx += 1;
    }

    // Skip word characters
    while idx < bytes.len() && bytes[idx].is_ascii_alphanumeric() {
      idx += 1;
    }

    idx
  }

  fn previous_word_boundary(&self, offset: usize) -> usize {
    let bytes = self.content.as_bytes();
    let mut idx = offset;

    // Move backwards skipping any non-word characters
    while idx > 0 && !bytes[idx - 1].is_ascii_alphanumeric() {
      idx -= 1;
    }

    // Move backwards skipping word characters
    while idx > 0 && bytes[idx - 1].is_ascii_alphanumeric() {
      idx -= 1;
    }

    idx
  }
}

impl EntityInputHandler for TextInput {
  fn text_for_range(
    &mut self,
    range_utf16: Range<usize>,
    actual_range: &mut Option<Range<usize>>,
    _window: &mut Window,
    _cx: &mut Context<Self>,
  ) -> Option<String> {
    let range = self.range_from_utf16(&range_utf16);
    actual_range.replace(self.range_to_utf16(&range));
    Some(self.content[range].to_string())
  }

  fn selected_text_range(
    &mut self,
    _ignore_disabled_input: bool,
    _window: &mut Window,
    _cx: &mut Context<Self>,
  ) -> Option<UTF16Selection> {
    Some(UTF16Selection {
      range: self.range_to_utf16(&self.selected_range),
      reversed: self.selection_reversed,
    })
  }

  fn marked_text_range(
    &self,
    _window: &mut Window,
    _cx: &mut Context<Self>,
  ) -> Option<Range<usize>> {
    self
      .marked_range
      .as_ref()
      .map(|range| self.range_to_utf16(range))
  }

  fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
    self.marked_range = None;
  }

  fn replace_text_in_range(
    &mut self,
    range_utf16: Option<Range<usize>>,
    new_text: &str,
    _: &mut Window,
    cx: &mut Context<Self>,
  ) {
    let range = range_utf16
      .as_ref()
      .map(|range_utf16| self.range_from_utf16(range_utf16))
      .or(self.marked_range.clone())
      .unwrap_or(self.selected_range.clone());

    self.content =
      (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..]).into();
    self.selected_range = range.start + new_text.len()..range.start + new_text.len();
    self.marked_range.take();
    cx.notify();
  }

  fn replace_and_mark_text_in_range(
    &mut self,
    range_utf16: Option<Range<usize>>,
    new_text: &str,
    new_selected_range_utf16: Option<Range<usize>>,
    _window: &mut Window,
    cx: &mut Context<Self>,
  ) {
    let range = range_utf16
      .as_ref()
      .map(|range_utf16| self.range_from_utf16(range_utf16))
      .or(self.marked_range.clone())
      .unwrap_or(self.selected_range.clone());

    self.content =
      (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..]).into();
    if !new_text.is_empty() {
      self.marked_range = Some(range.start..range.start + new_text.len());
    } else {
      self.marked_range = None;
    }
    self.selected_range = new_selected_range_utf16
      .as_ref()
      .map(|range_utf16| self.range_from_utf16(range_utf16))
      .map(|new_range| new_range.start + range.start..new_range.end + range.end)
      .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());

    cx.notify();
  }

  fn bounds_for_range(
    &mut self,
    range_utf16: Range<usize>,
    bounds: Bounds<Pixels>,
    _window: &mut Window,
    _cx: &mut Context<Self>,
  ) -> Option<Bounds<Pixels>> {
    let last_layout = self.last_layout.as_ref()?;
    let range = self.range_from_utf16(&range_utf16);
    Some(Bounds::from_corners(
      point(
        bounds.left() + last_layout.x_for_index(range.start),
        bounds.top(),
      ),
      point(
        bounds.left() + last_layout.x_for_index(range.end),
        bounds.bottom(),
      ),
    ))
  }

  fn character_index_for_point(
    &mut self,
    point: gpui::Point<Pixels>,
    _window: &mut Window,
    _cx: &mut Context<Self>,
  ) -> Option<usize> {
    let line_point = self.last_bounds?.localize(&point)?;
    let last_layout = self.last_layout.as_ref()?;

    assert_eq!(last_layout.text, self.content);
    let utf8_index = last_layout.index_for_x(point.x - line_point.x)?;
    Some(self.offset_to_utf16(utf8_index))
  }
}

impl Render for TextInput {
  fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    div()
      .flex()
      .key_context("TextInput")
      .track_focus(&self.focus_handle(cx))
      .cursor(CursorStyle::IBeam)
      .on_action(cx.listener(Self::backspace))
      .on_action(cx.listener(Self::delete))
      .on_action(cx.listener(Self::left))
      .on_action(cx.listener(Self::alt_left))
      .on_action(cx.listener(Self::cmd_left))
      .on_action(cx.listener(Self::right))
      .on_action(cx.listener(Self::alt_right))
      .on_action(cx.listener(Self::cmd_right))
      .on_action(cx.listener(Self::select_left))
      .on_action(cx.listener(Self::select_word_left))
      .on_action(cx.listener(Self::select_start))
      .on_action(cx.listener(Self::select_right))
      .on_action(cx.listener(Self::select_word_right))
      .on_action(cx.listener(Self::select_end))
      .on_action(cx.listener(Self::select_all))
      .on_action(cx.listener(Self::home))
      .on_action(cx.listener(Self::end))
      .on_action(cx.listener(Self::show_character_palette))
      .on_action(cx.listener(Self::paste))
      .on_action(cx.listener(Self::cut))
      .on_action(cx.listener(Self::copy))
      .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
      .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
      .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
      .on_mouse_move(cx.listener(Self::on_mouse_move))
      .bg(rgb(0xeeeeee))
      .line_height(px(30.))
      .child(
        div()
          .h(px(30. + 4. * 2.))
          .w_full()
          .p(px(4.))
          .bg(white())
          .child(TextElement { input: cx.entity() }),
      )
  }
}

impl Focusable for TextInput {
  fn focus_handle(&self, _: &App) -> FocusHandle {
    self.focus_handle.clone()
  }
}
