use crate::primitive::{Primitive, TextStyleOverride};
use crate::TerminalRenderer;
use crate::renderer::{DefaultStyling, DefaultOverride};
use iced_native::widget::text_input;
use iced_native::{HorizontalAlignment, Point, Rectangle, VerticalAlignment};

#[derive(Default, Clone)]
pub struct TextInputStyle {
    base: DefaultStyling,
    placeholder: TextStyleOverride,
    focused: DefaultOverride
}

impl text_input::Renderer for TerminalRenderer {
    type Style = TextInputStyle;

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
        state: &text_input::State,
        style: &Self::Style,
    ) -> Primitive {
        let mut styling = style.base.clone();
        if state.is_focused()
        {
            styling = styling.apply(&style.focused);
        }
        let mut text = value.to_string();
        if text == "" {
            text = placeholder.into();
            styling.text_style = styling.text_style.apply(&style.placeholder)
        }
        let bounds_text = Rectangle {
            width: src_text_bounds.width,
            height: src_text_bounds.height,
            x: src_text_bounds.x + 1.,
            y: src_text_bounds.y + 1.,
        };
        let prim_text = <Self as iced_native::widget::text::Renderer>::draw(
            self,
            &styling,
            bounds_text,
            &text,
            size - 1,
            font,
            None,
            HorizontalAlignment::Left,
            VerticalAlignment::Top,
        );
        Primitive::Group(vec![Primitive::BoxDisplay(bounds.snap(), styling.box_style), prim_text])
    }
}
