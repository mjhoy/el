pub struct DrawRep {
    pub c: char,
    pub x: i32,
    pub y: i32,
}

pub fn c(_c: char, _x: i32, _y: i32) -> DrawRep {
    DrawRep { c: _c, x: _x, y: _y }
}

pub trait Doodad {
    fn render(&self) -> DrawRep;
}
