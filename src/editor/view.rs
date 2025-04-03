use super::terminal::{Coordinates, Terminal};

mod buffer;
use buffer::Buffer;

const EDITOR_NAME: &str = "Ecto";
const EDITOR_VERSION: &str = "0.0";


pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Coordinates
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn initialize(&mut self) {
        self.draw_empty_screen();
        if self.buffer.is_empty() {
            self.draw_welcome_message();
        } else {
            self.render_full();
        }

    }

    pub fn render_full(&mut self) {
        if !self.needs_redraw {
            return;
        }
        let Coordinates { x:width, y: height} = self.size;
        if height == 0 || width == 0 {
            return;
        }

        if self.buffer.is_empty() {
            self.draw_empty_screen();
            self.draw_welcome_message();
            self.needs_redraw = false;
            return;
        }

        for current_row in 0..height {
            if let Some(line) = self.buffer.data.get(current_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(current_row, truncated_line);
            }
        }
        self.needs_redraw = false;
    }

    pub fn render_line(at: usize, text: &str) {
        let result = Terminal::write_line(at, text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn resize(&mut self, to: Coordinates) {
        self.size = to;
        self.needs_redraw = true;
    }

    fn draw_welcome_message(&self) {
        // TODO handle the case where the message is smaller than the width
        let Coordinates { x: width, y: height } = self.size;
        let welcome_strlen = EDITOR_NAME.len();
        // Allow this integer division because we don't care if the welcome message is _exactly
        // centered on the screen.
        #[allow(clippy::integer_division)]
        let editor_line = height / 3;
        let version_line = editor_line.saturating_add(1);
        #[allow(clippy::integer_division)]
        let padding_count = (width.saturating_sub(welcome_strlen) / 2).saturating_sub(1);

        // add padding to strings and draw
        let padding = " ".repeat(padding_count);
        let editor_text = &format!("~{padding}{EDITOR_NAME}");
        Self::render_line(editor_line, editor_text);
        let version_text = &format!("~{padding}v{EDITOR_VERSION}");
        Self::render_line(version_line, version_text);
    }

    fn draw_empty_screen(&self) {
        let Coordinates{y: height, ..} = self.size;
        for n in 0..height {
            Self::render_line(n, "~");
        }
    }
}

