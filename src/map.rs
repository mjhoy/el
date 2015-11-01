use ncurses::*;

pub type Event = fn(&mut Map, usize) -> bool;

pub struct Map {
    pub data    : Vec<char>,
    pub visible : Vec<char>,
    pub w       : usize,
    pub h       : usize,
    pub pos     : usize,
    events      : Vec<Event>
}

fn can_see_through(c: char) -> bool {
    c != '|' && c != '#'
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

impl Map {
    pub fn new(data: String, width: usize, height: usize, events: Vec<Event>) -> Map {
        let mut initial_pos = 0;
        let mut d: Vec<char> = vec![' '; width * height];
        for (pos, c) in data.chars().enumerate() {
            if c == 'L' {
                initial_pos = pos;
                d[pos] = ' ';
            } else {
                d[pos] = c;
            }
        }

        let mut m1 = Map {
            data: d.clone(),
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

        for i in (0..h) {
            for j in (0..w) {
                let cur_coord = (w * i) + j;
                if cur_coord == self.pos {
                    printw("L");
                } else {
                    let cur_char = self.data[cur_coord];
                    let print_char = match cur_char {
                        '0' | '1' | '2' | '3' | '4' =>
                            ' ',
                        'L' => ' ',
                        x => x
                    };
                    if self.visible[cur_coord] == 'X' {
                        printw(&print_char.to_string());
                    } else {
                        printw(" ");
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

        match self.data[new_pos] {
            '#' | '|' => false, // can't walk through walls
            '0' |
            '1' |
            '2' |
            '3' |
            '4' => {
                let idx : usize = self.data[new_pos].to_string().parse::<usize>().unwrap();
                let event = self.events[idx];
                event(self, new_pos);
                true
            }
            _ => {
                self.pos = new_pos;
                self.update_visibility();
                true
            }
        }
    }

    fn update_visibility(&mut self) {
        let mut set_new : bool = true;
        let w = self.w;
        let h = self.h;

        let mut vis = self.visible.clone();
        vis[self.pos] = 'X';

        while set_new {
            set_new = false;
            for i in (0..h) {
                for j in (0..w) {
                    let cur_coord = (w * i) + j;
                    let cur_char  = vis[cur_coord];
                    if cur_char == 'X' && can_see_through(self.data[cur_coord]) {

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

