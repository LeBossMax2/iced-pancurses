use crate::primitive::{Primitive, BoxStyle};
use crate::TerminalRenderer;

use iced_native::widget::scrollable;
use iced_native::Rectangle;

#[derive(Default, Clone)]
pub struct ScrollableStyle {
    background: BoxStyle
}

impl scrollable::Renderer for TerminalRenderer {
    type Style = ScrollableStyle;

    fn scrollbar(
        &self,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        _offset: u32,
        _scrollbar_width: u16,
        _scrollbar_margin: u16,
        _scroller_width: u16,
    ) -> Option<scrollable::Scrollbar> {
        None
    }

    fn draw(
        &mut self,
        _scrollable: &scrollable::State,
        bounds: Rectangle,
        _content_bounds: Rectangle,
        _is_mouse_over: bool,
        _is_mouse_over_scrollbar: bool,
        _scrollbar: Option<scrollable::Scrollbar>,
        offset: u32,
        style: &Self::Style,
        content: Self::Output,
    ) -> Primitive {
        Primitive::Group(vec![
            Primitive::BoxDisplay(bounds.snap(), style.background.clone()),
            content.with_offset(offset),
        ])
    }
}
