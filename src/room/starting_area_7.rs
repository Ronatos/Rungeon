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

/// Starting Area 7
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
/// 1 well (w) will need to be added to a tile in the center of the room
///
/// # # # #   # # #
/// # # # #   # # #
/// # # #     # # #
///       w  
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
    let well = Node::Tile(Tile {
        kind: TileKind::Well,
        icon: TileIcon::Well
    });

    let mut starting_area7 = Grid::new(8, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), floor.clone(),floor.clone(),wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), floor.clone(),floor.clone(),wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
    ]);

    // Randomly place a 5x5ft well in the center of the room
    let d4 = dice::roll(4);
    if d4 == 1 {
        starting_area7.nodes[27] = well.clone();
    }
    else if d4 == 2 {
        starting_area7.nodes[28] = well.clone();
    }
    else if d4 == 3 {
        starting_area7.nodes[35] = well.clone();
    }
    else {
        starting_area7.nodes[36] = well.clone();
    }

    // Possibility of 5ft passage removed due to creation of impassable passages
    // 10ft passages do just fine on their own
    starting_area7 = place_passage(starting_area7, Wall::North, 10);
    starting_area7 = place_passage(starting_area7, Wall::South, 10);
    starting_area7 = place_passage(starting_area7, Wall::East, 10);
    starting_area7 = place_passage(starting_area7, Wall::West, 10);
    starting_area7
}
