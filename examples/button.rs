use iced_native::widget::button::State as ButtonState;
use iced_native::{Button, Cache, Column, Element, Text};
use iced_pancurses::PancursesRenderer;

#[derive(Debug, Clone, Copy)]
pub enum MyMessage {
    ClickedButton,
}

pub struct MyState {
    viewport_size: (u32, u32),
    button_state: ButtonState,
    clicked: u32,
    pub cache: Cache,
}

impl MyState {
    pub fn view(&mut self) -> Element<MyMessage, PancursesRenderer> {
        Column::new()
            .max_width(self.viewport_size.0)
            .max_height(self.viewport_size.1)
            .spacing(1)
            .push(Text::new(&format!("Button clicked {} times", self.clicked)))
            .push(
                Button::new(&mut self.button_state, Text::new("Hello!"))
                    .padding(1)
                    .on_press(MyMessage::ClickedButton),
            )
            .into()
    }

    pub fn new(viewport_size: (u32, u32)) -> Self {
        MyState {
            viewport_size,
            button_state: ButtonState::new(),
            clicked: 0,
            cache: Cache::default(),
        }
    }

    pub fn handle_messages(&mut self, messages: Vec<MyMessage>) {
        messages.into_iter().for_each(|m| match m {
            MyMessage::ClickedButton => self.clicked += 1,
        });
    }
}

fn main() {
    let mut renderer = PancursesRenderer::default();
    let (view_y, view_x) = renderer.size();
    let mut state = MyState::new((view_x, view_y));
    loop {
        let cache = state.cache.clone();
        let root = state.view();
        let mut ui = iced_native::UserInterface::build(root, cache, &mut renderer);
        let primitives = ui.draw(&mut renderer);
        renderer.draw(primitives);
        if let Some(events) = renderer.handle() {
            let messages = ui.update(&renderer, events.into_iter());
            drop(ui);
            state.handle_messages(messages);
        }
        renderer.flush();
    }
}
