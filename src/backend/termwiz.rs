use crate::{
    backend::Backend,
    buffer::Cell,
    layout::Rect,
    style::{Color, Modifier},
};
use std::{error::Error, io};
use termwiz::{
    caps::Capabilities,
    cell::*,
    color::*,
    surface::*,
    terminal::{buffered::BufferedTerminal, SystemTerminal, Terminal},
};

pub struct TermwizBackend {
    buffered_terminal: BufferedTerminal<SystemTerminal>,
}

impl TermwizBackend {
    pub fn new() -> Result<TermwizBackend, Box<dyn Error>> {
        let mut buffered_terminal =
            BufferedTerminal::new(SystemTerminal::new(Capabilities::new_from_env()?)?)?;
        buffered_terminal.terminal().set_raw_mode()?;
        buffered_terminal.terminal().enter_alternate_screen()?;
        Ok(TermwizBackend { buffered_terminal })
    }

    pub fn with_buffered_terminal(instance: BufferedTerminal<SystemTerminal>) -> TermwizBackend {
        TermwizBackend {
            buffered_terminal: instance,
        }
    }

    pub fn buffered_terminal(&self) -> &BufferedTerminal<SystemTerminal> {
        &self.buffered_terminal
    }

    pub fn buffered_terminal_mut(&mut self) -> &mut BufferedTerminal<SystemTerminal> {
        &mut self.buffered_terminal
    }
}

impl Backend for TermwizBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, cell) in content {
            self.buffered_terminal.add_changes(vec![
                Change::CursorPosition {
                    x: Position::Absolute(x as usize),
                    y: Position::Absolute(y as usize),
                },
                Change::Attribute(AttributeChange::Foreground(cell.style.fg.into())),
                Change::Attribute(AttributeChange::Background(cell.style.bg.into())),
            ]);

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Intensity(
                    if cell.style.modifier.contains(Modifier::BOLD) {
                        Intensity::Bold
                    } else if cell.style.modifier.contains(Modifier::DIM) {
                        Intensity::Half
                    } else {
                        Intensity::Normal
                    },
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Italic(
                    cell.style.modifier.contains(Modifier::ITALIC),
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Underline(
                    if cell.style.modifier.contains(Modifier::UNDERLINED) {
                        Underline::Single
                    } else {
                        Underline::None
                    },
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Reverse(
                    cell.style.modifier.contains(Modifier::REVERSED),
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Invisible(
                    cell.style.modifier.contains(Modifier::HIDDEN),
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::StrikeThrough(
                    cell.style.modifier.contains(Modifier::CROSSED_OUT),
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Blink(
                    if cell.style.modifier.contains(Modifier::SLOW_BLINK) {
                        Blink::Slow
                    } else if cell.style.modifier.contains(Modifier::RAPID_BLINK) {
                        Blink::Rapid
                    } else {
                        Blink::None
                    },
                )));

            self.buffered_terminal.add_change(&cell.symbol);
        }
        Ok(())
    }
    fn hide_cursor(&mut self) -> Result<(), io::Error> {
        self.buffered_terminal
            .add_change(Change::CursorShape(CursorShape::Hidden));
        Ok(())
    }
    fn show_cursor(&mut self) -> Result<(), io::Error> {
        self.buffered_terminal
            .add_change(Change::CursorShape(CursorShape::Default));
        Ok(())
    }
    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        let (x, y) = self.buffered_terminal.cursor_position();
        Ok((x as u16, y as u16))
    }
    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.buffered_terminal.add_change(Change::CursorPosition {
            x: Position::Absolute(x as usize),
            y: Position::Absolute(y as usize),
        });

        Ok(())
    }
    fn clear(&mut self) -> Result<(), io::Error> {
        self.buffered_terminal
            .add_change(Change::ClearScreen(termwiz::color::ColorAttribute::Default));
        Ok(())
    }
    fn size(&self) -> Result<Rect, io::Error> {
        let (term_width, term_height) = self.buffered_terminal.dimensions();
        let max = u16::max_value();
        Ok(Rect::new(
            0,
            0,
            if term_width > usize::from(max) {
                max
            } else {
                term_width as u16
            },
            if term_height > usize::from(max) {
                max
            } else {
                term_height as u16
            },
        ))
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        self.buffered_terminal
            .flush()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(())
    }
}

impl Into<ColorAttribute> for Color {
    fn into(self) -> ColorAttribute {
        match self {
            Color::Reset => ColorAttribute::Default,
            Color::Black | Color::Gray | Color::DarkGray => AnsiColor::Black.into(),
            Color::Red | Color::LightRed => AnsiColor::Red.into(),
            Color::Green | Color::LightGreen => AnsiColor::Green.into(),
            Color::Yellow | Color::LightYellow => AnsiColor::Yellow.into(),
            Color::Magenta | Color::LightMagenta => AnsiColor::Purple.into(),
            Color::Cyan | Color::LightCyan => AnsiColor::Aqua.into(),
            Color::White => AnsiColor::White.into(),
            Color::Blue | Color::LightBlue => AnsiColor::Blue.into(),
            Color::Indexed(i) => ColorAttribute::PaletteIndex(i),
            Color::Rgb(r, g, b) => ColorAttribute::TrueColorWithPaletteFallback(
                RgbColor::new(r, g, b),
                AnsiColor::Black.into(),
            ),
        }
    }
}
