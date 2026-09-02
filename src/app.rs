use crossterm::event::{self, MouseButton, MouseEvent};
use ratatui::{
    layout::{Position, Rect},
    style::Stylize,
    text::Line,
    widgets::Widget,
};
use std::{default, env, fs, io, path::PathBuf, time::Duration};

use crate::{
    action::{self, Action},
    app,
};

#[derive(Debug, Default)]
pub struct Buttons {
    // id: usize,
    pub label: String,
    pub area: Rect,
    pub value: String,
}

#[derive(Debug, Default)]
pub struct App {
    /// Allow the user to exit the app
    pub exit_app: bool,
    pub folders: Vec<Vec<String>>,
    pub current_dir: PathBuf,
    pub buttons: Vec<Buttons>,
}

impl App {
    /// Constructs a new instance of `app`
    pub fn new() -> Self {
        // let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from(""));

        let entries = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        let mut app = Self {
            current_dir: entries,
            folders: Vec::new(),
            exit_app: false,
            buttons: Vec::new(),
        };

        app.load_folders();
        app
    }

    pub fn load_folders(&mut self) {
        self.folders.clear();

        println!("{:?}", fs::read_dir(&self.current_dir));
        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            // println!("{:?}", entries);
            for entry in entries.flatten() {
                let path = entry.path();

                // println!("entters");

                if path.is_dir()
                    && let Some(name) = &path.file_name().and_then(|n| n.to_str())
                {
                    self.folders.push(vec![
                        name.to_string(),
                        self.current_dir.join(name).to_string_lossy().to_string(),
                    ]);

                    // println!("{:?}", self.folders);
                }
            }
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        // catch input
        self.update()
    }

    fn update(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(250))? {
            let action_opt = Action::from_event(crossterm::event::read()?);

            if let Some(action) = action_opt {
                match action {
                    Action::Mouse(action) => {
                        let position = Position {
                            x: action.column,
                            y: action.row,
                        };

                        // Right after: let position = Position { x: ..., y: ... };

                        // eprintln!("Click at: ({}, {})", position.x, position.y);
                        // eprintln!("Buttons count: {}", self.buttons.len());
                        // for (i, btn) in self.buttons.iter().enumerate() {
                        //     eprintln!("  Button {}: label={}, area={:?}", i, btn.label, btn.area);
                        // }

                        let clicked_label = self
                            .buttons
                            .iter()
                            .find(|btn| btn.area.contains(position))
                            .map(|btn| btn.value.clone());

                        if let Some(label) = clicked_label {
                            self.current_dir = PathBuf::from(&label);
                            // println!("dir: {:?}", self.current_dir);
                            // println!("Label: {}", label);

                            self.load_folders();
                        }
                    }
                    Action::Quit => self.quit(),
                    _ => (),
                }
            }
        }

        Ok(())
    }

    pub fn quit(&mut self) {
        self.exit_app = true;
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // Render a title of layout
        Line::from("Process overview").bold().render(area, buf);
    }
}
