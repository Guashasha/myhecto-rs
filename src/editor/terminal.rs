use std::io::{stdout, Error, Write};

use crossterm::{
    queue,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    Command,
};

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
    Top,
    Bottom,
    FullRight,
    FullLeft,
}

fn queue_command(command: impl Command) -> Result<(), Error> {
    queue!(stdout(), command)
}

pub fn execute_queue() -> Result<(), Error> {
    stdout().flush()
}

pub fn clear_screen() -> Result<(), Error> {
    queue_command(crossterm::cursor::Hide)?;
    queue_command(Clear(crossterm::terminal::ClearType::All))?;
    queue_command(crossterm::cursor::MoveTo(0, 0))?;
    queue_command(crossterm::cursor::Show)?;
    execute_queue()
}

pub fn clear_line() -> Result<(), Error> {
    queue_command(Clear(crossterm::terminal::ClearType::CurrentLine))?;
    execute_queue()
}

pub fn print(text: &str) -> Result<(), Error> {
    queue_command(crossterm::style::Print(text))
}

pub fn move_cursor_to(position: &Position) -> Result<(), Error> {
    queue_command(crossterm::cursor::MoveTo(
        position.x as u16,
        position.y as u16,
    ))?;
    execute_queue()
}

pub fn change_to_insert_caret() -> Result<(), Error> {
    queue_command(crossterm::cursor::SetCursorStyle::BlinkingBar)?;
    execute_queue()
}

pub fn change_to_normal_caret() -> Result<(), Error> {
    queue_command(crossterm::cursor::SetCursorStyle::BlinkingBlock)?;
    execute_queue()
}

pub fn terminate() -> Result<(), Error> {
    execute_queue()?;
    disable_raw_mode()?;
    Ok(())
}

pub fn initialize() -> Result<(), Error> {
    enable_raw_mode()?;
    clear_screen()?;
    execute_queue()?;
    Ok(())
}
