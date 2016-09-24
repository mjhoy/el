#[macro_use] extern crate log;
extern crate log4rs;
extern crate ncurses;

use std::{thread, time};
use ncurses::*;
mod window;
use window::*;

/// Starts the game. Must eventually call teardown().
pub fn startup(log_path: Option<&str>) {
    match log_path {
        Some(p) => log4rs::init_file(p, Default::default()).unwrap(),
        None    => ()
    }

    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8"); // if your locale is like mine(zh_CN.UTF-8)

    initscr();
    start_color();
}

/// Ends the game.
pub fn teardown() {
    endwin();
}

fn printw_centered(width: i32, row: i32, message: &str) {
    mvprintw(
        row,
        (width - message.chars().count() as i32) / 2,
        &format!("{}", message)
        );
}

pub fn intro() {
    let title = "L";

    let mut row = 0;
    let mut col = 0;

    getmaxyx(stdscr, &mut row, &mut col);

    let mid_y = (row / 2) - 1;

    attron(A_BOLD());
    printw_centered(col, mid_y, title);
    attroff(A_BOLD());
    refresh();

    thread::sleep(time::Duration::from_millis(1500));
    printw_centered(
        col,
        mid_y + 1,
        "hit the any key to continue ");
    refresh();
    getch();

    setupname();

    let mut max_x = 0;
    let mut max_y = 0;

    getmaxyx(stdscr, &mut max_y, &mut max_x);

    let field = Window::new(max_y, max_x - 25, 0, 0);
    let console = Window::new(max_y, 25, 0, (max_x - 25));

    field.refresh();
    console.refresh();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    let _ = getch();

    field.msg("hello there.");
    field.refresh();
    let _ = getch();
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
