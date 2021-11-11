use crate::grid;
use std::fmt;

/// World
/// 
/// The real challenge is that each of these grids can
/// have as many nodes as I want them to. If the world
/// has the largest scale per node, this is at the
/// continent scale. 
/// 
/// 01 02 03 | 10 11 12 | 19 20 21
/// 04 05 06 | 13 14 15 | 22 23 24
/// 07 08 09 | 16 17 18 | 25 26 27
/// ---------+----------+---------
/// 28 29 30 | 37 38 39 | 46 47 48
/// 31 32 33 | 40 41 42 | 49 50 51
/// 34 35 36 | 43 44 45 | 52 53 54
/// ---------+----------+---------
/// 55 56 57 | 64 65 66 | 73 74 75
/// 58 59 60 | 67 68 69 | 76 77 78
/// 61 62 63 | 70 71 72 | 79 80 81

pub struct World {
    grid: grid::Grid
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}