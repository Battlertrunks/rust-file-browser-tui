use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind,
};

/// TODO: Make sure the enum values are being used
#[allow(dead_code)]
pub enum Action {
    // System & lifecyle actions
    Render,
    Resize(u16, u16),
    Quit,

    // inputs
    Key(KeyEvent),
    Mouse(MouseEvent), // Application-specific Intents

                       // Asynchronous / Background Task Results
}

impl Action {
    pub fn from_event(event: Event) -> Option<Self> {
        match event {
            // Handle key press events
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => Some(Action::Quit),
                _ => Some(Action::Key(key)),
            },

            Event::Mouse(mouse) if mouse.kind.is_down() => match mouse.kind {
                MouseEventKind::Down(MouseButton::Left) => Some(Action::Mouse(mouse)), // Will change when making the folders
                _ => Some(Action::Mouse(mouse)),
            },

            // Terminal resize events
            Event::Resize(width, height) => Some(Action::Resize(width, height)),

            _ => None,
        }
    }
}
