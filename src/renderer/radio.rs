use crate::primitive::{Primitive, TextStyle, TextStyleOverride};
use crate::TerminalRenderer;
use iced_native::widget::radio::Renderer;
use iced_native::Rectangle;

#[derive(Clone)]
pub struct RadioStyle {
    base: TextStyle,
    selected: (TextStyleOverride, char),
    unselected: (TextStyleOverride, char)
}

impl Default for RadioStyle {
    fn default() -> Self {
        Self {
            base: Default::default(),
            selected: (Default::default(), 'x'),
            unselected: (Default::default(), 'o')
        }
    }
}

impl Renderer for TerminalRenderer {
    type Style = RadioStyle;

    const DEFAULT_SIZE: u16 = 1;

    const DEFAULT_SPACING: u16 = 1;

    fn draw(
        &mut self,
        bounds: Rectangle,
        is_selected: bool,
        _is_mouse_over: bool,
        label: Primitive,
        style: &Self::Style,
    ) -> Self::Output {
        let (styling, radiochar) = if is_selected {
            &style.selected
        }
        else {
            &style.unselected
        };
        let styling = style.base.apply(styling);

        Primitive::Group(vec![
            Primitive::Char(bounds.x as i32, bounds.y as i32, *radiochar, styling),
            label,
        ])
    }
}
