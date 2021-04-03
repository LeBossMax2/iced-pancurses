use crate::primitive::Primitive;
use crate::style::SliderStyle;
use crate::TerminalRenderer;

use iced_native::widget::slider;
use iced_native::{Point, Rectangle};

use std::ops::RangeInclusive;

impl slider::Renderer for TerminalRenderer {
    type Style = SliderStyle;

    const DEFAULT_HEIGHT: u16 = 1;

    fn draw(
        &mut self,
        bounds: Rectangle,
        _cursor_position: Point,
        range: RangeInclusive<f32>,
        value: f32,
        _is_dragging: bool,
        style: &Self::Style,
    ) -> Primitive {
        let (range_start, range_end) = range.into_inner();
        let marker_offset =
            bounds.width * ((value - range_start) / (range_end - range_start).max(1.0));
        let x = ((bounds.x + marker_offset) as i32).min((bounds.x + bounds.width) as i32 - 1);

        Primitive::Group(vec![
            Primitive::BoxDisplay(bounds.snap(), style.background.clone()),
            Primitive::Char(x, bounds.y as i32, style.slider.1, style.slider.0.clone()),
        ])
    }
}
