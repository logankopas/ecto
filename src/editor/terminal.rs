use crossterm::cursor::{MoveTo, Show, Hide};
use crossterm::{queue, Command};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::style::Print;
use std::io::{stdout, Error, Write};


#[derive(Copy, Clone, Default)]
pub struct Coordinates {
    // Top Left is (0, 0), values increase as you 
    // go down or right
    pub x: usize,
    pub y: usize
}

/// Represents the Terminal.
/// Edge Case for platforms where `usize` < `u16`:
/// Regardless of the actual size of the Terminal, this representation
/// only spans over at most `usize::MAX` or `u16::size` rows/columns, whichever is smaller.
/// Each size returned truncates to min(`usize::MAX`, `u16::MAX`)
/// And should you attempt to set the cursor out of these bounds, it will also be turncated.
pub struct Terminal;

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

    /// Moves the cursor to the given Coordinates.
    /// # Arguments
    /// * `Coordinates` = the `Coordinates` to move the cursor to. Will be truncated to `u16::MAX`
    ///   if bigger.
    pub fn move_cursor_to(Coordinates { x, y }: Coordinates) -> Result<(), Error> {
        // clippy::as_conversions: see doc above
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(x as u16, y as u16))?;
        Ok(())
    }

    pub fn write(string: &str) -> Result<(), Error> {
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

    /// Returns the current size of this Terminal.
    /// Edge Case for systems whith `usize` < `u16`:
    /// * A `Size` representing the terminal size. Any coordinate `z` truncated to `usize` if
    ///   `usize` < `z` < `u16`
    pub fn size() -> Result<Coordinates, Error> {
        let ( x_u16, y_u16 ) = size()?;
        // clippy::as_conversions: See doc above
        #[allow(clippy::as_conversions)]
        let x = x_u16 as usize;
        // clippy::as_conversions: See doc above
        #[allow(clippy::as_conversions)]
        let y = y_u16 as usize;
        Ok(Coordinates{ x, y })
    }
}
