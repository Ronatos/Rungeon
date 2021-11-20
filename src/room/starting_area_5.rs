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

/// Starting Area 5
/// 
/// Base Shape
/// # # # # # # # # # # # #
/// # # # # # # # # # # # #
/// # #                 # #
/// # #                 # #
/// # #                 # #
/// # #                 # #
/// # # # # # # # # # # # #
/// # # # # # # # # # # # #
/// 
/// OR
/// 
/// # # # # # # # #
/// # # # # # # # #
/// # #         # #
/// # #         # #
/// # #         # #
/// # #         # #
/// # #         # #
/// # #         # #
/// # #         # #
/// # #         # #
/// # # # # # # # #
/// # # # # # # # #
/// 
/// 4 passages will need to be added on the each wall
///
/// # # # # #     # # # # #
/// # # # # #     # # # # #
/// # #                 
///                     # #
///                     # #
/// # #                 # #
/// # # # # # # # # #   # #
/// # # # # # # # # #   # #
/// 
/// OR
/// 
/// # #   # # # # #
/// # #   # # # # #
/// # #         # #
/// # #         # #
/// # #            
/// # #            
/// # #         # #
///             # #
///             # #
/// # #         # #
/// # # # # #   # #
/// # # # # #   # #
pub fn new() -> Grid {
    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });

    if dice::roll(2) == 1 {
        // Let's build a horizontal starting area 5
        let mut starting_area5 = Grid::new(12, vec![
            wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone()
        ]);

        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::North, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::North, 10);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::South, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::South, 10);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::East, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::East, 10);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::West, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::West, 10);
        }
        starting_area5
    }
    else {
        // Let's build a vertical starting area 4
        let mut starting_area5 = Grid::new(8, vec![
            wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        ]);

        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::North, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::North, 10);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::South, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::South, 10);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::East, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::East, 10);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::West, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area5 = place_passage(starting_area5, Wall::West, 10);
        }
        starting_area5
    }
}
