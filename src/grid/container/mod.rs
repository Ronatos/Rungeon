use crate::grid;
use std::fmt;

#[derive(Clone)]
pub struct Container {
    pub grid: grid::Grid
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}
