// a doodad is an element on the map that can be drawn.
use ncurses::*;

#[allow(dead_code)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}

impl Color {
    pub fn to_ncurses_constant(&self) -> i16 {
        match *self {
            Color::Black   => COLOR_BLACK,
            Color::Red     => COLOR_RED,
            Color::Green   => COLOR_GREEN,
            Color::Yellow  => COLOR_YELLOW,
            Color::Blue    => COLOR_BLUE,
            Color::Magenta => COLOR_MAGENTA,
            Color::Cyan    => COLOR_CYAN,
            Color::White   => COLOR_WHITE
        }
    }
}

pub struct DrawInstructions {
    pub c  : char,
    pub fg : Color,
    pub bg : Color
}

pub trait Doodad {
    fn draw(&self) -> DrawInstructions;
    fn passable(&self) -> bool;
    fn seethrough(&self) -> bool;
}
