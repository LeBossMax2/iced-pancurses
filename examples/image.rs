use iced_native::{Column, Container, Element, Image, Length, Text};
use iced_pancurses::{TerminalRenderer, Sandbox};

struct MyState;

impl Sandbox for MyState {
    type Message = ();

    fn new() -> Self {
        MyState
    }

    fn update(&mut self, _message: ()) {}

    fn view(&mut self) -> Element<'_, (), TerminalRenderer> {
        Container::new(
            Column::new()
                .spacing(1)
                .push(Text::new("Hello image !").width(Length::Shrink))
                .push(Image::new("resources/ferris.png"))
                .width(Length::Shrink),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

fn main() -> iced_pancurses::Result {
    MyState::run()
}
