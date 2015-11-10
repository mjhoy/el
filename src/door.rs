use doodad::*;

pub struct Door {
    pub horizontal: bool,
    pub open: bool
}

impl Door {
    pub fn open(&mut self) -> () {
        self.open = true;
    }
}

impl Doodad for Door {
    fn draw(&self) -> DrawInstructions {
        let c = match (self.open, self.horizontal) {
            (false, true)  => '-',
            (false, false) => '|',
            (true, false)  => '`',
            (true, true)   => ','
        };
        DrawInstructions {
            c:  c,
            fg: Color::White,
            bg: Color::Black
        }
    }

    fn passable(&self) -> bool { self.open }
    fn seethrough(&self) -> bool { self.open }
    fn move_action(&mut self) {
        if !self.open {
            self.open();
        }
    }
}
