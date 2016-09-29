use std::collections::LinkedList;
use std::cell::RefCell;
use doodad::*;
use player::*;
use map::*;
use elconstants::*;
use ncurses::*;

pub struct Engine <'a> {
    pub doodads: LinkedList<&'a RefCell<Doodad>>,
    pub player: &'a RefCell<Player>,
    pub center: &'a RefCell<(i32, i32)>,
    pub map: Map,
}

impl<'a> Engine<'a> {
    pub fn new(
        doodads: LinkedList<&'a RefCell<Doodad>>,
        player: &'a RefCell<Player>,
        center: &'a RefCell<(i32, i32)>,
        map: Map,
    ) -> Engine<'a> {
        Engine {
            doodads: doodads,
            player: player,
            center: &center,
            map: map,
        }
    }

    pub fn update_player_pos(&self, input: i32) {
        let mut p = self.player.borrow_mut();
        let mut new_y = p.y;
        let mut new_x = p.x;
        match input {
            EL_DOWN  => new_y += 1,
            EL_UP    => new_y -= 1,
            EL_LEFT  => new_x -= 1,
            EL_RIGHT => new_x += 1,
            _ => (),
        }

        if new_y >= 0 && new_y <= 20 && new_x >= 0 && new_x <= 20 {
            p.y = new_y;
            p.x = new_x;
        }
    }

    pub fn draw_tiles(&self) {
        let mut y = 0;
        let mut x = 0;
        for row in &self.map {
            for tile in row {
                if tile.block_sight {
                    mvprintw(y, x, "#");
                } else {
                    mvprintw(y, x, "-");
                }
                x += 1;
            }
            y += 1;
            x = 0;
        }
        refresh();
    }

    pub fn draw(&self) {
        for d_cell in self.doodads.iter() {
            let d = d_cell.borrow();
            let rep: DrawRep = d.render();
            mvprintw(rep.y, rep.x, &rep.c.to_string());
        }

        refresh();
    }
}
