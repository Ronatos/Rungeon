use crate::dice;
use crate::grid::Grid as Grid;
use crate::grid::Node as Node;
use crate::grid::room::place_passage as place_passage;
use crate::grid::room::Wall as Wall;
use crate::grid::tile::Tile as Tile;
use crate::grid::tile::TileIcon as TileIcon;
use crate::grid::tile::TileKind as TileKind;

// https://github.com/Ronatos/rungeon/wiki/Room#starting-area-7
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
