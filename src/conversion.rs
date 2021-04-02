use iced_native::Color as IcedColor;
use terminal::Color as TermColor;

pub fn color(color: IcedColor) -> TermColor
{
	return TermColor::Rgb((color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8);
}
