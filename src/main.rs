#[macro_use]
extern crate ncurses;

use ncurses::*;

fn main() {
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8"); // if your locale is like mine(zh_CN.UTF-8)

    initscr();
    start_color();

    intro();
    endwin();
}


fn intro() {
    let title = "L";

    let mut row = 0;
    let mut col = 0;

    getmaxyx(stdscr, &mut row, &mut col);

    attron(A_BOLD());
    attron(A_BOLD());
    mvprintw(
        (row / 2) - 1,
        (col - (title.chars().count() as i32)) / 2,
        &format!("{}", title));
    attroff(A_BOLD());

    mvprintw(
        row-1,
        0,
        "hit a key to continue ");

    refresh();
    getch();

    setupname();
}


fn setupname() {
    let default_name = "mikey";
    let mut name: String = "".to_string();

    let mut row = 0;
    let mut col = 0;

    getmaxyx(stdscr, &mut row, &mut col);

    // not gonna fuck around with getpwuid()...

    clear();
    mvprintw(
        row-1,
        0,
        &format!("what's your name? [{}] ", default_name));
    getstr(&mut name);
    if name.chars().count() == 0 {
        name = default_name.to_string();
    }

    clear();
    mvprintw(
        row-1,
        0,
        &format!("hello, {}! (hit a key to continue)", name)
            );
    refresh();
    getch();
}
