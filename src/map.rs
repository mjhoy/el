#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub explored: bool,
    pub block_sight: bool
}

impl Tile {
    pub fn open() -> Tile {
        Tile {
            blocked: false,
            explored: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Tile {
        Tile {
            blocked: true,
            explored: false,
            block_sight: true,
        }
    }
}

pub type Map = Vec<Vec<Tile>>;
