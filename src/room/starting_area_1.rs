use crate::room::construction;
use construction::place_passage as place_passage;
use construction::Wall as Wall;

use crate::dice;

use crate::grid;
use grid::Grid as Grid;
use grid::Node as Node;
use grid::tile::Tile as Tile;
use grid::tile::TileIcon as TileIcon;
use grid::tile::TileKind as TileKind;

/// Starting Area 1
/// 
/// Base Shape
/// # # # # # # # #
/// # # # # # # # #
/// # #         # #
/// # #         # #
/// # #         # #
/// # #         # #
/// # # # # # # # #
/// # # # # # # # #
/// 
/// Four passages will need to be added randomly: 1 per wall
/// Each passage can either be 5ft or 10ft wide,
/// and extends 10ft from the room.
///
/// # #     # # # #
/// # #     # # # #
/// # #         # #
///                    
///             # #
/// # #         # #
/// # # #   # # # #
/// # # #   # # # #
pub fn new() -> Grid {
    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });

    let mut starting_area1 = Grid::new(8, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone()
    ]);

    if dice::roll(12) <= 2 {
        starting_area1 = place_passage(starting_area1, Wall::North, 5);
    }
    else {
        starting_area1 = place_passage(starting_area1, Wall::North, 10);
    }

    if dice::roll(12) <= 2 {
        starting_area1 = place_passage(starting_area1, Wall::South, 5);
    }
    else {
        starting_area1 = place_passage(starting_area1, Wall::South, 10);
    }

    if dice::roll(12) <= 2 {
        starting_area1 = place_passage(starting_area1, Wall::East, 5);
    }
    else {
        starting_area1 = place_passage(starting_area1, Wall::East, 10);
    }

    if dice::roll(12) <= 2 {
        starting_area1 = place_passage(starting_area1, Wall::West, 5);
    }
    else {
        starting_area1 = place_passage(starting_area1, Wall::West, 10);
    }

    starting_area1
}