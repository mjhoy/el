use doodad::*;
use ncurses::*;

pub fn render(instructions: DrawInstructions) {
    init_pair(1,
              instructions.fg.to_ncurses_constant(),
              instructions.bg.to_ncurses_constant());
    attron(COLOR_PAIR(1));
    let s : String = instructions.c.to_string();
    printw(&s);
    attroff(COLOR_PAIR(1));
}

pub fn render_empty() {
    printw(" ");
}
