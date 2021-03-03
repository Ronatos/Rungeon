mod grid;

use grid::tile as tile;

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;

// Room -------------------------------------------------------------------------------------------

struct Room {
    grid: TileGrid,
    exits: Exits,
    longest_dimension: u8,
    kind: RoomKind,
}

struct RoomExit {
    location: ExitLocation,
    leads_to: RoomKind
}

enum Exits {
    One([RoomExit; 1]),
    Two([RoomExit; 2]),
    Three([RoomExit; 3]),
    Four([RoomExit; 4]),
    Five([RoomExit; 5]),
    Six([RoomExit; 6])
}

enum ExitLocation {
    Top,
    Bottom,
    Left,
    Right
}

enum RoomKind {
    Chamber,
    Entrance,
    Passage
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

// Map --------------------------------------------------------------------------------------------

struct Map {
    grid: RoomGrid
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

fn main() {
    println!("{}", generate_starting_area_1());
}


// fn generate_map() -> Map {

//     let starting_area = generate_starting_area();
//     let passage_top = generate_passage_from(&starting_area, ExitLocation::Bottom);

//     // The problem here is that the only reason I know these passages are in order
//     // top, bottom, left, right
//     // is because that's the order I added them in to the starting_area structure,
//     // not because there's any data saying so.
//     // Definitely a problem.
//     // let mut passages: Vec<Room> = Vec::new();
//     // match starting_area.exits {
//     //     Exits::One(room_exits) => {
//     //         for exit in room_exits.iter() {
//     //             passages.push(generate_passage_from(starting_area, exit.location));
//     //         }
//     //     },
//     //     Exits::Two(room_exits) => {
//     //         for (i, exit) in room_exits.iter().enumerate() {
//     //             passages.push(generate_passage_from(starting_area, exit.location));
//     //         }
//     //     },
//     //     Exits::Three(room_exits) => {
//     //         for (i, exit) in room_exits.iter().enumerate() {
//     //             passages.push(generate_passage_from(starting_area, exit.location));
//     //         }
//     //     },
//     //     Exits::Four(room_exits) => {
//     //         for (i, exit) in room_exits.iter().enumerate() {
//     //             passages.push(generate_passage_from(starting_area, exit.location));
//     //         }
//     //     },
//     //     Exits::Five(room_exits) => {
//     //         for (i, exit) in room_exits.iter().enumerate() {
//     //             passages.push(generate_passage_from(starting_area, exit.location));
//     //         }
//     //     },
//     //     Exits::Six(room_exits) => {
//     //         for (i, exit) in room_exits.iter().enumerate() {
//     //             passages.push(generate_passage_from(starting_area, exit.location));
//     //         }
//     //     }
//     // }

//     // map[column][row]
//     let mut map: [[Room; 3]; 3] = [
//         [
//             generate_starting_area(),
//             passage_top,
//             generate_starting_area()
//         ],
//         [
//             generate_starting_area(),
//             starting_area,
//             generate_starting_area()
//         ],
//         [
//             generate_starting_area(),
//             generate_starting_area(),
//             generate_starting_area()
//         ]
//     ];

//     Map {
//         grid: RoomGrid::Square3x3(map)
//     }
// }

// So what if this generated the map, and each starting_area function
// recursively built out the map and returned it
fn generate_starting_area() -> grid::Grid {

    let mut rng = rand::thread_rng();
    // let starting_area = rng.gen_range(1..11);
    let starting_area = 1;

    match starting_area {
        1 => generate_starting_area_1(),
        // 2 => get_starting_area_2(),
        // 3 => get_starting_area_3(),
        // 4 => get_starting_area_4(),
        // 5 => get_starting_area_5(),
        // 6 => get_starting_area_6(),
        // 7 => get_starting_area_7(),
        // 8 => get_starting_area_8(),
        // 9 => get_starting_area_9(),
        // 10 => get_starting_area_10(),
        _ => generate_starting_area_1()
    }
}

fn generate_starting_area_1() -> grid::Grid {
    /*
        Starting Area 1

        Base Shape
        # # # # # # # #
        # # # # # # # #
        # #         # #
        # #         # #
        # #         # #
        # #         # #
        # # # # # # # #
        # # # # # # # #

        Four passages will need to be added randomly: 1 per wall
        Each passage can either be 5ft or 10ft wide,
        and extends 10ft from the room.

        # #     # # # #
        # #     # # # #
        # #         # #
                    
                    # #
        # #         # #
        # # #   # # # #
        # # #   # # # #
    */

    let num_columns = 8;
    let tiles_in_grid = 0;
    let mut room: Vec<grid::Module> = Vec::new();
    while tiles_in_grid <= 64 {
        if tiles_in_grid < num_columns * 2 // First 2 rows
        || tiles_in_grid > num_columns * 6 // Last 2 rows
        || tiles_in_grid % 8 // 1st column
        || (tiles_in_grid - 1) % 8 // 2nd column
        || (tiles_in_grid + 1) % 8 // 8th column
        || (tiles_in_grid + 2) % 8 { // 7th column
            room.push(grid::Module::Tile(
                tile::Tile {
                    kind: tile::TileKind::Wall,
                    icon: tile::Icon::Wall
                }
            ))
        }
        
        room.push(grid::Module::Tile(
            tile::Tile {
                kind: tile::TileKind::Floor,
                icon: tile::Icon::Floor
            }
        ));
        tiles_in_grid = tiles_in_grid + 1;
    }

    impl Distribution<ExitLocation> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExitLocation {
            match rng.gen_range(0..4) {
                0 => ExitLocation::Top,
                1 => ExitLocation::Bottom,
                2 => ExitLocation::Left,
                3 => ExitLocation::Right,
                _ => ExitLocation::Top
            }
        }
    }

    // Recursively generate rooms based on exits
    // The map knows which indexes are filled and which aren't
    // because it passes a hash map in to the plant_room_seed()
    // function. This hash map relates unfilled indexes with
    // rooms as they are recursively generated.

    // hash_map = {
    //     0: Empty,
    //     1: Empty,
    //     2: Empty,
    //     3: Empty,
    //     4: Empty,
    //     5: Empty,
    //     6: Empty,
    //     7: Empty,
    //     8: Empty
    //   };
      
    //   map = // looks like this
    //   0 | 1 | 2
    //   - + - + -
    //   3 | 4 | 5
    //   - + - + -
    //   6 | 7 | 8
      
    //   starting_entrance_location = bottom // can be random, but needs to initially match up with the dungeon logically. you wouldn't start the dungeon from index 8 with an entrance from the north
      
    //   starting_room_index = 7 // this could again be random, as long as it matched up with the starting_entrance_location
      
    //   plant_room_generation_seed(hash_map, starting_entrance_location, starting_room_index);
      
    //   plant_room_generation(hash_map, entrance_location, hash_key) {
    //     if (hash_map.hash_key is full || hash_key doesn't exist) {
    //       stop all the recursion!;
    //     }
      
    //     R = Room {
    //       tiles tiles tiles,
    //       exits: [randomly placed exits]
    //     };
    //     hash_map(hash_key: R);
    //     for each exit in R:
    //       if exit.location == top {
    //         plant_room_generation(hash_map, bottom, current_hash_key - 3);
    //       }
    //       else if exit.location == bottom {
    //         plant_room_generation(hash_map, top, current_hash_key + 3);
    //       }
    //       else if exit.location == left && current_hash_key % 3 != 0  {
    //         plant_room_generation(hash_map, right, current_hash_key - 1);
    //       }
    //       else if exit.location == left && current_hash_key % 3 == 0 {
    //         stop the recursion! // we hit a wall at 3 or 6
    //       else if exit.location == right && (current_hash_key - 2) % 3 != 0  {
    //         plant_room_generation(hash_map, left, current_hash_key + 1);
    //       }
    //       else if exit.location == right && (current_hash_key - 2) % 3 == 0 {
    //         stop the recursion! // we hit a wall at 2 or 5
    //       else {
    //         stop the recursion? // isn't this every case?
    //       }
    //     }
    //   }

    let mut rng = rand::thread_rng();
    let entrance_location: ExitLocation = rng.gen();

    let 

    let exit_top = RoomExit {
        location: ExitLocation::Top,
        leads_to: if let ExitLocation::Top = entrance_location {
            RoomKind::Entrance
        }
        else {
            RoomKind::Passage
        }
    };
    let exit_bottom = RoomExit {
        location: ExitLocation::Bottom,
        leads_to: if let ExitLocation::Bottom = entrance_location {
            RoomKind::Entrance
        }
        else {
            RoomKind::Passage
        }
    };
    let exit_left = RoomExit {
        location: ExitLocation::Left,
        leads_to: if let ExitLocation::Left = entrance_location {
            RoomKind::Entrance
        }
        else {
            RoomKind::Passage
        }
    };
    let exit_right = RoomExit {
        location: ExitLocation::Right,
        leads_to: if let ExitLocation::Right = entrance_location {
            RoomKind::Entrance
        }
        else {
            RoomKind::Passage
        }
    };

    // Top passage
    if rng.gen_range(1..13) <= 2 { // This will be a 5ft wide passage
        // (column, row)
        let coordinate_bottom: (usize, usize) = (rng.gen_range(2..6), 1);
        let coordinate_top: (usize, usize) = (coordinate_bottom.0, 0);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top.1][coordinate_top.0].kind = TileKind::Floor;
        starting_area_1[coordinate_top.1][coordinate_top.0].icon = Icon::Floor;
        starting_area_1[coordinate_bottom.1][coordinate_bottom.0].kind = TileKind::Floor;
        starting_area_1[coordinate_bottom.1][coordinate_bottom.0].icon = Icon::Floor;
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_left: (usize, usize) = (rng.gen_range(2..5), 1);
        let coordinate_bottom_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 1);
        let coordinate_top_left: (usize, usize) = (coordinate_bottom_left.0, 0);
        let coordinate_top_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 0);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].kind = TileKind::Floor;
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].icon = Icon::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].kind = TileKind::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].icon = Icon::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = TileKind::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = Icon::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = TileKind::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = Icon::Floor;
    }

    // Bottom passage
    if rng.gen_range(1..13) <= 2 { // This will be a 5ft wide passage
        // (column, row)
        let coordinate_bottom: (usize, usize) = (rng.gen_range(2..6), 7);
        let coordinate_top: (usize, usize) = (coordinate_bottom.0, 6);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top.1][coordinate_top.0].kind = TileKind::Floor;
        starting_area_1[coordinate_top.1][coordinate_top.0].icon = Icon::Floor;
        starting_area_1[coordinate_bottom.1][coordinate_bottom.0].kind = TileKind::Floor;
        starting_area_1[coordinate_bottom.1][coordinate_bottom.0].icon = Icon::Floor;
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_left: (usize, usize) = (rng.gen_range(2..5), 7);
        let coordinate_bottom_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 7);
        let coordinate_top_left: (usize, usize) = (coordinate_bottom_left.0, 6);
        let coordinate_top_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 6);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].kind = TileKind::Floor;
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].icon = Icon::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].kind = TileKind::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].icon = Icon::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = TileKind::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = Icon::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = TileKind::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = Icon::Floor;
    }

    // Left passage
    if rng.gen_range(1..13) <= 2 { // This will be a 5ft wide passage
        // (column, row)
        let coordinate_right: (usize, usize) = (1, rng.gen_range(2..6));
        let coordinate_left: (usize, usize) = (0, coordinate_right.1);

        // starting_area_1[row][column]
        starting_area_1[coordinate_left.1][coordinate_left.0].kind = TileKind::Floor;
        starting_area_1[coordinate_left.1][coordinate_left.0].icon = Icon::Floor;
        starting_area_1[coordinate_right.1][coordinate_right.0].kind = TileKind::Floor;
        starting_area_1[coordinate_right.1][coordinate_right.0].icon = Icon::Floor;
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_right: (usize, usize) = (1, rng.gen_range(3..6));
        let coordinate_bottom_left: (usize, usize) = (0, coordinate_bottom_right.1);
        let coordinate_top_left: (usize, usize) = (0, coordinate_bottom_right.1 - 1);
        let coordinate_top_right: (usize, usize) = (1, coordinate_bottom_right.1 - 1);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].kind = TileKind::Floor;
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].icon = Icon::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].kind = TileKind::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].icon = Icon::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = TileKind::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = Icon::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = TileKind::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = Icon::Floor;
    }

    // Right passage
    if rng.gen_range(1..13) <= 2 { // This will be a 5ft wide passage
        // (column, row)
        let coordinate_right: (usize, usize) = (7, rng.gen_range(2..6));
        let coordinate_left: (usize, usize) = (6, coordinate_right.1);

        // starting_area_1[row][column]
        starting_area_1[coordinate_left.1][coordinate_left.0].kind = TileKind::Floor;
        starting_area_1[coordinate_left.1][coordinate_left.0].icon = Icon::Floor;
        starting_area_1[coordinate_right.1][coordinate_right.0].kind = TileKind::Floor;
        starting_area_1[coordinate_right.1][coordinate_right.0].icon = Icon::Floor;
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_right: (usize, usize) = (7, rng.gen_range(3..6));
        let coordinate_bottom_left: (usize, usize) = (6, coordinate_bottom_right.1);
        let coordinate_top_left: (usize, usize) = (6, coordinate_bottom_right.1 - 1);
        let coordinate_top_right: (usize, usize) = (7, coordinate_bottom_right.1 - 1);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].kind = TileKind::Floor;
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].icon = Icon::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].kind = TileKind::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].icon = Icon::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = TileKind::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = Icon::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = TileKind::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = Icon::Floor;
    }

    Room {
        grid: TileGrid::Square8x8(starting_area_1),
        exits: Exits::Four([exit_top, exit_bottom, exit_left, exit_right]),
        longest_dimension: 20,
        kind: RoomKind::Chamber
    }
}

fn generate_passage_from(room: &Room, entrance_location: ExitLocation) -> Room {
    let mut rng = rand::thread_rng();

    let width = match room.kind {
        RoomKind::Passage => {
            // width must be at most 10
            if rng.gen_range(1..13) <= 2 {
                5
            }
            else {
                10
            }
        },
        RoomKind::Chamber => {
            let max_width = room.longest_dimension - 5;
            let dice_sides = if max_width < 10 {
                2
            }
            else if max_width < 20 {
                12
            }
            else if max_width < 30 {
                14
            }
            else if max_width < 40 {
                16
            }
            else {
                20
            };
            match rng.gen_range(1..dice_sides + 1) {
                1..=2 => 5,
                3..=12 => 10,
                13..=14 => 20,
                15..=16 => 30,
                17..=20 => 40,
                _ => 10
            }
        },
        _ => 10 // technically this should throw an error. Entrance should never get here
    };

    match rng.gen_range(1..21) {
        1..=2 => generate_passage_1(entrance_location, width),
        // 3 => generate_passage_2(entrance_location, width),
        // 4 => generate_passage_3(entrance_location, width),
        // 5 => generate_passage_4(entrance_location, width),
        // 6..=7 => generate_passage_5(entrance_location, width),
        // 8..=9 => generate_passage_6(entrance_location, width),
        // 10 => generate_passage_7(entrance_location, width),
        // 11..=12 => generate_passage_8(entrance_location, width),
        // 13..=14 => generate_passage_9(entrance_location, width),
        // 15..=19 => generate_chamber(entrance_location, width),
        // 20 => generate_stairs(entrance_location, width),
        _ => generate_passage_1(entrance_location, width)
    }
}

fn generate_passage_1(entrance_location: ExitLocation, width: u8) -> Room {
    /*
        At this point I should know exactly which passage I need to print.

        Passage 1
        ----------
        P1 Variant 1a
        # # #    #   #
        #   #    #   #
        #   #    #   #
        #   #    #   #
        #   #    #   #
        #   # or #   # 
        #   #    # # #

        P1 Variant 2a
        # # # #    # # # #
        #     #    #     #
        #     #    #     #
        #     #    #     #
        #     #    #     #
        #     #    #     #
        #     # or #     #

        P1 Variant 3a
        #        #
        #        #
        #        #
        #        #
        #        #
        #        #

        P1 Variant 4a
        #            #
        #            #
        #            #
        #            #
        #            #
        #            #

        P1 Variant 5a
        #       ##       #
        #                #
        #       ##       #
        #       ##       #
        #                #
        #       ##       #

        P1 Variant 6a
        #   ##      ##   #
        #                #
        #   ##      ##   #
        #   ##      ##   #
        #                #
        #   ##      ##   #

        P1 Variant 1b
        # # # # # # #    # # # # # # #
                    # or #
        # # # # # # #    # # # # # # #

        P1 Variant 2b
        # # # # # # # or # # # # # # #
                    #    #
                    #    #
        # # # # # # #    # # # # # # #

        P1 Variant 3b
        # # # # # #




        # # # # # #

        P1 Variant 4b
        # # # # # #






        # # # # # #

        P1 Variant 5b
        # # # # # #
             
             
             
        #   # #   #
        #   # #   #



        # # # # # #

        P1 Variant 6b
        # # # # # #

        
        #   # #   #

        
        #   # #   #
        

        # # # # # #
    */

    match entrance_location {
        ExitLocation::Top => {
            match width {
                5 => {
                    let passage_1: [[Tile; 3]; 7] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let top_exit = RoomExit {
                        location: ExitLocation::Top,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleTall3x7(passage_1),
                        exits: Exits::One([top_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                },
                10 => {
                    let passage_1: [[Tile; 4]; 7] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let top_exit = RoomExit {
                        location: ExitLocation::Top,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleTall4x7(passage_1),
                        exits: Exits::One([top_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                },
                _ => { // This needs to be an exhaustive list of the room widths. 10 is here as a placeholder
                    let passage_1: [[Tile; 4]; 7] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let top_exit = RoomExit {
                        location: ExitLocation::Top,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleTall4x7(passage_1),
                        exits: Exits::One([top_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                }
            }
        },
        ExitLocation::Bottom => {
            match width {
                5 => {
                    let passage_1: [[Tile; 3]; 7] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let bottom_exit = RoomExit {
                        location: ExitLocation::Bottom,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleTall3x7(passage_1),
                        exits: Exits::One([bottom_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                },
                10 => {
                    let passage_1: [[Tile; 4]; 7] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let bottom_exit = RoomExit {
                        location: ExitLocation::Bottom,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleTall4x7(passage_1),
                        exits: Exits::One([bottom_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                },
                _ => { // This needs to be an exhaustive list of the room widths. 10 is here as a placeholder
                    let passage_1: [[Tile; 4]; 7] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let bottom_exit = RoomExit {
                        location: ExitLocation::Bottom,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleTall4x7(passage_1),
                        exits: Exits::One([bottom_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                }
            }
        },
        ExitLocation::Left => {
            match width {
                5 => {
                    let passage_1: [[Tile; 7]; 3] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let left_exit = RoomExit {
                        location: ExitLocation::Left,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleWide7x3(passage_1),
                        exits: Exits::One([left_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                },
                10 => {
                    let passage_1: [[Tile; 7]; 4] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let left_exit = RoomExit {
                        location: ExitLocation::Left,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleWide7x4(passage_1),
                        exits: Exits::One([left_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                },
                _ => { // This needs to be an exhaustive list of the room widths. 10 is here as a placeholder
                    let passage_1: [[Tile; 7]; 4] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let left_exit = RoomExit {
                        location: ExitLocation::Left,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleWide7x4(passage_1),
                        exits: Exits::One([left_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                }
            }
        },
        ExitLocation::Right => {
            match width {
                5 => {
                    let passage_1: [[Tile; 7]; 3] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let right_exit = RoomExit {
                        location: ExitLocation::Right,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleWide7x3(passage_1),
                        exits: Exits::One([right_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                },
                10 => {
                    let passage_1: [[Tile; 7]; 4] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let right_exit = RoomExit {
                        location: ExitLocation::Right,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleWide7x4(passage_1),
                        exits: Exits::One([right_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                },
                _ => { // This needs to be an exhaustive list of the room widths. 10 is here as a placeholder
                    let passage_1: [[Tile; 7]; 4] = [
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile { kind: TileKind::Floor, icon: Icon::Floor }
                        ],
                        [
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile { kind: TileKind::Wall, icon: Icon::Wall }
                        ]
                    ];

                    let right_exit = RoomExit {
                        location: ExitLocation::Right,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleWide7x4(passage_1),
                        exits: Exits::One([right_exit]),
                        longest_dimension: 30,
                        kind: RoomKind::Passage
                    }
                }
            }
        }
    }
}