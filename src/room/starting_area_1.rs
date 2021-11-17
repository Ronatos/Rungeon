use crate::dice;

use crate::grid;
use grid::Grid as Grid;
use grid::Node as GridNode;
use grid::container::Container as Container;
use grid::tile::Tile as Tile;
use grid::tile::TileIcon as TileIcon;
use grid::tile::TileKind as TileKind;

use rand::Rng;

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
pub fn new() -> Container {
    let wall = GridNode::Tile(Tile {kind: TileKind::Wall, icon: TileIcon::Wall});
    let floor = GridNode::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

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

    // Top passage
    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
        starting_area1 = generate_5ft_passage_ns(starting_area1, 0, 1);
    }
    else { // This will be a 10ft wide passage
        starting_area1 = generate_10ft_passage_ns(starting_area1, 0, 1);
    }

    // Bottom passage
    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
        starting_area1 = generate_5ft_passage_ns(starting_area1, 6, 7);
    }
    else { // This will be a 10ft wide passage
        starting_area1 = generate_10ft_passage_ns(starting_area1, 6, 7);
    }

    // Left passage
    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
        starting_area1 = generate_5ft_passage_ew(starting_area1, 0, 1);
    }
    else { // This will be a 10ft wide passage
        starting_area1 = generate_10ft_passage_ew(starting_area1, 0, 1);
    }

    // Right passage
    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
        starting_area1 = generate_5ft_passage_ew(starting_area1, 6, 7);
    }
    else { // This will be a 10ft wide passage
        starting_area1 = generate_10ft_passage_ew(starting_area1, 6, 7);
    }

    Container {grid: starting_area1}
}

fn generate_5ft_passage_ns(mut starting_area: Grid, top_row: usize, bottom_row: usize) -> Grid {
    let mut rng = rand::thread_rng();
    let floor = GridNode::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

    let column = rng.gen_range(2..6);
    starting_area.nodes[column + (starting_area.columns * top_row)] = floor.clone();
    starting_area.nodes[column + (starting_area.columns * bottom_row)] = floor.clone();
    starting_area
}

fn generate_5ft_passage_ew(mut starting_area: Grid, left_column: usize, right_column: usize) -> Grid {
    let mut rng = rand::thread_rng();
    let floor = GridNode::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

    let row = rng.gen_range(2..6);
    starting_area.nodes[(row * starting_area.columns) + left_column] = floor.clone();
    starting_area.nodes[(row * starting_area.columns) + right_column] = floor.clone();
    starting_area
}

fn generate_10ft_passage_ns(mut starting_area: Grid, top_row: usize, bottom_row: usize) -> Grid {
    let mut rng = rand::thread_rng();
    let floor = GridNode::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

    let left_column = rng.gen_range(2..5);
    starting_area.nodes[left_column + (starting_area.columns * top_row)] = floor.clone();
    starting_area.nodes[left_column + (starting_area.columns * bottom_row)] = floor.clone();
    starting_area.nodes[(left_column + 1) + (starting_area.columns * top_row)] = floor.clone();
    starting_area.nodes[(left_column + 1) + (starting_area.columns * bottom_row)] = floor.clone();
    starting_area
}

fn generate_10ft_passage_ew(mut starting_area: Grid, left_column: usize, right_column: usize) -> Grid {
    let mut rng = rand::thread_rng();
    let floor = GridNode::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

    let top_row = rng.gen_range(2..5);
    starting_area.nodes[(top_row * starting_area.columns) + left_column] = floor.clone();
    starting_area.nodes[(top_row * starting_area.columns) + right_column] = floor.clone();
    starting_area.nodes[((top_row + 1) * starting_area.columns) + left_column] = floor.clone();
    starting_area.nodes[((top_row + 1) * starting_area.columns) + right_column] = floor.clone();
    starting_area
}