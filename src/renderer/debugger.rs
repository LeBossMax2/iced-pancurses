use crate::{primitive::Primitive, primitive::BoxStyle, TerminalRenderer};
use iced_native::layout::Debugger;
use iced_native::{Layout, Color, Point, Widget, Rectangle};
use terminal::Color as TermColor;

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
        let mut primitives = Vec::new();
        explain_layout(layout, 0, &mut primitives);
        Primitive::Group(primitives)
    }
}

fn explain_layout(layout: Layout<'_>, depth: u8, output: &mut Vec<Primitive>) {
    let style = BoxStyle::default().with_color(TermColor::AnsiValue(depth));
    output.push(Primitive::BoxDisplay(layout.bounds().snap(), style));
    for child in layout.children() {
        explain_layout(child, depth + 1, output);
    }
}