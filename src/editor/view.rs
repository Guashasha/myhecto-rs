use std::io::Error;

use buffer::Buffer;

use super::terminal;

mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    pub buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), Error> {
        for row in 0..crossterm::terminal::size()?.1 {
            terminal::move_cursor_to(&terminal::Position {
                x: 0,
                y: row as usize,
            })?;
            terminal::clear_line()?;
            if let Some(line) = self.buffer.contents.get(row as usize) {
                Self::draw_line(line)?;
            } else {
                Self::draw_empty_line()?;
            }
        }

        terminal::execute_queue()
    }

    pub fn render_title_screen(&self) -> Result<(), Error> {
        for row in 0..crossterm::terminal::size()?.1 {
            terminal::move_cursor_to(&terminal::Position {
                x: 0,
                y: row as usize,
            })?;
            terminal::clear_line()?;
            Self::draw_empty_line()?;
        }

        Self::draw_title()?;
        terminal::execute_queue()
    }

    fn draw_empty_line() -> Result<(), std::io::Error> {
        terminal::print("~")
    }

    fn draw_title() -> Result<(), std::io::Error> {
        let title_y_position = (crossterm::terminal::size()?.1 / 3) - 2;
        let title_x_position = (crossterm::terminal::size()?.0 / 2) - 2;

        terminal::move_cursor_to(&terminal::Position {
            x: title_x_position as usize,
            y: title_y_position as usize,
        })?;

        terminal::print(NAME)?;

        terminal::move_cursor_to(&terminal::Position {
            x: title_x_position as usize + 2,
            y: title_y_position as usize + 1,
        })?;

        terminal::print(VERSION)
    }

    fn draw_line(line: &str) -> Result<(), std::io::Error> {
        terminal::print(line)
    }

    pub fn fill_buffer(&mut self, file_contents: &str) {
        for line in file_contents.lines() {
            self.buffer.contents.push(line.to_string());
        }
    }
}
