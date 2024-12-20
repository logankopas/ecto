use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;

mod terminal;
use terminal::{Terminal, Coordinates};

const EDITOR_NAME: &str = "Ecto";
const EDITOR_VERSION: &str = "0.0";


pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self){
        Terminal::initialize().unwrap();
        Terminal::flush_queue().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_rows() -> Result<(), Error> {
        // Assumes the cursor is in the top left already
        let Coordinates{y: height, ..} = Terminal::size()?;
        for n in 0..height {
            Terminal::move_cursor_to(Coordinates{x: 0, y: n})?;
            Terminal::clear_line()?;
            Terminal::write("~")?;
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let Coordinates { x: width, y: height } = Terminal::size()?;
        let welcome_strlen = u16::try_from(EDITOR_NAME.len()).unwrap();
        let version_strlen = u16::try_from(EDITOR_VERSION.len()).unwrap();
        // TODO check that these don't go past the end of the window
        Terminal::move_cursor_to(Coordinates { x: (width - welcome_strlen)/2 - 1, y: height/3 })?;
        Terminal::write(EDITOR_NAME)?;
        Terminal::move_cursor_to(Coordinates { x: (width - version_strlen)/2 - 1, y:(height/3) + 1 })?;
        Terminal::write(format!("v{EDITOR_VERSION}"))?;
        Ok(())
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);

        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::write("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Self::draw_welcome_message()?;
            Terminal::move_cursor_to(Coordinates { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::flush_queue()?;
        Ok(())
    }
}


