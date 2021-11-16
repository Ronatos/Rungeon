pub mod starting_area_1;
pub mod starting_area_2;

// ------------------------------------------------------------------------------------------------

// pub mod tile;
// mod passage;
// mod starting_area;

// use crate::grid;
// use rand::distributions::{Distribution, Standard};
// use rand::Rng;
// use std::fmt;

// pub struct Room {
//     pub grid: grid::Grid,
//     pub kind: RoomKind,
//     pub exits: Vec<Exit>,
//     pub longest_dimension: u8
// }

// impl fmt::Display for Room {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.grid)
//     }
// }

// /// RoomKind is a necessary enum because each kind of room requires different properties.
// /// 
// /// # Variants
// /// 
// /// * `Chamber` -
// /// * `StartingArea` - 
// /// * `Passage` -
// enum RoomKind {
//     Chamber,
//     Passage,
//     StartingArea
// }

// pub struct Exit {
//     pub leads_to: RoomKind,
//     pub location: ExitLocation
// }

// pub enum ExitLocation {
//     Bottom,
//     Left,
//     Right,
//     Top
// }

// impl Distribution<ExitLocation> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExitLocation {
//         match rng.gen_range(0..4) {
//             0 => ExitLocation::Top,
//             1 => ExitLocation::Bottom,
//             2 => ExitLocation::Left,
//             3 => ExitLocation::Right,
//             _ => ExitLocation::Top
//         }
//     }
// }
