use std::io::Error;

use buffer::Buffer;

use super::terminal;

mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    pub buffer: Buffer,
    pub needs_redraw: bool,
    position: terminal::Position,
    scroll_offset: terminal::Position,
    width: u16,
    height: u16,
}

impl Default for View {
    fn default() -> Self {
        let (terminal_width, terminal_height) =
            crossterm::terminal::size().expect("Couln't get terminal size");

        View {
            buffer: Buffer::default(),
            needs_redraw: true,
            position: terminal::Position { x: 0, y: 0 },
            scroll_offset: terminal::Position { x: 0, y: 0 },
            width: terminal_width,
            height: terminal_height,
        }
    }
}

impl View {
    pub fn render(&mut self) -> Result<(), Error> {
        terminal::clear_screen()?;
        let mut row = 0;
        let mut text_line = 0;

        while row < self.height {
            terminal::move_cursor_to(&terminal::Position {
                x: 0,
                y: row as usize,
            })?;
            if let Some(line) = self.buffer.contents.get(text_line as usize) {
                text_line += 1;
                row += self.draw_line(&line.to_string(), row as usize)? as u16;
            }
        }

        self.needs_redraw = false;
        terminal::execute_queue()
    }

    pub fn render_title_screen(&self) -> Result<(), Error> {
        for row in 0..self.height {
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

    fn draw_line(&mut self, line: &str, row: usize) -> Result<usize, std::io::Error> {
        self.needs_redraw = false;

        let mut lines_used = 1;
        let mut l_pointer = 0;
        let mut r_pointer = self.width as usize;

        while r_pointer < line.len() {
            terminal::print(line[l_pointer..r_pointer].trim())?;
            terminal::move_cursor_to(&terminal::Position { x: 0, y: row + 1 })?;
            l_pointer += self.width as usize;
            r_pointer += self.width as usize;
            lines_used += 1;
        }

        terminal::clear_line()?;
        terminal::print(line[l_pointer..line.len()].trim())?;

        Ok(lines_used)
    }

    fn fill_buffer(&mut self, file_contents: &str) {
        for line in file_contents.lines() {
            self.buffer.contents.push(line.to_string());
        }
    }

    pub fn update_terminal_size(&mut self, width: u16, height: u16) {
        self.height = height;
        self.width = width;
        self.needs_redraw = true;
    }

    pub fn load_file(&mut self, file_path: String) -> Result<(), Error> {
        let file_contents = std::fs::read_to_string(file_path)?;

        for line in file_contents.lines() {
            self.fill_buffer(line);
        }

        Ok(())
    }
}
