use crate::dice;

use crate::grid;
use grid::Grid as Grid;
use grid::Node as GridNode;
use grid::tile::Tile as Tile;
use grid::tile::TileIcon as TileIcon;
use grid::tile::TileKind as TileKind;

use rand::Rng;

/// Starting Area 2
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
/// 1 passage will need to be added randomly: 1 random wall
/// Each passage can either be 5ft or 10ft wide,
/// and extends 10ft from the room.
/// 
/// 2 doors will need to be added randomly: 2 random walls not already occupied by a passage
/// d for 'door'
///
/// # #     # # # #
/// # #     # # # #
/// # #         # #
/// # #         d       
/// # #         # #
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

    let mut starting_area2 = Grid::new(8, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone()
    ]);

    // Repeat exactly 3 times.
    // 1. Select a wall from the remaining empty walls at random.
    // 2. Determine if a passage or door is to be built
    // 2a. Build a passage
    // 2b. Build a door
    // 3. Remove whichever wall is selected from the list of available walls.

    let mut exits_to_build = 3;
    let mut num_doors = 0;
    let mut num_passages = 0;
    let mut empty_walls = vec![Wall::North, Wall::South, Wall::East, Wall::West];
    let mut rng = rand::thread_rng();
    while exits_to_build > 0 {
        let wall_number = rng.gen_range(1..empty_walls + 1);
        

        exits_to_build = exits_to_build - 1;
    }

    starting_area2

    // let mut occupied_wall_list: Vec<Wall> = Vec::new();
    // let mut passages_placed = 0;
    // let doors_placed = 0;
    // loop {
    //     let roll = dice::roll(4);
    //     if roll == 1 && !vec_contains_wall(occupied_wall_list, Wall::North) {
    //         // We can put something on the north wall

    //         if passages_placed == 0 {
    //             // We want to place a passage
    //             if dice::roll(12) <= 2 { // This will be a 5ft wide passage
    //                 starting_area2 = generate_5ft_passage_ns(starting_area2, 0, 1);
    //             }
    //             else { // This will be a 10ft wide passage
    //                 starting_area2 = generate_10ft_passage_ns(starting_area2, 0, 1);
    //             }
    //             passages_placed = passages_placed + 1;
    //             occupied_wall_list.push(Wall::North);
    //         }
    //         else if doors_placed < 2 {
    //             // We want to place a door
    //         }
    //         else {
    //             // We've placed everything we can. Time to end.
    //             break;
    //         }
    //     }
    //     else if roll == 2 && !vec_contains_wall(occupied_wall_list, Wall::South) {
    //         // We can put something on the south wall

    //         if passages_placed == 0 {
    //             // We want to place a passage
    //             if dice::roll(12) <= 2 { // This will be a 5ft wide passage
    //                 starting_area2 = generate_5ft_passage_ns(starting_area2, 6, 7);
    //             }
    //             else { // This will be a 10ft wide passage
    //                 starting_area2 = generate_10ft_passage_ns(starting_area2, 6, 7);
    //             }
    //             passages_placed = passages_placed + 1;
    //             occupied_wall_list.push(Wall::South);
    //         }
    //         else if doors_placed < 2 {
    //             // We want to place a door
    //         }
    //         else {
    //             // We've placed everything we can. Time to end.
    //             break;
    //         }
    //     }
    //     else if roll == 3 && !vec_contains_wall(occupied_wall_list, Wall::East) {
    //         // We can put something on the east wall

    //         if passages_placed == 0 {
    //             // We want to place a passage
    //             if dice::roll(12) <= 2 { // This will be a 5ft wide passage
    //                 starting_area2 = generate_5ft_passage_ew(starting_area2, 6, 7);
    //             }
    //             else { // This will be a 10ft wide passage
    //                 starting_area2 = generate_10ft_passage_ew(starting_area2, 6, 7);
    //             }
    //             passages_placed = passages_placed + 1;
    //             occupied_wall_list.push(Wall::East);
    //         }
    //         else if doors_placed < 2 {
    //             // We want to place a door
    //         }
    //         else {
    //             // We've placed everything we can. Time to end.
    //             break;
    //         }
    //     }
    //     else if roll == 2 && !vec_contains_wall(occupied_wall_list, Wall::West) {
    //         // We can put something on the west wall

    //         if passages_placed == 0 {
    //             // We want to place a passage
    //             if dice::roll(12) <= 2 { // This will be a 5ft wide passage
    //                 starting_area2 = generate_5ft_passage_ew(starting_area2, 0, 1);
    //             }
    //             else { // This will be a 10ft wide passage
    //                 starting_area2 = generate_10ft_passage_ew(starting_area2, 0, 1);
    //             }
    //             passages_placed = passages_placed + 1;
    //             occupied_wall_list.push(Wall::West);
    //         }
    //         else if doors_placed < 2 {
    //             // We want to place a door
    //         }
    //         else {
    //             // We've placed everything we can. Time to end.
    //             break;
    //         }
    //     }
    // }

    // starting_area2
}

enum Wall {
    North,
    South,
    East,
    West
}

fn vec_contains_wall(vector: Vec<Wall>, target_wall: Wall) -> bool {
    for wall in 0..vector.len() {
        if matches!(wall, target_wall) {
            return true
        }
    }
    false
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