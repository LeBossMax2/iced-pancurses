use crate::primitive::Primitive;
use crate::TerminalRenderer;
use iced_native::widget::radio::Renderer;
use iced_native::Rectangle;

impl Renderer for TerminalRenderer {
    type Style = ();

    const DEFAULT_SIZE: u16 = 1;

    const DEFAULT_SPACING: u16 = 1;

    fn draw(
        &mut self,
        bounds: Rectangle,
        is_selected: bool,
        _is_mouse_over: bool,
        label: Primitive,
        _style: &Self::Style,
    ) -> Self::Output {
        let radiochar = if is_selected { 'x' } else { 'o' };
        Primitive::Group(vec![
            Primitive::Char(bounds.x as i32, bounds.y as i32, radiochar),
            label,
        ])
    }
}
