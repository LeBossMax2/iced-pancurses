use crate::{primitive::Primitive, primitive::BoxStyleOverride, TerminalRenderer};
use crate::renderer::DefaultOverride;
use iced_native::{container, Element, Layout, Point, Rectangle};

#[derive(Default, Clone)]
pub struct ContainerStyle(BoxStyleOverride, DefaultOverride);

impl<W: std::io::Write> container::Renderer for TerminalRenderer<W> {
    type Style = ContainerStyle;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        viewport: &Rectangle,
        style: &Self::Style,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        Primitive::Group(vec![
            Primitive::BoxDisplay(bounds.snap(), defaults.box_style.apply(&style.0)),
            content.draw(self, &defaults.apply(&style.1), content_layout, cursor_position, viewport)
        ])
    }
}
