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
    pub icon: TileIcon
}

/// The Display function tells std::fmt how to display a tile on the screen.
/// Tiles must be "displayable" because the Grid Display method relies on displaying Modules,
/// which are in turn eventually made of tiles at the most basic level. Tile icons are displayed.
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.icon)
    }
}

/// An enum specifying what kind of Tile is being referenced. This is considered the true
/// nature of the tile, despite what it may appear to be.
/// 
/// # Variants
/// 
/// * `Floor` - This is used to specify that a tile is a floor tile.
/// * `Wall` - This is used to specify that a tile is a wall tile.
pub enum TileKind {
    Floor,
    Wall
}

/// An enum specifying the type of icon that should be displayed when a tile is displayed.
/// This is what is displayed on the screen, despite what it may actually be.
/// 
/// # Variants
/// 
/// * `Floor` - An empty space. " "
/// * `Wall` - A number sign. "#"
pub enum TileIcon {
    Floor,
    Wall
}

/// The Display function tells std::fmt how to display a tile on the screen.
/// This is where the TileIcon variant is decoded to reveal what the icon actually looks like.
/// The recursive Display tree always terminates branches at this function.
impl fmt::Display for TileIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileIcon::Wall => write!(f, "{} ", "#"),
            TileIcon::Floor => write!(f, "{} ", " ")
        }
    }
}
