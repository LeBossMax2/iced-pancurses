use crate::primitive::Primitive;
use crate::TerminalRenderer;
use iced_native::widget::text_input;
use iced_native::{HorizontalAlignment, Point, Rectangle, VerticalAlignment};

impl text_input::Renderer for TerminalRenderer {
    type Style = ();

    fn measure_value(&self, value: &str, _size: u16, _font: Self::Font) -> f32
    {
        value.chars().count() as f32
    }

    fn offset(
        &self,
        _text_bounds: Rectangle,
        _font: Self::Font,
        _size: u16,
        _value: &text_input::Value,
        _state: &text_input::State,
    ) -> f32 {
        0.0
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        src_text_bounds: Rectangle,
        _cursor_position: Point,
        font: Self::Font,
        size: u16,
        placeholder: &str,
        value: &text_input::Value,
        _state: &text_input::State,
        _style: &Self::Style,
    ) -> Primitive {
        let mut text = value.to_string();
        if text == "" {
            text = placeholder.into();
        }
        let bounds_text = Rectangle {
            width: src_text_bounds.width,
            height: src_text_bounds.height,
            x: src_text_bounds.x + 1.,
            y: src_text_bounds.y + 1.,
        };
        let prim_text = <Self as iced_native::widget::text::Renderer>::draw(
            self,
            &Default::default(),
            bounds_text,
            &text,
            size - 1,
            font,
            None,
            HorizontalAlignment::Left,
            VerticalAlignment::Top,
        );
        Primitive::Group(vec![Primitive::BoxDisplay(bounds), prim_text])
    }
}
