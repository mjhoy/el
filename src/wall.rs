use doodad::*;

pub struct Wall;

impl Doodad for Wall {
    fn draw(&self) -> DrawInstructions {
        DrawInstructions {
            c: '#',
            fg: Color::Green,
            bg: Color::Black
        }
    }

    fn passable(&self) -> bool { false }
    fn seethrough(&self) -> bool { false }
}

