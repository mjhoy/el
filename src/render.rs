use doodad::*;
use ncurses::*;

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

pub fn render(instructions: DrawInstructions) {
    let fg = instructions.fg;
    let bg = instructions.bg;
    let n = color_to_index(fg) + (color_to_index(bg) * 8);
    attron(COLOR_PAIR(n));
    let s : String = instructions.c.to_string();
    printw(&s);
    attroff(COLOR_PAIR(1));
}

pub fn render_empty() {
    printw(" ");
}
