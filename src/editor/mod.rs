use crossterm::event::KeyCode;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

mod terminal;
mod user_configuration;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    user_controls: user_configuration::UserControls,
    current_mode: EditorMode,
    should_quit: bool,
}

enum EditorMode {
    Normal,
    Insert,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            user_controls: user_configuration::get_user_controls()
                .expect("Couldn't load custom or default configuration"),
            current_mode: EditorMode::Normal,
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        Self::initialize()?;

        loop {
            let event = read()?;
            match self.current_mode {
                EditorMode::Normal => Self::evaluate_normal_event(self, &event)?,
                EditorMode::Insert => Self::evaluate_insert_event(self, &event)?,
            }

            if self.should_quit {
                Self::quit()?;
                break;
            }
        }

        disable_raw_mode()?;

        Ok(())
    }

    fn evaluate_normal_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(event) = *event {
            if let Char(c) = event.code {
                if self.user_controls.move_left == c {
                    terminal::move_caret(terminal::MovementDirection::Left)?;
                    return Ok(());
                } else if self.user_controls.move_up == c {
                    terminal::move_caret(terminal::MovementDirection::Up)?;
                    return Ok(());
                } else if self.user_controls.move_down == c {
                    terminal::move_caret(terminal::MovementDirection::Down)?;
                    return Ok(());
                } else if self.user_controls.move_right == c {
                    terminal::move_caret(terminal::MovementDirection::Right)?;
                    return Ok(());
                } else if self.user_controls.insert_mode == c {
                    self.change_to_insert_mode()?;
                } else {
                    match c {
                        'q' if event.modifiers == KeyModifiers::CONTROL => {
                            self.should_quit = true;
                        }
                        _ => (),
                    }
                }
            }
        }

        Ok(())
    }

    fn evaluate_insert_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(event) = *event {
            if let KeyCode::Esc = event.code {
                self.change_to_normal_mode()?;
            }

            if let Char(c) = event.code {
                match c {
                    _ => (),
                }
            }
        }

        Ok(())
    }

    fn quit() -> Result<(), std::io::Error> {
        terminal::clear_screen()?;
        terminal::print("Closing hecto...")?;
        terminal::execute_queue()
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        terminal::clear_screen()?;
        Self::draw_initial_rows()
    }

    fn draw_initial_rows() -> Result<(), std::io::Error> {
        for row in 0..crossterm::terminal::size()?.1 {
            terminal::print("~")?;
            terminal::move_cursor_to(terminal::Position {
                x: 0,
                y: row as usize,
            })?;
        }

        Self::queue_title()?;
        terminal::move_cursor_to(terminal::Position { x: 0, y: 0 })?;
        terminal::execute_queue()
    }

    fn queue_title() -> Result<(), std::io::Error> {
        let title_y_position = (crossterm::terminal::size()?.1 / 3) - 2;
        let title_x_position = (crossterm::terminal::size()?.0 / 2) - 2;

        terminal::move_cursor_to(terminal::Position {
            x: title_x_position as usize,
            y: title_y_position as usize,
        })?;

        terminal::print(NAME)?;

        terminal::move_cursor_to(terminal::Position {
            x: title_x_position as usize + 2,
            y: title_y_position as usize + 1,
        })?;

        terminal::print(VERSION)
    }

    fn change_to_insert_mode(&mut self) -> Result<(), std::io::Error> {
        self.current_mode = EditorMode::Insert;
        terminal::change_to_insert_caret()
    }

    fn change_to_normal_mode(&mut self) -> Result<(), std::io::Error> {
        self.current_mode = EditorMode::Normal;
        terminal::change_to_normal_caret()
    }
}
