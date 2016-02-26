extern crate ncurses;

use std::cell::*;
use std::rc::Rc;

use ncurses::*;
use wall::*;
use door::*;
use doodad::*;
use render::*;


pub type Event = fn(&mut Map, usize) -> bool;

#[allow(dead_code)]
pub struct Map {
    pub data    : Rc<RefCell<Vec<Option<Box<Doodad>>>>>,
    pub visible : Vec<char>,
    pub w       : usize,
    pub h       : usize,
    pub pos     : usize,
    events      : Vec<Event>
}

pub fn map_from_str(s: &'static str) -> Map {
    let mut longest = 0;
    for line in s.lines() {
        if line.len() > longest {
            longest = line.len();
        }
    }
    let mut data_list: Vec<String> = vec![];

    for line in s.lines() {
        let line_length = line.len();
        let to_pad = longest - line_length;
        let mut s = String::new();
        s.push_str(line);
        for _ in 0..(to_pad) { s.push(' '); }
        data_list.push(s);
    }

    let width = longest;
    let height = s.lines().count();
    let data = data_list.join("");

    Map::new(data, width, height, vec![])
}

fn can_see_through(c: &Option<Box<Doodad>>) -> bool {
    match c {
        &Some(ref x) => { x.seethrough() }
        &None => true
    }
}

fn check_and_mark_neighbor(vis : &mut Vec<char>,
                           coord : usize) -> bool
{
    if coord < vis.len() && vis[coord] == ' ' {
        vis[coord] = 'X';
        true
    } else {
        false
    }
}

fn char_to_doodad(c: char) -> Option<Box<Doodad>> {
    match c {
        '#' => Some(Box::new(Wall)),
        '|' => Some(Box::new(Door { horizontal: false, open: false })),
        '-' => Some(Box::new(Door { horizontal: true, open: false })),
        _   => None
    }
}

impl Map {
    pub fn new(data: String, width: usize, height: usize, events: Vec<Event>) -> Map {
        let mut initial_pos = 0;
        let mut d: Vec<Option<Box<Doodad>>> = vec![];
        for (pos, c) in data.chars().enumerate() {
            if c == 'L' {
                initial_pos = pos;
                d.push(char_to_doodad(' '));
            } else {
                d.push(char_to_doodad(c));
            }
        }

        let mut m1 = Map {
            data: Rc::new(RefCell::new(d)),
            visible: vec![' '; width * height],
            w: width,
            h: height,
            pos: initial_pos,
            events: events
        };

        // update the initial visibility
        m1.update_visibility();
        m1
    }

    pub fn render(&self) {
        let w = self.w;
        let h = self.h;

        clear();

        for i in 0..h {
            for j in 0..w {
                let cur_coord = (w * i) + j;
                if cur_coord == self.pos {
                    printw("L");
                } else {
                    let ref cur_dood = self.data.borrow()[cur_coord];
                    match cur_dood {
                        &Some(ref x) => {
                            let instr = x.draw();
                            if self.visible[cur_coord] == 'X' {
                                render(instr);
                            } else {
                                render_empty();
                            }
                        }
                        &None => render_empty()
                    }
                }
            }
            printw("\n");
        }
        refresh();
    }

    pub fn move_pos(&mut self, ch: char) -> bool {
        let new_pos = match ch {
            'h' => self.pos - 1,
            'l' => self.pos + 1,
            'j' => self.pos + self.w,
            'k' => self.pos - self.w,
            _   => return false
        };

        if new_pos >= (self.w * self.h) {
            return false;
        }

        let res = match self.data.borrow_mut()[new_pos] {
            Some(ref mut x) => {
                let passable = x.passable();
                x.move_action();
                passable
            }
            // '#' | '|' => false, // can't walk through walls
            // '0' |
            // '1' |
            // '2' |
            // '3' |
            // '4' => {
            //     let idx : usize = self.data[new_pos].to_string().parse::<usize>().unwrap();
            //     let event = self.events[idx];
            //     event(self, new_pos);
            //     true
            // }
            _ => {
                true
            }
        };
        if res {
            self.pos = new_pos;
        }
        self.update_visibility();
        res
    }

    fn update_visibility(&mut self) {
        let mut set_new : bool = true;
        let w = self.w;
        let h = self.h;

        let mut vis = self.visible.clone();
        vis[self.pos] = 'X';

        while set_new {
            set_new = false;
            for i in 0..h {
                for j in 0..w {
                    let cur_coord = (w * i) + j;
                    let cur_char  = vis[cur_coord];
                    if cur_char == 'X' && can_see_through(&self.data.borrow()[cur_coord]) {

                        let mut neighbor;
                        let mut did_mark = false;
                        // north
                        if cur_coord > w {
                            neighbor = cur_coord - w;
                            did_mark = check_and_mark_neighbor(&mut vis, neighbor);
                        }

                        // south
                        neighbor = cur_coord + w;
                        did_mark = check_and_mark_neighbor(&mut vis, neighbor) || did_mark;

                        // east
                        if j != w {
                            neighbor = cur_coord + 1;
                            did_mark = check_and_mark_neighbor(&mut vis, neighbor) || did_mark;
                        }

                        // west
                        if j != 0 {
                            neighbor = cur_coord - 1;
                            did_mark = check_and_mark_neighbor(&mut vis, neighbor) || did_mark;
                        }

                        set_new = set_new || did_mark;
                    }
                }
            }
        }
        self.visible = vis;
    }
}


// -------- private tests ----------------

#[test]
fn test_check_and_mark_neighbor() {
    let mut vis = vec![' ', ' ', ' '];

    assert_eq!(check_and_mark_neighbor(&mut vis, 3), false);
    assert_eq!(vec![' ', ' ', ' '], vis);

    assert_eq!(check_and_mark_neighbor(&mut vis, 0), true);
    assert_eq!(vec!['X', ' ', ' '], vis);

    assert_eq!(check_and_mark_neighbor(&mut vis, 0), false);
    assert_eq!(vec!['X', ' ', ' '], vis);
}
