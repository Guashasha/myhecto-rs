use std::fs;

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyModifiers,
};
use terminal::{MovementDirection, Position};
use view::View;

mod terminal;
mod user_configuration;
mod view;

pub struct Editor {
    user_controls: user_configuration::UserControls,
    current_mode: EditorMode,
    should_quit: bool,
    location: terminal::Position,
    position: terminal::Position,
    view: View,
}

enum EditorMode {
    Normal,
    Insert,
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            user_controls: user_configuration::get_user_controls()
                .expect("Couldn't load custom or default configuration"),
            current_mode: EditorMode::Normal,
            should_quit: false,
            location: Position { x: 0, y: 0 },
            position: Position { x: 0, y: 0 },
            view: View::default(),
        }
    }
}

impl Editor {
    pub fn run(&mut self, file: Option<&String>) -> Result<(), std::io::Error> {
        if let Some(file) = file {
            self.read_file(file);
        }

        terminal::initialize()?;

        loop {
            self.refresh_screen()?;

            if self.should_quit {
                Self::quit()?;
                break;
            }

            let event = read()?;
            match self.current_mode {
                EditorMode::Normal => Self::evaluate_normal_event(self, &event)?,
                EditorMode::Insert => Self::evaluate_insert_event(self, &event)?,
            }
        }

        terminal::terminate()
    }

    fn read_file(&mut self, file_path: &String) -> Result<(), std::io::Error> {
        let file_contents = fs::read_to_string(file_path)?;

        for line in file_contents.lines() {
            self.view.fill_buffer(line);
        }

        Ok(())
    }

    fn evaluate_normal_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(event) = *event {
            match event.code {
                Char('q') if event.modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                Char(c) => {
                    if self.user_controls.move_left == c {
                        self.move_caret(terminal::MovementDirection::Left, 1)?;
                    } else if self.user_controls.move_up == c {
                        self.move_caret(terminal::MovementDirection::Up, 1)?;
                    } else if self.user_controls.move_down == c {
                        self.move_caret(terminal::MovementDirection::Down, 1)?;
                    } else if self.user_controls.move_right == c {
                        self.move_caret(terminal::MovementDirection::Right, 1)?;
                    } else if self.user_controls.insert_mode == c {
                        self.change_to_insert_mode()?;
                    }
                }
                KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Up
                | KeyCode::Down => self.handle_movement_keys(&event.code)?,
                _ => (),
            }
        }

        Ok(())
    }

    fn evaluate_insert_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(event) = *event {
            match event.code {
                // print the character if there is a file
                Char(_) => todo!(),
                KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Up
                | KeyCode::Down => self.handle_movement_keys(&event.code)?,
                KeyCode::Esc => self.change_to_normal_mode()?,
                _ => (),
            }
        }

        Ok(())
    }

    fn handle_movement_keys(&mut self, key: &KeyCode) -> Result<(), std::io::Error> {
        match key {
            KeyCode::PageUp => self.move_caret(MovementDirection::Top, 0)?,
            KeyCode::PageDown => self.move_caret(MovementDirection::Bottom, 0)?,
            KeyCode::Home => self.move_caret(MovementDirection::FullLeft, 0)?,
            KeyCode::End => self.move_caret(MovementDirection::FullRight, 0)?,
            KeyCode::Left => self.move_caret(MovementDirection::Left, 1)?,
            KeyCode::Right => self.move_caret(MovementDirection::Right, 1)?,
            KeyCode::Up => self.move_caret(MovementDirection::Up, 1)?,
            KeyCode::Down => self.move_caret(MovementDirection::Down, 1)?,
            _ => (),
        }

        Ok(())
    }

    fn quit() -> Result<(), std::io::Error> {
        terminal::clear_screen()?;
        terminal::print("Closing hecto...")?;
        terminal::execute_queue()
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::quit()
        } else {
            if self.view.buffer.is_empty() {
                self.view.render_title_screen()?;
            } else {
                self.view.render()?;
            }
            terminal::move_cursor_to(&self.location)
        }
    }

    fn change_to_insert_mode(&mut self) -> Result<(), std::io::Error> {
        self.current_mode = EditorMode::Insert;
        terminal::change_to_insert_caret()
    }

    fn change_to_normal_mode(&mut self) -> Result<(), std::io::Error> {
        self.current_mode = EditorMode::Normal;
        terminal::change_to_normal_caret()
    }

    fn move_caret(
        &mut self,
        direction: MovementDirection,
        amount: usize,
    ) -> Result<(), std::io::Error> {
        match direction {
            MovementDirection::Left if self.location.x > 0 => self.location.x -= amount,
            MovementDirection::Right => self.location.x += amount,
            MovementDirection::Up if self.location.y > 0 => self.location.y -= amount,
            MovementDirection::Down => self.location.y += amount,
            MovementDirection::Top => self.location.y = 0,
            MovementDirection::Bottom => self.location.y = crossterm::terminal::size()?.1 as usize,
            MovementDirection::FullRight => {
                self.location.x = crossterm::terminal::size()?.0 as usize
            }
            MovementDirection::FullLeft => self.location.x = 0,
            _ => (),
        }

        terminal::move_cursor_to(&self.location)
    }
}
