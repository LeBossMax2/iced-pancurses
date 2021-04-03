use crate::TerminalRenderer;
use crate::subscription::SubscriptionPool;
use iced_native::Command;
use iced_native::{Cache, Element, Subscription, UserInterface, Size, Point};

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub trait Application: Sized {
    type Message: std::fmt::Debug + Send + Sync + Clone;

    /// Initializes the Application.
    ///
    /// This is were you should return the initial state of the application
    ///
    /// You can return a Command, if for example you need to perform async computation in the background on startup.
    /// For example: load a file, do HTTP request in the background, etc...
    fn new() -> (Self, Command<Self::Message>);

    /// Handles a message and updates the state of the Application.
    ///
    /// This is where to define an update logic. All messages produced by user interactions
    /// will be handled by this method.
    ///
    /// Additionnaly, any Command returned will be executed in the background.
    fn update(&mut self, messages: Vec<Self::Message>) -> Vec<Command<Self::Message>>;

    /// Returns the widgets to display in the Application.
    ///
    /// These widgets can produce messages based on user interaction, that will get handled
    /// by the update method.
    fn view(&mut self) -> Element<'_, Self::Message, TerminalRenderer>;

    /// Returns the event Subscription for the current state of the
    /// application.
    ///
    /// A Subscription will be kept alive as long as you keep returning it,
    /// and the __messages__ produced will be handled by
    /// [`update`](#tymethod.update).
    ///
    /// By default, this method returns an empty Subscription.
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    /// Launches the Application.
    ///
    /// This method will take ownership of the current thread, and will NOT return.
    ///
    /// This should probably be the last thing you would call at the end of the entrypoint of your program.
    fn run() -> crate::Result
    where
        Self: 'static,
    {
        // Creates the renderer and the default state
        let mut renderer = TerminalRenderer::default();//.nodelay();
        let (mut state, command) = Self::new();
        let mut cache = Cache::default();

        // Creates the threadpool for subscriptions
        let mut thread_pool = futures::executor::ThreadPool::new()
            .expect("Could not create thread pool for asynchronous operations");

        let mut subscription_pool = SubscriptionPool::default();

        // Creates an event queue to be used by async subscriptions
        let event_queue = Arc::new(Mutex::new(Some(VecDeque::default())));

        spawn_command(command, &mut thread_pool, event_queue.clone());

        let mut cursor_position = Point::new(-1.0, -1.0); // Cursor available

        loop {
            let size = renderer.size();
            let bounds = Size::new(size.0 as f32, size.1 as f32);
            subscription_pool.update(state.subscription(), &mut thread_pool, event_queue.clone());
            // Consumes the cache and renders the UI to primitives
            let mut ui = UserInterface::build(state.view(), bounds, cache, &mut renderer);

            // Displays the new state of the sandbox using the renderer
            let primitives = ui.draw(&mut renderer, cursor_position);
            renderer.draw(primitives)?;

            // Polls pancurses events and apply them on the ui, generating Application::Messages
            let mut messages = Vec::new();
            let events = renderer.handle()?;
            if !events.is_empty() {
                for event in &events {
                    if let iced_native::Event::Mouse(iced_native::mouse::Event::CursorMoved { x, y }) = *event {
                        cursor_position = Point::new(x, y);
                    }
                }

                let statuses = ui.update(&events, cursor_position, None, &renderer, &mut messages);
                events.into_iter().zip(statuses).for_each(|(e, s)| subscription_pool.broadcast(e, s));
                    
                if !messages.is_empty() {
                    renderer.flush()?;
                }
            }

            // Polls Application::Messages from the Receiver
            let mut evt_queue = event_queue.lock().expect("Poisoned lock");
            let mut events = evt_queue.take().unwrap();
            messages.append(&mut events.drain(..).collect());
            *evt_queue = Some(VecDeque::default());
            drop(evt_queue);

            // Stores back the cache
            cache = ui.into_cache();

            if !messages.is_empty() {
                // Applies updates on the state with given messages if any.
                // Launching update can generate Commands, so we spawn their futures so as to resolve them.
                let commands = state.update(messages);
                commands.into_iter().for_each(|command| {
                    spawn_command(command, &mut thread_pool, event_queue.clone())
                })
            }

            // Sleep in order to meet the FPS goal
            thread::sleep(Duration::from_millis(30));
        }
    }
}

fn spawn_command<Message: Send + 'static>(
    command: Command<Message>,
    thread_pool: &mut futures::executor::ThreadPool,
    event_queue: Arc<Mutex<Option<VecDeque<Message>>>>,
) {
    use futures::FutureExt;
    let futures = command.futures();
    for future in futures {
        let event_queue = event_queue.clone();
        let future = future.map(move |message| {
            let mut evt_queue = event_queue.lock().unwrap();
            let mut taken = evt_queue.take().unwrap();
            taken.push_back(message);
            *evt_queue = Some(taken);
        });
        thread_pool.spawn_ok(future);
    }
}
