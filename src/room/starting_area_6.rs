use crate::room::construction;
use construction::place_passage as place_passage;
use construction::Wall as Wall;

use crate::grid;
use grid::Grid as Grid;
use grid::Node as Node;
use grid::tile::Tile as Tile;
use grid::tile::TileIcon as TileIcon;
use grid::tile::TileKind as TileKind;

/// Starting Area 6
/// 
/// Base Shape
/// # # # # # # # #
/// # # # # # # # #
/// # # #     # # #
/// # #         # #
/// # #         # #
/// # # #     # # #
/// # # # # # # # #
/// # # # # # # # #
/// 
/// 1 passage will need to be added on each wall
///
/// # # # #   # # #
/// # # # #   # # #
/// # # #     # # #
///          
/// # #         
/// # # #     # # #
/// # # #     # # #
/// # # #     # # #
pub fn new() -> Grid {
    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });

    let mut starting_area6 = Grid::new(8, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), floor.clone(),floor.clone(),wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), floor.clone(),floor.clone(),wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
    ]);

    // Possibility of 5ft passage removed due to creation of impassable passages
    // 10ft passages do just fine on their own
    starting_area6 = place_passage(starting_area6, Wall::North, 10);
    starting_area6 = place_passage(starting_area6, Wall::South, 10);
    starting_area6 = place_passage(starting_area6, Wall::East, 10);
    starting_area6 = place_passage(starting_area6, Wall::West, 10);
    starting_area6
}
