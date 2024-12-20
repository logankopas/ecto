use super::terminal::{Coordinates, Terminal};
use std::io::Error;

const EDITOR_NAME: &str = "Ecto";
const EDITOR_VERSION: &str = "0.0";


pub struct View {}

impl View {
    pub fn render() -> Result<(), Error>{
        View::draw_empty_screen()?;
        View::draw_welcome_message()?;
        View::hello_world()?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let Coordinates { x: width, y: height } = Terminal::size()?;
        let welcome_strlen = EDITOR_NAME.len();
        let version_strlen = EDITOR_VERSION.len();
        // Allow this integer division because we don't care if the welcome message is _exactly
        // centered on the screen.
        // TODO we still need to check that this won't overflow the screen
        #[allow(clippy::integer_division)]
        Terminal::move_cursor_to(Coordinates { x: (width.saturating_sub(welcome_strlen))/2, y: height/3 })?;
        Terminal::write(EDITOR_NAME)?;
        #[allow(clippy::integer_division)]
        Terminal::move_cursor_to(Coordinates { x: (width.saturating_sub(version_strlen))/2, y:(height/3).saturating_add(1) })?;
        Terminal::write(&format!("v{EDITOR_VERSION}"))?;
        Ok(())
    }

    fn draw_empty_screen() -> Result<(), Error> {
        let Coordinates{y: height, ..} = Terminal::size()?;
        for n in 0..height {
            Terminal::move_cursor_to(Coordinates{x: 0, y: n})?;
            Terminal::clear_line()?;
            Terminal::write("~")?;
        }
        Ok(())
    }

    fn hello_world() -> Result<(), Error> {
        Terminal::move_cursor_to(Coordinates { x: 0, y: 0 })?;
        Terminal::clear_line()?;
        Terminal::write("Hello, friends!")
    }
}

