use crate::{primitive::Primitive, TerminalRenderer};
use iced_native::{row, Element, Layout, Point, Rectangle};

impl row::Renderer for TerminalRenderer {
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        children: &[Element<'_, Message, Self>],
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Self::Output {
        Primitive::Group(
            children
                .iter()
                .zip(layout.children())
                .map(|(child, layout)| child.draw(self, defaults, layout, cursor_position, viewport))
                .collect(),
        )
    }
}
