use crate::primitive::Primitive;
use crate::style::CheckboxStyle;
use crate::TerminalRenderer;

use iced_native::widget::checkbox::Renderer;
use iced_native::Rectangle;

impl Renderer for TerminalRenderer {
    type Style = CheckboxStyle;

    const DEFAULT_SIZE: u16 = 1;

    const DEFAULT_SPACING: u16 = 1;

    fn draw(
        &mut self,
        bounds: Rectangle,
        is_checked: bool,
        _is_mouse_over: bool,
        label: Self::Output,
        style: &Self::Style,
    ) -> Self::Output {
        let (styling, boxchar) = if is_checked {
            &style.checked
        }
        else {
            &style.unchecked
        };
        let styling = style.base.apply(styling);

        Primitive::Group(vec![
            Primitive::Char(bounds.x as i32, bounds.y as i32, *boxchar, styling),
            label,
        ])
    }
}
