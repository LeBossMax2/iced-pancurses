use crate::primitive::Primitive;
use crate::style::ButtonStyle;
use crate::TerminalRenderer;
use iced_native::widget::button;
use iced_native::{Element, Layout, Point, Rectangle};

impl button::Renderer for TerminalRenderer {
    const DEFAULT_PADDING: u16 = 1;

    type Style = ButtonStyle;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        is_disabled: bool,
        is_pressed: bool,
        style: &Self::Style,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        let styling = if is_disabled {
            &style.disabled
        }
        else if is_pressed {
            &style.pressed
        }
        else {
            &style.normal
        };
        let styling = style.base.merge(styling);
        let defaults = defaults.apply(&styling);

        let content_primitive = content.draw(self, &defaults, content_layout, cursor_position, &bounds);

        Primitive::Group(vec![
            Primitive::BoxDisplay(bounds.snap(), defaults.box_style),
            content_primitive
        ])
    }
}
