use iced_native::Color as IcedColor;
use crossterm::style::Color as TermColor;

pub fn color(color: IcedColor) -> TermColor
{
	color_from_rbg([color.r, color.g, color.b])
}

pub fn color_from_rbg(color: [f32; 3]) -> TermColor
{
	TermColor::Rgb {
		r: (color[0] * 255.0) as u8,
		g: (color[1] * 255.0) as u8,
		b: (color[2] * 255.0) as u8
	}
}