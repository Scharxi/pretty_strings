use std::fmt::{Display, Formatter};
use std::io::Write;
use std::thread;
use std::time::Duration;

pub struct Text {
    text: String,
}

pub struct HexColor(i32, i32, i32);

impl HexColor {
    pub fn new(dec: i32) -> Self {
        let (r, g, b) = ((dec >> 16) & 0xFF, (dec >> 8) & 0xFF, dec & 0xFF);
        Self(r, g, b)
    }
}

impl From<HexColor> for (i32, i32, i32) {
    fn from(value: HexColor) -> Self {
        (value.0, value.1, value.2)
    }
}

pub trait Colored {
    fn color(&self, dec: i32) -> Self;
}

impl Colored for Text {
    fn color(&self, dec: i32) -> Self {
        let (r, g, b) = HexColor::new(dec).into();
        let text = format!("\x1b[38;2;{r};{g};{b}m{self}\x1b[39m");
        Self { text }
    }
}

pub trait Animation {
    fn step_by_step(&self, delay: f32);
}

impl Animation for Text {
    fn step_by_step(&self, delay: f32) {
        for i in self.text.chars() {
            print!("{i}");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs_f32(delay));
            print!("");
        }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

pub trait TextStyle {
    fn bold(&self) -> Self;
    fn underline(&self) -> Self;
    fn italic(&self) -> Self;
    fn strikethrough(&self) -> Self;
}

impl TextStyle for Text {
    fn bold(&self) -> Self {
        let text = format!("\x1b[1m{self}\x1b[22m");
        Self { text }
    }

    fn underline(&self) -> Self {
        let text = format!("\x1b[4m{self}\x1b[24m");
        Self { text }
    }

    fn italic(&self) -> Self {
        let text = format!("\x1b[3m{self}\x1b[23m");
        Self { text }
    }

    fn strikethrough(&self) -> Self {
        let text = format!("\x1b[9m{self}\x1b[29m");
        Self { text }
    }
}