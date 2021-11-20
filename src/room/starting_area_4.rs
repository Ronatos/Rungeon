use crate::room::construction;
use construction::place_door as place_door;
use construction::place_passage as place_passage;
use construction::Wall as Wall;

use crate::dice;

use crate::grid;
use grid::Grid as Grid;
use grid::Node as Node;
use grid::tile::Tile as Tile;
use grid::tile::TileIcon as TileIcon;
use grid::tile::TileKind as TileKind;

/// Starting Area 4
/// 
/// Base Shape
/// # # # # # # # # # # # # # # # # # # # #
/// # # # # # # # # # # # # # # # # # # # #
/// # #                                 # #
/// # #   # #   # #   # #   # #   # #   # #
/// # #   # #   # #   # #   # #   # #   # #
/// # #                                 # #
/// # # # # # # # # # # # # # # # # # # # #
/// # # # # # # # # # # # # # # # # # # # #
/// 
/// OR
/// 
/// # # # # # # # #
/// # # # # # # # #
/// # #         # #
/// # #   # #   # #
/// # #   # #   # #
/// # #         # #
/// # #   # #   # #
/// # #   # #   # #
/// # #         # #
/// # #   # #   # #
/// # #   # #   # #
/// # #         # #
/// # #   # #   # #
/// # #   # #   # #
/// # #         # #
/// # #   # #   # #
/// # #   # #   # #
/// # #         # #
/// # # # # # # # #
/// # # # # # # # #
/// 
/// 2 passages will need to be added on the long walls,
/// and 2 doors will need to be added on the short walls
///
/// # # # # # # # # # # # # # # # #     # #
/// # # # # # # # # # # # # # # # #     # #
/// # #                                 # #
///   d   # #   # #   # #   # #   # #   # #
/// # #   # #   # #   # #   # #   # #   # #
/// # #                                 d  
/// # # # # # # # # #   # # # # # # # # # #
/// # # # # # # # # #   # # # # # # # # # #
/// 
/// OR
/// 
/// # # # #   # # #
/// # # # # d # # #
/// # #         # #
/// # #   # #      
/// # #   # #      
/// # #         # #
/// # #   # #   # #
///       # #   # #
/// # #         # #
/// # #   # #   # #
/// # #   # #   # #
/// # #         # #
/// # #   # #   # #
/// # #   # #   # #
/// # #         # #
/// # #   # #   # #
/// # #   # #   # #
/// # #         # #
/// # # # d # # # #
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

    if dice::roll(2) == 1 {
        // Let's build a horizontal starting area 4
        let mut starting_area4 = Grid::new(20, vec![
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone()
        ]);

        starting_area4 = place_door(starting_area4, Wall::East);
        starting_area4 = place_door(starting_area4, Wall::West);
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::North, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::North, 10);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::South, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::South, 10);
        }
        starting_area4
    }
    else {
        // Let's build a vertical starting area 4
        let mut starting_area4 = Grid::new(8, vec![
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone()
        ]);

        starting_area4 = place_door(starting_area4, Wall::North);
        starting_area4 = place_door(starting_area4, Wall::South);
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::East, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::East, 10);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::West, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::West, 10);
        }
        starting_area4
    }
}