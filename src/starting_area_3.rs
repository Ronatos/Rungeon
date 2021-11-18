// use crate::dice;

// use crate::grid;
// use grid::Grid as Grid;
// use grid::Node as Node;
// use grid::tile::Tile as Tile;
// use grid::tile::TileIcon as TileIcon;
// use grid::tile::TileKind as TileKind;

// use rand::Rng;

// /// Starting Area 3
// /// 
// /// Base Shape
// /// # # # # # # # # # # # #
// /// # # # # # # # # # # # #
// /// # #                 # #
// /// # #                 # #
// /// # #                 # #
// /// # #                 # #
// /// # #                 # #
// /// # #                 # #
// /// # #                 # #
// /// # #                 # #
// /// # # # # # # # # # # # #
// /// # # # # # # # # # # # # 
// /// 
// /// 3 doors will need to be added randomly: 3 random walls not already occupied by a passage
// /// d for 'door'
// ///
// /// # # # # # #   # # # # #
// /// # # # # # # d # # # # #
// /// # #                 # #
// /// # #                 # #
// /// # #                 # #
// /// # #                 # #
// ///   d                 # #
// /// # #                 # #
// /// # #                 # #
// /// # #                 # #
// /// # # # # # # # # d # # #
// /// # # # # # # # #   # # # 
// pub fn new() -> Grid {
//     let wall = Node::Tile(Tile {
//         kind: TileKind::Wall,
//         icon: TileIcon::Wall
//     });
//     let floor = Node::Tile(Tile {
//         kind: TileKind::Floor,
//         icon: TileIcon::Floor
//     });

//     let mut starting_area2 = Grid::new(8, vec![
//         wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
//         wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
//         wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
//         wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
//         wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
//         wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
//         wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
//         wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone()
//     ]);

//     // Repeat exactly 3 times.
//     // 1. Select a wall from the remaining empty walls at random.
//     // 2. Determine if a passage or door is to be built
//     // 2a. Build a passage
//     // 2b. Build a door
//     // 3. Remove whichever wall is selected from the list of available walls.

//     let mut exits_to_build = 3;
//     let mut num_doors = 0;
//     let mut num_passages = 0;
//     let mut empty_walls = vec![Wall::North, Wall::South, Wall::East, Wall::West];
//     let mut rng = rand::thread_rng();
//     while exits_to_build > 0 {
//         let wall_index = rng.gen_range(0..empty_walls.len());
//         let wall_selection = &empty_walls[wall_index];

//         if num_passages != 1 {
//             match wall_selection {
//                 Wall::North => {
//                     if dice::roll(12) <= 2 { // This will be a 5ft wide passage
//                         starting_area2 = generate_5ft_passage_ns(starting_area2, 0, 1);
//                     }
//                     else { // This will be a 10ft wide passage
//                         starting_area2 = generate_10ft_passage_ns(starting_area2, 0, 1);
//                     }
//                 },
//                 Wall::South => {
//                     if dice::roll(12) <= 2 { // This will be a 5ft wide passage
//                         starting_area2 = generate_5ft_passage_ns(starting_area2, 6, 7);
//                     }
//                     else { // This will be a 10ft wide passage
//                         starting_area2 = generate_10ft_passage_ns(starting_area2, 6, 7);
//                     }
//                 },
//                 Wall::East => {
//                     if dice::roll(12) <= 2 { // This will be a 5ft wide passage
//                         starting_area2 = generate_5ft_passage_ew(starting_area2, 0, 1);
//                     }
//                     else { // This will be a 10ft wide passage
//                         starting_area2 = generate_10ft_passage_ew(starting_area2, 0, 1);
//                     }
//                 },
//                 Wall::West => {
//                     if dice::roll(12) <= 2 { // This will be a 5ft wide passage
//                         starting_area2 = generate_5ft_passage_ew(starting_area2, 6, 7);
//                     }
//                     else { // This will be a 10ft wide passage
//                         starting_area2 = generate_10ft_passage_ew(starting_area2, 6, 7);
//                     }
//                 }
//             }
//             num_passages = num_passages + 1;
//         }
//         else if num_doors != 2 {
//             match wall_selection {
//                 Wall::North => {
//                     starting_area2 = place_door(starting_area2, Wall::North, 0, 1);
//                 },
//                 Wall::South => {
//                     starting_area2 = place_door(starting_area2, Wall::South, 6, 7);
//                 },
//                 Wall::East => {
//                     starting_area2 = place_door(starting_area2, Wall::East, 0, 1);
//                 },
//                 Wall::West => {
//                     starting_area2 = place_door(starting_area2, Wall::West, 6, 7);
//                 }
//             }
//             num_doors = num_doors + 1;
//         }

//         empty_walls.remove(wall_index);
//         exits_to_build = exits_to_build - 1;
//     }

//     starting_area2
// }

// enum Wall {
//     North,
//     South,
//     East,
//     West
// }

// fn generate_5ft_passage_ns(mut starting_area: Grid, top_row: usize, bottom_row: usize) -> Grid {
//     let mut rng = rand::thread_rng();
//     let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

//     let column = rng.gen_range(2..6);
//     starting_area.nodes[column + (starting_area.columns * top_row)] = floor.clone();
//     starting_area.nodes[column + (starting_area.columns * bottom_row)] = floor.clone();
//     starting_area
// }

// fn generate_5ft_passage_ew(mut starting_area: Grid, left_column: usize, right_column: usize) -> Grid {
//     let mut rng = rand::thread_rng();
//     let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

//     let row = rng.gen_range(2..6);
//     starting_area.nodes[(row * starting_area.columns) + left_column] = floor.clone();
//     starting_area.nodes[(row * starting_area.columns) + right_column] = floor.clone();
//     starting_area
// }

// fn generate_10ft_passage_ns(mut starting_area: Grid, top_row: usize, bottom_row: usize) -> Grid {
//     let mut rng = rand::thread_rng();
//     let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

//     let left_column = rng.gen_range(2..5);
//     starting_area.nodes[left_column + (starting_area.columns * top_row)] = floor.clone();
//     starting_area.nodes[left_column + (starting_area.columns * bottom_row)] = floor.clone();
//     starting_area.nodes[(left_column + 1) + (starting_area.columns * top_row)] = floor.clone();
//     starting_area.nodes[(left_column + 1) + (starting_area.columns * bottom_row)] = floor.clone();
//     starting_area
// }

// fn generate_10ft_passage_ew(mut starting_area: Grid, left_column: usize, right_column: usize) -> Grid {
//     let mut rng = rand::thread_rng();
//     let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

//     let top_row = rng.gen_range(2..5);
//     starting_area.nodes[(top_row * starting_area.columns) + left_column] = floor.clone();
//     starting_area.nodes[(top_row * starting_area.columns) + right_column] = floor.clone();
//     starting_area.nodes[((top_row + 1) * starting_area.columns) + left_column] = floor.clone();
//     starting_area.nodes[((top_row + 1) * starting_area.columns) + right_column] = floor.clone();
//     starting_area
// }

// fn place_door(mut starting_area: Grid, wall: Wall, lower_coordinate: usize, higher_coordinate: usize) -> Grid {
//     let mut rng = rand::thread_rng();
//     let door = Node::Tile(Tile {kind: TileKind::Door, icon: TileIcon::Door});
//     let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

//     match wall {
//         Wall::North => {
//             let column = rng.gen_range(2..6);
//             let top_row = lower_coordinate;
//             let bottom_row = higher_coordinate;
//             starting_area.nodes[column + (starting_area.columns * top_row)] = floor.clone();
//             starting_area.nodes[column + (starting_area.columns * bottom_row)] = door.clone();
//         },
//         Wall::South => {
//             let column = rng.gen_range(2..6);
//             let top_row = lower_coordinate;
//             let bottom_row = higher_coordinate;
//             starting_area.nodes[column + (starting_area.columns * top_row)] = door.clone();
//             starting_area.nodes[column + (starting_area.columns * bottom_row)] = floor.clone();
//         },
//         Wall::East => {
//             let row = rng.gen_range(2..6);
//             let left_column = lower_coordinate;
//             let right_column = higher_coordinate;
//             starting_area.nodes[(row * starting_area.columns) + left_column] = floor.clone();
//             starting_area.nodes[(row * starting_area.columns) + right_column] = door.clone();
//         },
//         Wall::West => {
//             let row = rng.gen_range(2..6);
//             let left_column = lower_coordinate;
//             let right_column = higher_coordinate;
//             starting_area.nodes[(row * starting_area.columns) + left_column] = door.clone();
//             starting_area.nodes[(row * starting_area.columns) + right_column] = floor.clone();
//         }
//     }
//     starting_area
// }