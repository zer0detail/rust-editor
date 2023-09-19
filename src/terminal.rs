use std::io::{self, stdout, Write};
use crate::Position;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::color;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            // Set terminal to raw_mode, which passes every character to the program as its typed
            // rather than letting a user type a whole line and waiting for <CR> to be pressed.
            // Which lets us process text in our editor as its typed instead of waiting for a new line.
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        println!("{}", termion::clear::All);
    }
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position{mut x, mut y} = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        // https://vt100.net/docs/vt100-ug/chapter3.html
        // This is the ansi escape code for Erase In Display
        // But Termion has something for that, so we use that instead.
        // print!("\x1b[2J");
        // Use Goto(1, 1) to move the cursor back up to the top left of the screen
        // otherwise we'd be down the bottom. not ideal for an editor screen.
        // Termion Goto uses the H (Cursor Position) command http://vt100.net/docs/vt100-ug/chapter3.html#CUP
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
