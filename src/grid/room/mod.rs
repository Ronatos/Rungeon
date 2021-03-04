mod passage;
mod starting_area;

use crate::grid;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;

pub struct Room {
    pub exits: Vec<Exit>,
    pub grid: grid::Grid,
    pub kind: RoomKind,
    pub longest_dimension: u8
}

pub struct Exit {
    pub leads_to: RoomKind,
    pub location: ExitLocation
}

pub enum RoomKind {
    Chamber,
    Passage,
    StartingArea
}

pub enum ExitLocation {
    Bottom,
    Left,
    Right,
    Top
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
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
