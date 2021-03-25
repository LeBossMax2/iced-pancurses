use crate::{primitive::Primitive, TerminalRenderer};
use iced_native::{container, Element, Layout, Point, Rectangle};

impl<W: std::io::Write> container::Renderer for TerminalRenderer<W> {
    type Style = ();

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        viewport: &Rectangle,
        _style: &Self::Style,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        Primitive::Group(vec![
            Primitive::BoxDisplay(bounds),
            content.draw(self, defaults, content_layout, cursor_position, viewport)
        ])
    }
}
