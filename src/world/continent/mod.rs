use crate::grid;
use std::fmt;

pub struct World {
    grid: grid::Grid
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}