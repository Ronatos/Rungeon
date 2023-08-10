use std::fmt;

// https://github.com/Ronatos/rungeon/wiki/Grid#gridtiletile
#[derive(Copy, Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub icon: TileIcon
}

// https://github.com/Ronatos/rungeon/wiki/Grid#gridtiletiledisplay
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.icon)
    }
}

// https://github.com/Ronatos/rungeon/wiki/Grid#gridtiletiletilekind
#[derive(Copy, Clone)]
pub enum TileKind {
    Door,
    Floor,
    Wall,
    Well
}

// https://github.com/Ronatos/rungeon/wiki/Grid#gridtiletiletileicon
#[derive(Copy, Clone)]
pub enum TileIcon {
    Door,
    Floor,
    Wall,
    Well
}

// https://github.com/Ronatos/rungeon/wiki/Grid#gridtiletiletileicondisplay
impl fmt::Display for TileIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileIcon::Door => write!(f, "{} ", "d"),
            TileIcon::Floor => write!(f, "{} ", " "),
            TileIcon::Wall => write!(f, "{} ", "#"),
            TileIcon::Well => write!(f, "{} ", "w"),
        }
    }
}
