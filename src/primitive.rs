use iced_native::{Rectangle};

use terminal::Color;

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

#[derive(Clone)]
pub struct BoxStyle {
    pub background_color: Color,
    pub border: BorderStyle
}

impl Default for BoxStyle
{
    fn default() -> Self
    {
        Self {
            background_color: Color::Reset,
            border: BorderStyle::default()
        }
    }
}

impl BoxStyle
{
    pub fn with_color(mut self, background_color: Color) -> Self
    {
        self.background_color = background_color;
        self
    }

    pub fn with_border(mut self, border: BorderStyle) -> Self
    {
        self.border = border;
        self
    }

    pub fn apply(&self, overrides: &BoxStyleOverride) -> Self
    {
        let background_color = overrides.background_color.as_ref().unwrap_or(&self.background_color).clone();
        let border = overrides.border.as_ref().unwrap_or(&self.border).clone();
        return  Self { background_color, border }
    }
}

#[derive(Default, Clone)]
pub struct BoxStyleOverride {
    pub background_color: Option<Color>,
    pub border: Option<BorderStyle>
}

impl BoxStyleOverride
{
    pub fn with_color(mut self, background_color: Color) -> Self
    {
        self.background_color = Some(background_color);
        self
    }

    pub fn with_border(mut self, border: BorderStyle) -> Self
    {
        self.border = Some(border);
        self
    }

    pub fn merge(&self, other: &BoxStyleOverride) -> Self
    {
        let background_color = other.background_color.as_ref().or(self.background_color.as_ref()).cloned();
        let border = other.border.as_ref().or(self.border.as_ref()).cloned();
        return  Self { background_color, border }
    }
}

/// The 2d array containing the char to draw for each side and corner
#[derive(Clone)]
pub struct BorderStyle(pub [[char; 3]; 3]);

impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle([
            ['┌', '─', '┐'],
            ['│', ' ', '│'],
            ['└', '─', '┘']
        ])
    }
}

#[derive(Clone)]
pub struct TextStyle {
    pub background_color: Color,
    pub foreground_color: Color
}

impl TextStyle
{
    pub fn with_background_color(mut self, color: Color) -> Self
    {
        self.background_color = color;
        self
    }

    pub fn with_foreground_color(mut self, color: Color) -> Self
    {
        self.foreground_color = color;
        self
    }

    pub fn apply(&self, overrides: &TextStyleOverride) -> Self
    {
        let background_color = overrides.background_color.unwrap_or(self.background_color);
        let foreground_color = overrides.foreground_color.unwrap_or(self.foreground_color);
        return  Self { background_color, foreground_color }
    }
}

impl Default for TextStyle
{
    fn default() -> Self
    {
        Self {
            background_color: Color::Reset,
            foreground_color: Color::Reset
        }
    }
}

#[derive(Default, Clone)]
pub struct TextStyleOverride {
    pub background_color: Option<Color>,
    pub foreground_color: Option<Color>
}

impl TextStyleOverride
{
    pub fn with_background_color(mut self, color: Color) -> Self
    {
        self.background_color = Some(color);
        self
    }

    pub fn with_foreground_color(mut self, color: Color) -> Self
    {
        self.foreground_color = Some(color);
        self
    }

    pub fn merge(&self, other: &TextStyleOverride) -> Self
    {
        let background_color = other.background_color.or(self.background_color);
        let foreground_color = other.foreground_color.or(self.foreground_color);
        return  Self { background_color, foreground_color }
    }
}