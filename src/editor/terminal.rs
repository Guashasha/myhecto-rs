use std::io::{stdout, Error, Write};

use crossterm::{cursor, queue, terminal::Clear, Command};

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
}

pub fn clear_screen() -> Result<(), Error> {
    queue_command(crossterm::cursor::Hide)?;
    queue_command(Clear(crossterm::terminal::ClearType::All))?;
    queue_command(crossterm::cursor::MoveTo(0, 0))?;
    queue_command(crossterm::cursor::Show)?;
    execute_queue()
}

pub fn print(text: &str) -> Result<(), Error> {
    queue_command(crossterm::style::Print(text))
}

pub fn move_cursor_to(position: Position) -> Result<(), Error> {
    queue_command(crossterm::cursor::MoveTo(
        position.x as u16,
        position.y as u16,
    ))
}

fn queue_command(command: impl Command) -> Result<(), Error> {
    queue!(stdout(), command)
}

pub fn execute_queue() -> Result<(), Error> {
    stdout().flush()
}

pub fn move_caret(direction: MovementDirection) -> Result<(), std::io::Error> {
    match direction {
        MovementDirection::Left => queue_command(crossterm::cursor::MoveLeft(1))?,
        MovementDirection::Right => queue_command(crossterm::cursor::MoveRight(1))?,
        MovementDirection::Down => queue_command(crossterm::cursor::MoveDown(1))?,
        MovementDirection::Up => queue_command(crossterm::cursor::MoveUp(1))?,
    }

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
