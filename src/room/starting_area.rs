use crate::grid as grid;
use crate::grid::room as room;
use crate::grid::tile as tile;

use rand::Rng;

fn generate_starting_area() -> room::Room {

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

fn generate_starting_area_1() -> room::Room {
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
    let mut starting_area_1: Vec<grid::Module> = Vec::new();
    while tiles_in_grid <= 64 {
        if tiles_in_grid < num_columns * 2 // First 2 rows
        || tiles_in_grid > num_columns * 6 // Last 2 rows
        || tiles_in_grid % 8 // 1st column
        || (tiles_in_grid - 1) % 8 // 2nd column
        || (tiles_in_grid + 1) % 8 // 8th column
        || (tiles_in_grid + 2) % 8 { // 7th column
            starting_area_1.push(grid::Module::Tile(
                tile::Tile {
                    kind: tile::TileKind::Wall,
                    icon: tile::Icon::Wall
                }
            ))
        }
        
        starting_area_1.push(grid::Module::Tile(
            tile::Tile {
                kind: tile::TileKind::Floor,
                icon: tile::Icon::Floor
            }
        ));
        tiles_in_grid = tiles_in_grid + 1;
    }

    let mut rng = rand::thread_rng();

    // Top passage
    if rng.gen_range(1..13) <= 2 { // This will be a 5ft wide passage
        // (column, row)
        let coordinate_bottom: (usize, usize) = (rng.gen_range(2..6), 1);
        let coordinate_top: (usize, usize) = (coordinate_bottom.0, 0);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top.1][coordinate_top.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_top.1][coordinate_top.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_bottom.1][coordinate_bottom.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_bottom.1][coordinate_bottom.0].icon = tile::Icon::Floor;
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_left: (usize, usize) = (rng.gen_range(2..5), 1);
        let coordinate_bottom_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 1);
        let coordinate_top_left: (usize, usize) = (coordinate_bottom_left.0, 0);
        let coordinate_top_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 0);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = tile::Icon::Floor;
    }

    // Bottom passage
    if rng.gen_range(1..13) <= 2 { // This will be a 5ft wide passage
        // (column, row)
        let coordinate_bottom: (usize, usize) = (rng.gen_range(2..6), 7);
        let coordinate_top: (usize, usize) = (coordinate_bottom.0, 6);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top.1][coordinate_top.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_top.1][coordinate_top.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_bottom.1][coordinate_bottom.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_bottom.1][coordinate_bottom.0].icon = tile::Icon::Floor;
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_left: (usize, usize) = (rng.gen_range(2..5), 7);
        let coordinate_bottom_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 7);
        let coordinate_top_left: (usize, usize) = (coordinate_bottom_left.0, 6);
        let coordinate_top_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 6);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = tile::Icon::Floor;
    }

    // Left passage
    if rng.gen_range(1..13) <= 2 { // This will be a 5ft wide passage
        // (column, row)
        let coordinate_right: (usize, usize) = (1, rng.gen_range(2..6));
        let coordinate_left: (usize, usize) = (0, coordinate_right.1);

        // starting_area_1[row][column]
        starting_area_1[coordinate_left.1][coordinate_left.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_left.1][coordinate_left.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_right.1][coordinate_right.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_right.1][coordinate_right.0].icon = tile::Icon::Floor;
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_right: (usize, usize) = (1, rng.gen_range(3..6));
        let coordinate_bottom_left: (usize, usize) = (0, coordinate_bottom_right.1);
        let coordinate_top_left: (usize, usize) = (0, coordinate_bottom_right.1 - 1);
        let coordinate_top_right: (usize, usize) = (1, coordinate_bottom_right.1 - 1);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = tile::Icon::Floor;
    }

    // Right passage
    if rng.gen_range(1..13) <= 2 { // This will be a 5ft wide passage
        // (column, row)
        let coordinate_right: (usize, usize) = (7, rng.gen_range(2..6));
        let coordinate_left: (usize, usize) = (6, coordinate_right.1);

        // starting_area_1[row][column]
        starting_area_1[coordinate_left.1][coordinate_left.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_left.1][coordinate_left.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_right.1][coordinate_right.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_right.1][coordinate_right.0].icon = tile::Icon::Floor;
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_right: (usize, usize) = (7, rng.gen_range(3..6));
        let coordinate_bottom_left: (usize, usize) = (6, coordinate_bottom_right.1);
        let coordinate_top_left: (usize, usize) = (6, coordinate_bottom_right.1 - 1);
        let coordinate_top_right: (usize, usize) = (7, coordinate_bottom_right.1 - 1);

        // starting_area_1[row][column]
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_top_left.1][coordinate_top_left.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_top_right.1][coordinate_top_right.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = tile::Icon::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = tile::TileKind::Floor;
        starting_area_1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = tile::Icon::Floor;
    }

    room::Room {
        exits: vec![
            room::Exit {
                leads_to: room::RoomKind::Passage,
                location: room::ExitLocation::Bottom
            },
            room::Exit {
                leads_to: room::RoomKind::Passage,
                location: room::ExitLocation::Left
            },
            room::Exit {
                leads_to: room::RoomKind::Passage,
                location: room::ExitLocation::Right
            },
            room::Exit {
                leads_to: room::RoomKind::Passage,
                location: room::ExitLocation::Top
            }
        ],
        grid: starting_area_1,
        kind: room::RoomKind::Chamber,
        longest_dimension: 20
    }
}
