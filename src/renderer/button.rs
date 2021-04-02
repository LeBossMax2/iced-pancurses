use crate::primitive::{Primitive, BoxStyleOverride};
use crate::renderer::DefaultOverride;
use crate::TerminalRenderer;
use iced_native::widget::button;
use iced_native::{Element, Layout, Point, Rectangle};

#[derive(Default, Clone)]
pub struct ButtonStyle {
    base: (BoxStyleOverride, DefaultOverride),
    disabled: (BoxStyleOverride, DefaultOverride),
    pressed: (BoxStyleOverride, DefaultOverride),
    normal: (BoxStyleOverride, DefaultOverride)
}

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
        let (styling, overrides) = if is_disabled {
            &style.disabled
        }
        else if is_pressed {
            &style.pressed
        }
        else {
            &style.normal
        };
        let styling = style.base.0.merge(styling);
        let overrides = style.base.1.merge(overrides);

        Primitive::Group(vec![
            Primitive::BoxDisplay(bounds.snap(), defaults.box_style.apply(&styling)),
            content.draw(self, &defaults.apply(&overrides), content_layout, cursor_position, &bounds)
        ])
    }
}
