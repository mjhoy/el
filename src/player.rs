use doodad::*;

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Doodad for Player {
    fn render(&self) -> DrawRep {
        return c('@', self.x, self.y);
    }
}
