use std::fmt;

/// A Tile is defined as a static part of the environment that
/// cannot be relocated, and has the ability to exist
/// "underneath" interactable objects such as a table,
/// pile of gold, or a goblin. The floors and walls are
/// examples of tiles. The tile's icon is printed if nothing
/// is occupying it.
/// 
/// # Fields
/// 
/// * `kind: TileKind` - The tile's true identity, despite what it looks like.
/// The TileKind field is an enum because there are a limited number of tile types,
/// and to allow performant comparisons.
/// * `icon: Icon` - What is to be displayed when the tile is unoccupied.
/// The Icon field is an enum because there are a limited number of icons that may
/// be displayed, and they should be consistent in their meaning. The icon is not
/// dependent upon the `kind` field, however. This is to allow cases such as secret
/// doors.
/// 
/// # Examples
/// 
/// ### Example 1
/// 
/// ```
/// Tile {
///     kind: TileKind::Wall,
///     icon: Icon::Wall
/// }
/// ```
/// ### Output
/// 
/// \#
pub struct Tile {
    pub kind: TileKind,
    pub icon: Icon
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.icon)
    }
}

pub enum TileKind {
    Floor,
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
