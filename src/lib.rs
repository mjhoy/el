#[macro_use] extern crate log;
extern crate log4rs;
extern crate ncurses;

mod doodad;
mod player;
mod engine;
mod map;
mod elconstants;

use std::collections::LinkedList;
use std::cell::RefCell;
use ncurses::*;
use doodad::*;
use player::*;
use engine::*;
use map::*;
use elconstants::*;

pub fn run(log_path: Option<&str>) {
    startup(log_path);

    let mut quit = false;
    let mut input;

    let mut width = 0;
    let mut height = 0;
    getmaxyx(stdscr, &mut height, &mut width);

    let player = Player { x: 0, y: 0 };
    let pcell = RefCell::new(player);

    // Doodads are things that can be drawn.
    let mut doodads: LinkedList<&RefCell<Doodad>> = LinkedList::new();

    doodads.push_front(&pcell);

    let mut map = vec![vec![Tile::open(); 20]; 20];

    map[1][2] = Tile::wall();

    let center = RefCell::new((0,0));

    let engine = Engine::new(doodads, &pcell, &center, map);

    // Game loop
    while !quit {
        engine.draw_tiles();
        engine.draw();
        input = getch();
        if input == EL_QUIT {
            quit = true;
        }
        engine.update_player_pos(input);
        clear();
    }
    teardown();
}

/// Starts the game. Must eventually call teardown().
fn startup(log_path: Option<&str>) {
    match log_path {
        Some(p) => log4rs::init_file(p, Default::default()).unwrap(),
        None    => ()
    }

    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8");

    initscr();
    raw();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
}

/// Ends the game.
fn teardown() {
    endwin();
}
