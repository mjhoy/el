use ncurses::*;

pub struct Window {
    win: *mut i8,
}

impl Drop for Window {
    fn drop(&mut self) {
        delwin(self.win);
    }
}

impl Window {
    pub fn new(
        height: i32,
        width: i32,
        start_y: i32,
        start_x: i32,
    ) -> Window {
        let window = Window {
            win: newwin(height, width, start_y, start_x)
        };
        window.bordered();
        window
    }

    fn bordered(&self) {
        box_(self.win, 0, 0);
    }

    pub fn refresh(&self) {
        wrefresh(self.win);
    }

    pub fn msg(&self, m: &str) {
        mvwprintw(self.win, 0, 0, m);
    }
}
