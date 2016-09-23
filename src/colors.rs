use ncurses::*;

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


static COLOR_N: [Color; 8] = [Color::Black,
                              Color::Red,
                              Color::Green,
                              Color::Yellow,
                              Color::Blue,
                              Color::Magenta,
                              Color::Cyan,
                              Color::White];
pub fn color_to_index(c: Color) -> i16 {
    match c {
        Color::Black    => 0,
        Color::Red      => 1,
        Color::Green    => 2,
        Color::Yellow   => 3,
        Color::Blue     => 4,
        Color::Magenta  => 5,
        Color::Cyan     => 6,
        Color::White    => 7,
    }
}

pub fn init_colors() {
    for (i,x) in COLOR_N.iter().enumerate() {
        for (j,y) in COLOR_N.iter().enumerate() {
            let n = i + (j * 8);
            init_pair(n as i16,
                      x.to_ncurses_constant(),
                      y.to_ncurses_constant());
        }
    }
}
