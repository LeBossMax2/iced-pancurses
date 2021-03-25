use crate::{primitive::Primitive, TerminalRenderer};
use iced_native::layout::Debugger;
use iced_native::{Layout, Color, Point, Widget, Rectangle};

impl Debugger for TerminalRenderer {
    fn explain<Message>(
        &mut self,
        _defaults: &Self::Defaults,
        _widget: &dyn Widget<Message, Self>,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
        _color: Color,
    ) -> Self::Output {
        Primitive::BoxDisplay(layout.bounds())
        /*let bounds = layout.bounds();
        if let Ok(sub_win) = self.terminal.subwin(
            bounds.height as i32,
            bounds.width as i32,
            bounds.y as i32,
            bounds.x as i32,
        ) {
            sub_win.border(0, 0, 0, 0, 0, 0, 0, 0);
            sub_win.delwin();
        }*/
    }
}
