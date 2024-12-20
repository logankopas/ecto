use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;

mod terminal;
use terminal::{Terminal, Coordinates};


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
            Terminal::move_cursor_to(Coordinates { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::flush_queue()?;
        Ok(())
    }
}

