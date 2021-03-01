use std::fmt;

/// A Tile is defined as a 
struct Tile {
    pub kind: TileKind,
    pub icon: Icon
}

impl Tile {
    pub fn new()
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.icon)
    }
}

pub enum TileKind {
    Door,
    Floor,
    Stairs,
    Wall
}

pub enum Icon {
    Floor,
    Wall
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Icon::Wall => write!(f, "{} ", "#"),
            Icon::Floor => write!(f, "{} ", " ")
        }
    }
}
