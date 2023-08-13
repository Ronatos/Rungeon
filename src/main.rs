mod dice;
mod grid;
use grid::Grid as Grid;
use grid::map as Map;

fn main() {
    let map: Grid = Map::new(1, 1, 0);
    println!("{}", map);
}