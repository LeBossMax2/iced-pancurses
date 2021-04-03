mod container;
mod button;
mod checkbox;
mod column;
mod debugger;
mod image;
mod radio;
mod row;
mod scrollable;
mod slider;
mod space;
mod text;
mod text_input;

use crate::primitive::Primitive;
use crate::style::DefaultStyling;
use core::time::Duration;
use iced_native::{
    keyboard, keyboard::KeyCode as IcedKeyCode,
    mouse::Button, mouse::Event as IcedMouseEvent, mouse::ScrollDelta,
    window::Event as WindowEvent
};
use iced_native::layout::Limits;
use iced_native::{Event, Renderer, Rectangle};
use std::io::Write;
use terminal::{
    Action, Attribute, Clear, Retrieved, Terminal, Value, Color,
    KeyCode as TermKeyCode, MouseEvent as TermMouseEvent, MouseButton
};

/// Terminal Renderer implementation for Iced
///
/// This is a both the shell and the renderer, it is the basic building block of your Iced
/// Application
pub struct TerminalRenderer<W: Write = std::io::Stdout> {
    /// Terminal window to use to print UI elements
    terminal: Terminal<W>,
    /// Terminal refresh delay, allows any terminal app to be non-blocking
    ///
    /// * Some(Duration) will set the target FPS of the Application
    /// * None means the application is polling user event
    refresh_delay: Option<Duration>,
}

impl Default for TerminalRenderer<std::io::Stdout> {
    /// Default config for a Pancurses renderer
    fn default() -> Self {
        match TerminalRenderer::<std::io::Stdout>::new() {
            Ok(tr) => tr,
            Err(e) => panic!("Error creating the terminal context: {}", e),
        }
    }
}

impl TerminalRenderer<std::io::Stdout> {
    pub fn new() -> crate::Result<Self> {
        let mut renderer = TerminalRenderer {
            terminal: terminal::stdout(),
            refresh_delay: None,
        };

        renderer.setup_terminal()?;

        Ok(renderer)
    }
}

impl<W: Write> Renderer for TerminalRenderer<W> {
    type Output = Primitive;

    type Defaults = DefaultStyling;

    fn layout<'a, Message>(
        &mut self,
        element: &iced_native::Element<'a, Message, Self>,
        limits: &Limits,
    ) -> iced_native::layout::Node {
        let abc = self
            .terminal
            .get(Value::TerminalSize)
            .expect("Failed to read terminal size");
        match abc {
            Retrieved::TerminalSize(x, y) => {
                let new_limits = limits.max_width(x as u32).max_height(y as u32);
                element.layout(self, &new_limits)
            }
            _ => unreachable!(),
        }
    }

    fn overlay(
        &mut self,
        base: Self::Output,
        overlay: Self::Output,
        _overlay_bounds: Rectangle,
    ) -> Self::Output
    {
        Primitive::Group(vec![base, overlay])
    }
}

fn convert_button(button: MouseButton) -> Button
{
    match button {
        MouseButton::Left => Button::Left,
        MouseButton::Right => Button::Right,
        MouseButton::Middle => Button::Middle,
        MouseButton::Unknown => Button::Other(4)
    }
}

fn move_cursor(x: u16, y: u16,) -> Event
{
    Event::Mouse(IcedMouseEvent::CursorMoved { x: x as f32 + 0.5, y: y as f32 + 0.5 })
}

fn move_cursor_and(x: u16, y: u16, other: Event) -> Vec<Event>
{
    vec![
        move_cursor(x, y),
        other
    ]
}

fn convert_modifiers(modifiers: terminal::KeyModifiers) -> keyboard::Modifiers
{
    keyboard::Modifiers {
        shift: modifiers.contains(terminal::KeyModifiers::SHIFT),
        control: modifiers.contains(terminal::KeyModifiers::CONTROL),
        alt: modifiers.contains(terminal::KeyModifiers::ALT),
        logo: false
    }
}

fn press_and_release(key_code: IcedKeyCode, modifiers: terminal::KeyModifiers) -> Vec<Event>
{
    let modifiers = convert_modifiers(modifiers);
    vec![
        Event::Keyboard(keyboard::Event::KeyPressed {
            key_code,
            modifiers
        }),
        Event::Keyboard(keyboard::Event::KeyReleased {
            key_code,
            modifiers
        })
    ]
}

fn get_border_index(value: u32, max: u32) -> usize {
    if max < 2 {
        1
    }
    else if value == 0 {
        0
    }
    else if value < max - 1 {
        1
    }
    else {
        2
    }
}

impl<W: Write> TerminalRenderer<W> {
    /// Polls event from the terminal window
    pub fn handle(&self) -> crate::Result<Vec<Event>> {
        let input = self.terminal.get(Value::Event(self.refresh_delay))?;
        Ok(match input {
            Retrieved::Event(Some(terminal::Event::Key(ke))) =>
            {
                Self::handle_key(ke)
            },
            Retrieved::Event(Some(terminal::Event::Mouse(me))) =>
            {
                Self::handle_mouse(me)
            },
            Retrieved::Event(Some(terminal::Event::Resize)) =>
            {
                let size = self.size();
                vec![Event::Window(WindowEvent::Resized { width: size.0 as u32, height: size.1 as u32 })]
            },
            /*
            Some(Input::KeyResize) => {
                self.flush();
                None
            }
            */
            _ => vec![],
        })
    }
    
    fn handle_key(event: terminal::KeyEvent) -> Vec<Event> {
        match event.code
        {
            TermKeyCode::Char(ch) => vec![Event::Keyboard(keyboard::Event::CharacterReceived(ch))],
            TermKeyCode::Backspace => press_and_release(IcedKeyCode::Backspace, event.modifiers),
            TermKeyCode::Enter => press_and_release(IcedKeyCode::Enter, event.modifiers),
            TermKeyCode::Left => press_and_release(IcedKeyCode::Left, event.modifiers),
            TermKeyCode::Right => press_and_release(IcedKeyCode::Right, event.modifiers),
            TermKeyCode::Up => press_and_release(IcedKeyCode::Up, event.modifiers),
            TermKeyCode::End => press_and_release(IcedKeyCode::End, event.modifiers),
            TermKeyCode::PageUp => press_and_release(IcedKeyCode::PageUp, event.modifiers),
            TermKeyCode::PageDown => press_and_release(IcedKeyCode::PageDown, event.modifiers),
            TermKeyCode::Tab => press_and_release(IcedKeyCode::Tab, event.modifiers),
            TermKeyCode::BackTab => press_and_release(IcedKeyCode::Tab, event.modifiers | terminal::KeyModifiers::SHIFT), // backtab = shift + tab
            TermKeyCode::Delete => press_and_release(IcedKeyCode::Delete, event.modifiers),
            TermKeyCode::Insert => press_and_release(IcedKeyCode::Insert, event.modifiers),
            TermKeyCode::Esc => press_and_release(IcedKeyCode::Escape, event.modifiers),
            TermKeyCode::F(1) => press_and_release(IcedKeyCode::F1, event.modifiers),
            TermKeyCode::F(2) => press_and_release(IcedKeyCode::F2, event.modifiers),
            TermKeyCode::F(3) => press_and_release(IcedKeyCode::F3, event.modifiers),
            TermKeyCode::F(4) => press_and_release(IcedKeyCode::F4, event.modifiers),
            TermKeyCode::F(5) => press_and_release(IcedKeyCode::F5, event.modifiers),
            TermKeyCode::F(6) => press_and_release(IcedKeyCode::F6, event.modifiers),
            TermKeyCode::F(7) => press_and_release(IcedKeyCode::F7, event.modifiers),
            TermKeyCode::F(8) => press_and_release(IcedKeyCode::F8, event.modifiers),
            TermKeyCode::F(9) => press_and_release(IcedKeyCode::F9, event.modifiers),
            TermKeyCode::F(10) => press_and_release(IcedKeyCode::F10, event.modifiers),
            TermKeyCode::F(11) => press_and_release(IcedKeyCode::F11, event.modifiers),
            TermKeyCode::F(12) => press_and_release(IcedKeyCode::F12, event.modifiers),
            TermKeyCode::F(13) => press_and_release(IcedKeyCode::F13, event.modifiers),
            TermKeyCode::F(14) => press_and_release(IcedKeyCode::F14, event.modifiers),
            TermKeyCode::F(15) => press_and_release(IcedKeyCode::F15, event.modifiers),
            TermKeyCode::F(16) => press_and_release(IcedKeyCode::F16, event.modifiers),
            TermKeyCode::F(17) => press_and_release(IcedKeyCode::F17, event.modifiers),
            TermKeyCode::F(18) => press_and_release(IcedKeyCode::F18, event.modifiers),
            TermKeyCode::F(19) => press_and_release(IcedKeyCode::F19, event.modifiers),
            TermKeyCode::F(20) => press_and_release(IcedKeyCode::F20, event.modifiers),
            TermKeyCode::F(21) => press_and_release(IcedKeyCode::F21, event.modifiers),
            TermKeyCode::F(22) => press_and_release(IcedKeyCode::F22, event.modifiers),
            TermKeyCode::F(23) => press_and_release(IcedKeyCode::F23, event.modifiers),
            TermKeyCode::F(24) => press_and_release(IcedKeyCode::F24, event.modifiers),
            _ => vec![]
        }
    }

    fn handle_mouse(event: TermMouseEvent) -> Vec<Event> {
        match event {
            TermMouseEvent::Down(button, x, y, _modifier) =>
                move_cursor_and(x, y,
                    Event::Mouse(IcedMouseEvent::ButtonPressed(convert_button(button)))),
            TermMouseEvent::Up(button, x, y, _modifier) =>
                move_cursor_and(x, y,
                    Event::Mouse(IcedMouseEvent::ButtonReleased(convert_button(button)))),
            TermMouseEvent::Drag(_button, x, y, _modifier) =>
                vec![move_cursor(x, y)],
            TermMouseEvent::ScrollDown(x, y, _modifier) =>
                move_cursor_and(x, y,
                    Event::Mouse(IcedMouseEvent::WheelScrolled { delta: ScrollDelta::Lines { x: 0.0, y: -1.0 } })),
            TermMouseEvent::ScrollUp(x, y, _modifier) =>
                move_cursor_and(x, y,
                    Event::Mouse(IcedMouseEvent::WheelScrolled { delta: ScrollDelta::Lines { x: 0.0, y: 1.0 } }))
        }
    }

    pub fn clear(&mut self) -> crate::Result {
        self.terminal.act(Action::ClearTerminal(Clear::All))
    }

    pub fn setup_terminal(&mut self) -> crate::Result {
        // Resets terminal state
        self.terminal.batch(Action::SetAttribute(Attribute::Reset))?;
        self.terminal.batch(Action::ClearTerminal(Clear::All))?;

        // Sets up various data for correct terminal processing
        self.terminal.batch(Action::EnableRawMode)?;
        self.terminal.batch(Action::EnterAlternateScreen)?;
        self.terminal.batch(Action::EnableMouseCapture)?;
        self.terminal.batch(Action::HideCursor)?;
        self.terminal.batch(Action::DisableBlinking)?;
        self.flush()?;
        Ok(())
    }

    // Sets nodelay to true in order to provide async actions
    pub fn target_fps(mut self, fps: u64) -> Self {
        self.refresh_delay = Some(Duration::from_millis(1000 / fps));
        self
    }

    /// Draws a given primitive onto the window
    pub fn draw(&mut self, primitive: Primitive) -> crate::Result {
        self.draw_batch(primitive)?;
        self.flush()?;
        Ok(())
    }
    
    fn draw_batch(&mut self, primitive: Primitive) -> crate::Result {
        match primitive {
            Primitive::Group(prims) => prims
                .into_iter()
                .map(|p| self.draw_batch(p))
                .collect::<crate::Result>(),
            Primitive::Text(texts, bounds, style) => {
                self.terminal.batch(Action::SetBackgroundColor(style.background_color))?;
                self.terminal.batch(Action::SetForegroundColor(style.foreground_color))?;
                let mut y = 0;
                texts
                    .into_iter()
                    .map(|l| {
                        self.terminal.batch(Action::MoveCursorTo(
                            bounds.x as u16,
                            bounds.y as u16 + y as u16,
                        ))?;
                        self.terminal.write(l.as_bytes())?;
                        y += 1;
                        Ok(())
                    })
                    .collect::<crate::Result>()
            }
            Primitive::BoxDisplay(bounds, style) => {
                self.terminal.batch(Action::SetBackgroundColor(style.background_color))?;
                for y in 0..bounds.height
                {
                    self.terminal.batch(Action::MoveCursorTo(
                        bounds.x as u16,
                        (y + bounds.y) as u16,
                    ))?;
                    let y_index = get_border_index(y, bounds.height);
                    for x in 0..bounds.width
                    {
                        let x_index = get_border_index(x, bounds.width);
                        write!(self.terminal, "{}", style.border.0[y_index][x_index])?;
                    }
                }
                self.terminal.batch(Action::SetBackgroundColor(Color::Reset))?;
                Ok(())
            }
            Primitive::Char(x, y, c, style) => {
                self.terminal.batch(Action::SetBackgroundColor(style.background_color))?;
                self.terminal.batch(Action::SetForegroundColor(style.foreground_color))?;
                self.terminal
                    .batch(Action::MoveCursorTo(x as u16, y as u16))?;
                self.terminal.write(format!("{}", c).as_bytes())?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn flush(&self) -> crate::Result
    {
        self.terminal.flush_batch()
    }

    /// Gets the current size of the terminal root window
    pub fn size(&self) -> (u16, u16) {
        match self
            .terminal
            .get(Value::TerminalSize)
            .expect("Failed to get terminal size")
        {
            Retrieved::TerminalSize(x, y) => (x, y),
            _ => unreachable!(),
        }
    }
}
