use crate::{app::App, tui::Tui};
use ratatui::{Terminal, backend::CrosstermBackend};

mod app;

mod tui;

mod action;

pub mod components;

fn main() -> color_eyre::Result<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;

    let mut tui = Tui::new(terminal);
    tui.enter()?;

    // Just like Monogame or LWJGL project: Render and then logic
    while !app.exit_app {
        tui.draw(&mut app)?;

        // This should be a event handler later...
        app.run()?;
    }

    tui.exit()?;
    Ok(())
}
