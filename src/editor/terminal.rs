use crossterm::cursor::{MoveTo, Show, Hide};
use crossterm::{queue, Command};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::style::Print;
use core::fmt::Display;
use std::io::{stdout, Error, Write};

pub struct Terminal;

#[derive(Copy, Clone)]
pub struct Coordinates {
    // Top Left is (0, 0), values increase as you 
    // go down or right
    pub x: u16,
    pub y: u16
}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        // initialization state: screen has been cleared, cursor moved
        // to the top left position, and the queue has been flushed.
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Coordinates { x: 0, y: 0 })?;
        Self::flush_queue()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(Coordinates { x, y }: Coordinates) -> Result<(), Error> {
        Self::queue_command(MoveTo(x, y))?;
        Ok(())
    }

    pub fn write(string: impl Display) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn flush_queue() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn size() -> Result<Coordinates, Error> {
        let ( x, y ) = size()?;
        Ok(Coordinates{ x, y })
    }
}
