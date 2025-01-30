use core::cmp::min;
use crossterm::event::{
    read, 
    Event, 
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};
use std::{env, io::Error};

mod terminal;
use terminal::{Terminal, Coordinates};

mod view;
use view::View;


#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    caret_position: Coordinates,
    view: View
}

impl Editor {

    pub fn run(&mut self){
        // Initialize the terminal
        Terminal::initialize().unwrap();
        Terminal::flush_queue().unwrap();

        // Initialize the view
        let args: Vec<String> = env::args().collect();
        if let Some(file_arg) = args.get(1) {
            self.view.load(file_arg);
        }
        self.view.initialize().unwrap();

        // run the text editor
        let result = self.repl();

        // Teardown
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
        match &event {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                modifiers,
                ..
            }) => match(code, modifiers) {
                (&KeyCode::Char('q'), &KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageDown
                    | KeyCode::PageUp
                    | KeyCode::Home
                    | KeyCode::End,
                    _
                ) => {
                    self.handle_move_caret(*code)?;

                }
                _ => {}
            },
            Event::Resize(width_u16, height_u16) => {
                // clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
                #[allow(clippy::as_conversions)]
                let height = *height_u16 as usize;
                // clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
                #[allow(clippy::as_conversions)]
                let width = *width_u16 as usize;
                self.view.resize(Coordinates {
                    x: width,
                    y: height,
                });
            }
            _ => {}
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

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(Coordinates { x: 0, y: 0 })?;
            Terminal::write("Goodbye.\r\n")?;
        } else {
            self.view.render_full()?;
            Terminal::move_cursor_to(self.caret_position)?;
        }
        Terminal::show_cursor()?;
        Terminal::flush_queue()?;
        Ok(())
    }
}


