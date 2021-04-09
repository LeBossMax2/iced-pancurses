pub use crossterm::style::Color;

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


#[derive(Default, Clone)]
pub struct DefaultStyling {
    pub box_style: BoxStyle,
    pub text_style: TextStyle
}

impl DefaultStyling
{
    pub fn with_box_style(mut self, style: BoxStyle) -> Self
    {
        self.box_style = style;
        self
    }

    pub fn with_text_style(mut self, style: TextStyle) -> Self
    {
        self.text_style = style;
        self
    }
	
    pub fn with_background_color(mut self, color: Color) -> Self
    {
        self.box_style.background_color = color;
        self.text_style.background_color = color;
        self
    }

    pub fn apply(&self, other: &DefaultOverride) -> Self
    {
        let box_style = self.box_style.apply(&other.box_style);
        let text_style = self.text_style.apply(&other.text_style);
        return  Self { box_style, text_style }
    }
}

#[derive(Default, Clone)]
pub struct DefaultOverride {
    pub box_style: BoxStyleOverride,
    pub text_style: TextStyleOverride
}

impl DefaultOverride
{
    pub fn with_box_style(mut self, style: BoxStyleOverride) -> Self
    {
        self.box_style = style;
        self
    }

    pub fn with_text_style(mut self, style: TextStyleOverride) -> Self
    {
        self.text_style = style;
        self
    }
	
    pub fn with_background_color(mut self, color: Color) -> Self
    {
        self.box_style.background_color = Some(color);
        self.text_style.background_color = Some(color);
        self
    }

    pub fn merge(&self, other: &DefaultOverride) -> Self
    {
        let box_style = self.box_style.merge(&other.box_style);
        let text_style = self.text_style.merge(&other.text_style);
        return  Self { box_style, text_style }
    }
}


#[derive(Default, Clone)]
pub struct ButtonStyle {
    pub base: DefaultOverride,
    pub disabled: DefaultOverride,
    pub pressed: DefaultOverride,
    pub normal: DefaultOverride
}

#[derive(Clone)]
pub struct CheckboxStyle {
    pub base: TextStyle,
    pub checked: (TextStyleOverride, char),
    pub unchecked: (TextStyleOverride, char)
}

impl Default for CheckboxStyle {
    fn default() -> Self {
        Self {
            base: Default::default(),
            checked: (Default::default(), 'x'),
            unchecked: (Default::default(), 'o')
        }
    }
}

#[derive(Default, Clone)]
pub struct ContainerStyle(pub BoxStyleOverride, pub DefaultOverride);

#[derive(Clone)]
pub struct RadioStyle {
    pub base: TextStyle,
    pub selected: (TextStyleOverride, char),
    pub unselected: (TextStyleOverride, char)
}

impl Default for RadioStyle {
    fn default() -> Self {
        Self {
            base: Default::default(),
            selected: (Default::default(), 'x'),
            unselected: (Default::default(), 'o')
        }
    }
}

#[derive(Default, Clone)]
pub struct ScrollableStyle {
    pub background: BoxStyle
}

#[derive(Clone)]
pub struct SliderStyle {
    pub background: BoxStyle,
    pub slider: (TextStyle, char)
}

impl Default for SliderStyle {
    fn default() -> Self {
        SliderStyle {
            background: BoxStyle {
                background_color: Color::Reset,
                border: BorderStyle([
                    ['├', '─', '┤'],
                    ['├', '─', '┤'],
                    ['├', '─', '┤']
                ])
            },
            slider: (Default::default(), 'O')
        }
    }
}

#[derive(Default, Clone)]
pub struct TextInputStyle {
    pub base: DefaultStyling,
    pub placeholder: TextStyleOverride,
    pub focused: DefaultOverride
}
