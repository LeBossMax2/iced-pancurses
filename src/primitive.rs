use iced_native::{Rectangle};

use crate::style::{TextStyle, BoxStyle};

pub enum Primitive {
    Char(i32, i32, char, TextStyle),
    BoxDisplay(Rectangle<u32>, BoxStyle),
    Empty,
    Group(Vec<Primitive>),
    Text(Vec<String>, Rectangle<u32>, TextStyle),
}

impl Primitive {
    pub fn with_offset(self, offset: u32) -> Primitive {
        match self {
            Primitive::BoxDisplay(mut bounds, style) => {
                bounds.y -= offset;
                Primitive::BoxDisplay(bounds, style)
            }
            Primitive::Char(x, y, content, style) => Primitive::Char(x, y - offset as i32, content, style),
            Primitive::Text(content, mut bounds, style) => {
                bounds.y -= offset;
                Primitive::Text(content, bounds, style)
            }
            Primitive::Group(primitives) => Primitive::Group(
                primitives
                    .into_iter()
                    .map(|p| p.with_offset(offset))
                    .collect(),
            ),
            _ => self,
        }
    }
}
