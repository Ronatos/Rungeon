use crate::dice;
use crate::grid::Grid as Grid;
use crate::grid::Node as Node;
use crate::map::room::place_door as place_door;
use crate::map::room::place_passage as place_passage;
use crate::map::room::Room as Room;
use crate::map::room::Wall as Wall;
use crate::tile::Tile as Tile;
use crate::tile::TileIcon as TileIcon;
use crate::tile::TileKind as TileKind;

// https://github.com/Ronatos/rungeon/wiki/Room#starting-area-4
pub fn new() -> Room {
    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });
    let exits = vec![Wall::North, Wall::South, Wall::East, Wall::West];

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
        Room::new(starting_area4, exits)
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
        Room::new(starting_area4, exits)
    }
}