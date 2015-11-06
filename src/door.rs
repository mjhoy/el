use doodad::*;

pub enum Door {
    Vertical,
    Horizontal
}

impl Doodad for Door {
    fn draw(&self) -> DrawInstructions {
        let c = match *self {
            Door::Vertical   => '|',
            Door::Horizontal => '-'
        };
        DrawInstructions {
            c:  c,
            fg: Color::White,
            bg: Color::Black
        }
    }

    fn passable(&self) -> bool { false }
    fn seethrough(&self) -> bool { false }
}
