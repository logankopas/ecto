use core::cmp::min;
use crossterm::event::{
    read, 
    Event::{self, Key}, 
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};
use std::io::Error;

mod terminal;
use terminal::{Terminal, Coordinates};

mod view;
use view::View;


pub struct Editor {
    should_quit: bool,
    caret_position: Coordinates
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false, caret_position: Coordinates{ x: 0, y: 0 } }
    }

    pub fn run(&mut self){
        Terminal::initialize().unwrap();
        Terminal::flush_queue().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;

        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, 
            modifiers, 
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageDown
                    | KeyCode::PageUp
                    | KeyCode::Home
                    | KeyCode::End => {
                        self.handle_move_caret(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn update_caret_position(&mut self, coordinates: Coordinates) {
        self.caret_position = coordinates;
    }

    fn handle_move_caret(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Coordinates{ mut x, mut y } = self.caret_position;
        let Coordinates{ x: width, y: height } = Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(y.saturating_add(1), height.saturating_sub(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(x.saturating_add(1), width.saturating_sub(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.update_caret_position(Coordinates { x, y });

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(Coordinates { x: 0, y: 0 })?;
            Terminal::write("Goodbye.\r\n")?;
        } else {
            View::render()?;
            Terminal::move_cursor_to(self.caret_position)?;
        }
        Terminal::show_cursor()?;
        Terminal::flush_queue()?;
        Ok(())
    }
}


