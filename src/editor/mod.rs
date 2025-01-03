use std::panic::{set_hook, take_hook};

use log::error;

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers, MouseEventKind,
};
use terminal::MovementDirection;
use view::View;

mod terminal;
mod user_configuration;
mod view;

pub struct Editor {
    user_controls: user_configuration::UserControls,
    current_mode: EditorMode,
    should_quit: bool,
    view: View,
}

enum EditorMode {
    Normal,
    Insert,
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::terminate().expect("Couldn't close hecto correctly");
        if self.should_quit {
            let _ = terminal::print("\rClosing hecto...");
        }
    }
}

impl Editor {
    pub fn new(file: Option<&String>) -> Result<Self, std::io::Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = terminal::terminate();
            error!("Program panicked by error: {:?}", panic_info);
            current_hook(panic_info);
        }));

        let mut editor = Editor {
            user_controls: user_configuration::get_user_controls().expect(
                "Couldn't load custom or default configuration, JSON file isn't formatted correctly",
            ),
            current_mode: EditorMode::Normal,
            should_quit: false,
            view: View::default(),
        };

        if let Some(file) = file {
            if let Err(err) = editor.view.load_file(file.to_string()) {
                error!("Couldn't open file: {err}");
            }
        }

        terminal::initialize()?;

        Ok(editor)
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                Self::quit()?;
                break;
            }

            match read() {
                Ok(event) => self.handle_event(event)?,
                Err(err) => {
                    error!("Couldn't read event: {err}");
                    panic!("couldn't read event correctly");
                }
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<(), std::io::Error> {
        if let Event::Resize(width, height) = event {
            self.view.update_terminal_size(width, height);
        } else if let Event::Mouse(mouse_event) = event {
            self.handle_mouse_events(mouse_event.kind);
        } else if let Key(key_event) = event {
            match key_event.code {
                KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Up
                | KeyCode::Down => self.handle_movement_keys(&key_event.code)?,
                Char('q') if key_event.modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true
                }
                _ => match self.current_mode {
                    EditorMode::Normal => self.evaluate_normal_event(key_event)?,
                    EditorMode::Insert => self.evaluate_insert_event(key_event)?,
                },
            }
        }

        Ok(())
    }

    fn evaluate_normal_event(&mut self, event: KeyEvent) -> Result<(), std::io::Error> {
        if let Char(c) = event.code {
            if self.user_controls.move_left == c {
                self.view.move_caret(terminal::MovementDirection::Left, 1)?;
            } else if self.user_controls.move_up == c {
                self.view.move_caret(terminal::MovementDirection::Up, 1)?;
            } else if self.user_controls.move_down == c {
                self.view.move_caret(terminal::MovementDirection::Down, 1)?;
            } else if self.user_controls.move_right == c {
                self.view
                    .move_caret(terminal::MovementDirection::Right, 1)?;
            } else if self.user_controls.insert_mode == c {
                self.change_to_insert_mode();
            }
        }

        Ok(())
    }

    fn evaluate_insert_event(&mut self, event: KeyEvent) -> Result<(), std::io::Error> {
        match event.code {
            Char(_) => self.view.needs_redraw = true,
            KeyCode::Esc => self.change_to_normal_mode(),
            _ => (),
        }

        Ok(())
    }

    fn handle_movement_keys(&mut self, key: &KeyCode) -> Result<(), std::io::Error> {
        match key {
            KeyCode::PageUp => self.view.move_caret(MovementDirection::Top, 0)?,
            KeyCode::PageDown => self.view.move_caret(MovementDirection::Bottom, 0)?,
            KeyCode::Home => self.view.move_caret(MovementDirection::FullLeft, 0)?,
            KeyCode::End => self.view.move_caret(MovementDirection::FullRight, 0)?,
            KeyCode::Left => self.view.move_caret(MovementDirection::Left, 1)?,
            KeyCode::Right => self.view.move_caret(MovementDirection::Right, 1)?,
            KeyCode::Up => self.view.move_caret(MovementDirection::Up, 1)?,
            KeyCode::Down => self.view.move_caret(MovementDirection::Down, 1)?,
            _ => (),
        }

        Ok(())
    }

    fn handle_mouse_events(&mut self, mouse_event: MouseEventKind) {
        let direction = match mouse_event {
            MouseEventKind::ScrollDown => MovementDirection::Down,
            MouseEventKind::ScrollUp => MovementDirection::Up,
            MouseEventKind::ScrollLeft => MovementDirection::Left,
            MouseEventKind::ScrollRight => MovementDirection::Right,
            _ => return,
        };

        self.view.scroll(direction, 1);
    }

    fn quit() -> Result<(), std::io::Error> {
        terminal::clear_screen()?;
        terminal::execute_queue()
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::quit()
        } else if self.view.needs_redraw {
            if self.view.buffer.is_empty() {
                self.view.render_title_screen()?;
            } else {
                self.view.render()?;
            }

            terminal::move_cursor_to(&self.view.position)
        } else {
            Ok(())
        }
    }

    fn change_to_insert_mode(&mut self) {
        self.current_mode = EditorMode::Insert;
        terminal::change_to_insert_caret();
    }

    fn change_to_normal_mode(&mut self) {
        self.current_mode = EditorMode::Normal;
        terminal::change_to_normal_caret();
    }
}
