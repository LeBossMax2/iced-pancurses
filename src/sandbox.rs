use crate::TerminalRenderer;
use iced_native::{Cache, Container, Element, Length, UserInterface, Point, Size};
use std::io::Stdout;

pub trait Sandbox: Sized {
    type Message: std::fmt::Debug + Send + Clone;

    /// Initializes the Sanbox
    ///
    /// Should return the initial state of the sandbox
    fn new() -> Self;

    /// Handles the dispatch of a message and updates the state of the sandbox
    ///
    /// This function should define the update logic.
    /// All messages produced by user interaction will be handled here.
    fn update(&mut self, message: Self::Message);

    /// Request drawing the new state of the UI
    ///
    /// Returns the root element to display using the renderer
    fn view(&mut self) -> Element<'_, Self::Message, TerminalRenderer<Stdout>>;

    /// Launches the sandbox and takes ownership of the current thread.
    ///
    /// This should be the last thing you execute at the end of the entrypoint of
    /// your program.
    ///
    /// TODO: Should support custom Writer
    fn run() -> crate::Result
    where
        Self: 'static,
    {
        // Creates the sandbox and its renderer
        let mut renderer = TerminalRenderer::<Stdout>::default();
        let mut state = Self::new();

        let mut cache = Some(Cache::default());
        let cursor_position = Point::default();

        let mut messages = Vec::new();

        loop {
            renderer.clear();
            let size = renderer.size();
            let bounds = Size::new(size.0 as f32, size.1 as f32);
            // Consumes the cache and renders the UI to primitives
            let view: Element<'_, Self::Message, TerminalRenderer<Stdout>> =
                Container::new(state.view())
                    .width(Length::Units(size.0))
                    .height(Length::Units(size.1))
                    .into();
            let mut ui = UserInterface::build(view, bounds, cache.take().unwrap(), &mut renderer);

            // Displays the new state of the sandbox using the renderer
            let primitives = ui.draw(&mut renderer, cursor_position);
            renderer.draw(primitives);

            // Polls pancurses events and apply them on the ui
            renderer
                .handle()?
                .map(|event| ui.update(&[event], cursor_position, None, &renderer, &mut messages));

            // Stores back the cache
            cache = Some(ui.into_cache());

            // Applies updates on the state with given messages if any
            for message in messages.drain(..) {
                state.update(message);
            }
        }
    }
}
