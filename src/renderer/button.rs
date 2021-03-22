use crate::primitive::Primitive;
use crate::TerminalRenderer;
use iced_native::widget::button;
use iced_native::{Element, Layout, Point, Rectangle};

impl button::Renderer for TerminalRenderer {
    const DEFAULT_PADDING: u16 = 1;

    type Style = ();

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        _is_disabled: bool,
        _is_pressed: bool,
        _style: &Self::Style,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        Primitive::Group(vec![
            Primitive::BoxDisplay(bounds),
            content.draw(self, defaults, content_layout, cursor_position, &bounds)
        ])
    }
}
