/// I think the right way to start talking about Rooms is to consider what kinds of properties they should have.
/// What is a room the way I have defined it? A room is akin to a Tile. It is displayed no differently by the grid.
/// However, it is slightly different in its Display method.
/// Where a Tile's icon is displayed, ending the recursive branch, a Room carries on the recursion exactly one more time beyond itself.
/// This is because Rooms are made of Tiles. I wonder, are tiles ever not part of a room? The answer to that is yes if an only if there are other
/// types of containers which would have tiles other than a room. Would I consider a section of the wilderness a room?
/// That doesn't really make a whole lot of sense, but I suppose it may depending on my implementation of the way the player moves around.
/// 
/// This leads to an interesting series of questions. Do I prefer the idea of having the camera follow and center on the player,
/// displaying the transforming landscape around them as they travel? Or would I prefer to display fixed size areas for the player to traverse
/// one at a time?
/// The answer to that isn't so simple. I like both, but it comes down to how these things are displayed to the screen.
/// I wonder if it would be possible to clear the screen every time the environment is drawn?
/// Let's say it was possible (because it probably is somehow). Do I like that artistic style more?
/// This is meant to be modeled after dungeons and dragons, so perhaps it's best to take a hint from that.
/// That system would make it obvious that individual rooms should be the only things displayed.
/// ADDITIONALLY, the game board should be wiped every time it's updated.
/// We would also probably want a minimap, but that could be handled later.
/// 
/// Okay, so what did we learn? We learned that we want to display a room of tiles to the player at any given time.
/// This could mean a wilderness as well. Perhaps wildernesses are just much bigger rooms,
/// or perhaps we could separate wildernesses up in to several rooms.
/// Wildernesses aside, this means that displaying a grid isn't necessarily as easy as we've already implemented it.
/// Not all grids should be displayed all the time. Only rooms should be displayed.
/// 
/// Grids should absolutely exist, but Rooms should be the only things that are displayed at any given moment.
/// 
/// We've made a great leap today, but the question remains - what properties make up a room?
/// A room is quite complex, and this could lead down a difficult rabbit hole.
/// 

pub mod tile;

pub struct Room {
    pub grid: grid::Grid,
    pub kind: RoomKind,
    pub exits: Vec<Exit>,
    pub longest_dimension: u8
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

/// RoomKind is a necessary enum because each kind of room requires different properties.
/// 
/// # Variants
/// 
/// * `Chamber` -
/// * `StartingArea` - 
/// * `Passage` -
enum RoomKind {
    Chamber,
    Passage,
    StartingArea
}

// ------------------------------------------------------------------------------------------------

mod passage;
mod starting_area;

use crate::grid;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;

pub struct Exit {
    pub leads_to: RoomKind,
    pub location: ExitLocation
}

pub enum ExitLocation {
    Bottom,
    Left,
    Right,
    Top
}

impl Distribution<ExitLocation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExitLocation {
        match rng.gen_range(0..4) {
            0 => ExitLocation::Top,
            1 => ExitLocation::Bottom,
            2 => ExitLocation::Left,
            3 => ExitLocation::Right,
            _ => ExitLocation::Top
        }
    }
}
