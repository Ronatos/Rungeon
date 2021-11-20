use crate::dice;

use crate::grid;
use grid::Grid as Grid;
use grid::Node as Node;
use grid::tile::Tile as Tile;
use grid::tile::TileIcon as TileIcon;
use grid::tile::TileKind as TileKind;

use rand::Rng;

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

        starting_area4 = place_door(starting_area4, Wall::East, 0, 1);
        starting_area4 = place_door(starting_area4, Wall::West, 18, 19);
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = generate_5ft_passage_ns(starting_area4, 0, 1);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = generate_10ft_passage_ns(starting_area4, 0, 1);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = generate_5ft_passage_ns(starting_area4, 6, 7);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = generate_10ft_passage_ns(starting_area4, 6, 7);
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

        starting_area4 = place_door(starting_area4, Wall::North, 0, 1);
        starting_area4 = place_door(starting_area4, Wall::South, 18, 19);
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = generate_5ft_passage_ew(starting_area4, 0, 1);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = generate_10ft_passage_ew(starting_area4, 0, 1);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = generate_5ft_passage_ew(starting_area4, 6, 7);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = generate_10ft_passage_ew(starting_area4, 6, 7);
        }
        starting_area4
    }
}

enum Wall {
    North,
    South,
    East,
    West
}

fn generate_5ft_passage_ns(mut starting_area: Grid, top_row: usize, bottom_row: usize) -> Grid {
    let mut rng = rand::thread_rng();
    let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

    let column = rng.gen_range(2..6);
    starting_area.nodes[column + (starting_area.columns * top_row)] = floor.clone();
    starting_area.nodes[column + (starting_area.columns * bottom_row)] = floor.clone();
    starting_area
}

fn generate_5ft_passage_ew(mut starting_area: Grid, left_column: usize, right_column: usize) -> Grid {
    let mut rng = rand::thread_rng();
    let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

    let row = rng.gen_range(2..6);
    starting_area.nodes[(row * starting_area.columns) + left_column] = floor.clone();
    starting_area.nodes[(row * starting_area.columns) + right_column] = floor.clone();
    starting_area
}

fn generate_10ft_passage_ns(mut starting_area: Grid, top_row: usize, bottom_row: usize) -> Grid {
    let mut rng = rand::thread_rng();
    let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

    let left_column = rng.gen_range(2..5);
    starting_area.nodes[left_column + (starting_area.columns * top_row)] = floor.clone();
    starting_area.nodes[left_column + (starting_area.columns * bottom_row)] = floor.clone();
    starting_area.nodes[(left_column + 1) + (starting_area.columns * top_row)] = floor.clone();
    starting_area.nodes[(left_column + 1) + (starting_area.columns * bottom_row)] = floor.clone();
    starting_area
}

fn generate_10ft_passage_ew(mut starting_area: Grid, left_column: usize, right_column: usize) -> Grid {
    let mut rng = rand::thread_rng();
    let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

    let top_row = rng.gen_range(2..5);
    starting_area.nodes[(top_row * starting_area.columns) + left_column] = floor.clone();
    starting_area.nodes[(top_row * starting_area.columns) + right_column] = floor.clone();
    starting_area.nodes[((top_row + 1) * starting_area.columns) + left_column] = floor.clone();
    starting_area.nodes[((top_row + 1) * starting_area.columns) + right_column] = floor.clone();
    starting_area
}

fn place_door(mut starting_area: Grid, wall: Wall, lower_coordinate: usize, higher_coordinate: usize) -> Grid {
    let mut rng = rand::thread_rng();
    let door = Node::Tile(Tile {kind: TileKind::Door, icon: TileIcon::Door});
    let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

    match wall {
        Wall::North => {
            let column = rng.gen_range(2..6);
            let top_row = lower_coordinate;
            let bottom_row = higher_coordinate;
            starting_area.nodes[column + (starting_area.columns * top_row)] = floor.clone();
            starting_area.nodes[column + (starting_area.columns * bottom_row)] = door.clone();
        },
        Wall::South => {
            let column = rng.gen_range(2..6);
            let top_row = lower_coordinate;
            let bottom_row = higher_coordinate;
            starting_area.nodes[column + (starting_area.columns * top_row)] = door.clone();
            starting_area.nodes[column + (starting_area.columns * bottom_row)] = floor.clone();
        },
        Wall::East => {
            let row = rng.gen_range(2..6);
            let left_column = lower_coordinate;
            let right_column = higher_coordinate;
            starting_area.nodes[(row * starting_area.columns) + left_column] = floor.clone();
            starting_area.nodes[(row * starting_area.columns) + right_column] = door.clone();
        },
        Wall::West => {
            let row = rng.gen_range(2..6);
            let left_column = lower_coordinate;
            let right_column = higher_coordinate;
            starting_area.nodes[(row * starting_area.columns) + left_column] = door.clone();
            starting_area.nodes[(row * starting_area.columns) + right_column] = floor.clone();
        }
    }
    starting_area
}