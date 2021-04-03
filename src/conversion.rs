use iced_native::Color as IcedColor;
use terminal::Color as TermColor;

pub fn color(color: IcedColor) -> TermColor
{
	return color_from_rbg([color.r, color.g, color.b]);
}

pub fn color_from_rbg(color: [f32; 3]) -> TermColor
{
	return TermColor::Rgb((color[0] * 255.0) as u8, (color[1] * 255.0) as u8, (color[2] * 255.0) as u8);
}