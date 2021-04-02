use crate::primitive::{Primitive, BoxStyle, TextStyle};
use crate::TerminalRenderer;

use iced_native::widget::slider;
use iced_native::{Point, Rectangle};

use std::ops::RangeInclusive;

#[derive(Default, Clone)]
pub struct SliderStyle {
    background: BoxStyle,
    slider: (TextStyle, char)
}

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

        Primitive::Group(vec![
            Primitive::BoxDisplay(bounds.snap(), style.background.clone()),
            Primitive::Char(bounds.x as i32 + marker_offset as i32, bounds.y as i32, style.slider.1, style.slider.0.clone()),
        ])
    }
}
