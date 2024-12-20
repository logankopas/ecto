use crossterm::cursor::{MoveTo, Show, Hide};
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::style::Print;
use std::io::{stdout, Write, Error};

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
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(Coordinates { x, y }: Coordinates) -> Result<(), Error> {
        queue!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn write(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
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
