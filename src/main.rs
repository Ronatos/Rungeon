mod dice;
mod grid;
mod map;
mod tile;

use crate::grid::Grid as Grid;
use crate::map as Map;

fn main() {
    let map: Grid = Map::new(1, 1, 0);
    println!("{}", map);
}