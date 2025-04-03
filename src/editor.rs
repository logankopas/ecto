use core::cmp::min;
use crossterm::event::{
    read,
    Event,
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};
use std::{
    env, io::Error,
    panic::take_hook, panic::set_hook
};

mod terminal;
use terminal::{Terminal, Coordinates};

mod view;
use view::View;


pub struct Editor {
    should_quit: bool,
    caret_position: Coordinates,
    view: View
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        // Handle panics
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        // Initialize the terminal
        Terminal::initialize()?;

        // Initialize the view
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_arg) = args.get(1) {
            view.load(file_arg);
        }
        view.initialize();

        Ok(Self {
            should_quit: false,
            caret_position: Coordinates::default(),
            view
        })
    }

    pub fn run(&mut self){

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
            match read() {
                Ok(event) => self.evaluate_event(&event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}")
                    }
                }
            }


        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
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
                    self.handle_move_caret(*code);

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
    }

    fn update_caret_position(&mut self, coordinates: Coordinates) {
        self.caret_position = coordinates;
    }

    fn handle_move_caret(&mut self, key_code: KeyCode) {
        let Coordinates{ mut x, mut y } = self.caret_position;
        let Coordinates{ x: width, y: height } = Terminal::size().unwrap_or_default();
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
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        let _ = Terminal::hide_cursor();
        self.view.render_full();
        let _ = Terminal::move_cursor_to(self.caret_position);
        Terminal::show_cursor()?;
        Terminal::flush_queue()?;
        Ok(())
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::write("Goodbye\r\n");
        }
    }
}


