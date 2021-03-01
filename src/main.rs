/*
    Ronatos (02/28 7:30pm):
    The big issue right now is how to handle more than one room in a map.
    Sure, I can just generate random rooms and slap them together,
    but that doesn't really get me anywhere.

    The hope is that I can create a room,
    easily see the connection points that room has,
    and generate a new room that sensibly connects to it.

    In that effort, I've prototyped out the addition of a passage,
    but have run in to a number of issues that should be addressed.

    Firstly, the code feels sloppy. I don't care for it.
    Development of Tile structure, Room structure, and starting_area_1 were
    very meticulous, and I want to feel that same way about passages.
    Right now, they feel rushed.

    Secondly, I've tacked on numerous fields in the effort of making passages work.
    I am glad I did it, because it has given me an understanding of some of the challenges
    involved in creating passages, but it is time to move on to a better
    implementation.
*/

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;

// Grids ------------------------------------------------------------------------------------------

enum TileGrid {
    RectangleTall3x6([[Tile; 3]; 6]),
    Square8x8([[Tile; 8]; 8])
}

enum RoomGrid {
    Square3x3([[Room; 3]; 3])
}

impl fmt::Display for TileGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileGrid::Square8x8(grid) => {
                for row in grid {
                    for element in row {
                        write!(f, "{}", element)?;
                    }
                    write!(f, "{}", "\n");
                }
                write!(f, "{}", "")
            }
        }
    }
}

impl fmt::Display for RoomGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoomGrid::Square3x3(grid) => {
                for row in grid {
                    for element in row {
                        write!(f, "{}---------------\n", element)?;
                    }
                }
                write!(f, "{}", "")
            }
        }
    }
}

// Tile -------------------------------------------------------------------------------------------

struct Tile {
    kind: TileKind,
    icon: Icon
}

enum TileKind {
    Door,
    Floor,
    Stairs,
    Wall
}

enum Icon {
    Floor,
    Wall
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Icon::Wall => write!(f, "{} ", "#"),
            Icon::Floor => write!(f, "{} ", " ")
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.icon)
    }
}

// Room -------------------------------------------------------------------------------------------

struct Room {
    grid: TileGrid,
    exits: Exits,
    longest_dimension: u8,
    kind: RoomKind
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

enum Orientation {
    Vertical,
    Horizontal
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
    println!("{}", generate_map());
}


fn generate_map() -> Map {

    let starting_area = generate_starting_area();

    // The problem here is that the only reason I know these passages are in order
    // top, bottom, left, right
    // is because that's the order I added them in to the starting_area structure,
    // not because there's any data saying so.
    // Definitely a problem.
    let passages: Vec<Room> = Vec::new();
    match starting_area.exits {
        Exits::Four(room_exits) => {
            for (i, exit) in room_exits.iter().enumerate() {
                passages.push(generate_passage_from(&starting_area, exit.location));
            }
        }
    }

    // map[column][row]
    let mut map: [[Room; 3]; 3] = [
        [
            generate_starting_area(),
            generate_starting_area(),
            generate_starting_area()
        ],
        [
            generate_starting_area(),
            generate_starting_area(),
            generate_starting_area()
        ],
        [
            generate_starting_area(),
            generate_starting_area(),
            generate_starting_area()
        ]
    ];

    Map {
        grid: RoomGrid::Square3x3(map)
    }
}

fn generate_starting_area() -> Room {

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

fn generate_starting_area_1() -> Room {
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

    // starting_area_1[column][row]
    let mut starting_area_1: [[Tile; 8]; 8] = [
        [
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
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
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall }
        ],
        [
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall }
        ],
        [
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall }
        ],
        [
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Floor, icon: Icon::Floor },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall }
        ],
        [
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
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
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall },
            Tile { kind: TileKind::Wall, icon: Icon::Wall }
        ]
    ];

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

    let mut rng = rand::thread_rng();
    let entrance_location: ExitLocation = rng.gen();

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

fn generate_passage_from(room: &Room, origin: ExitLocation) -> Room {
    let orientation = match origin {
        ExitLocation::Top => Orientation::Vertical,
        ExitLocation::Bottom => Orientation::Vertical,
        ExitLocation::Left => Orientation::Horizontal,
        ExitLocation::Right => Orientation::Horizontal
    };

    let rng = rand::thread_rng();

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
        }
    };

    match rng.gen_range(1..21) {
        1..=2 => generate_passage_1(orientation, width),
        // 3 => generate_passage_2(orientation, width),
        // 4 => generate_passage_3(orientation, width),
        // 5 => generate_passage_4(orientation, width),
        // 6..=7 => generate_passage_5(orientation, width),
        // 8..=9 => generate_passage_6(orientation, width),
        // 10 => generate_passage_7(orientation, width),
        // 11..=12 => generate_passage_8(orientation, width),
        // 13..=14 => generate_passage_9(orientation, width),
        // 15..=19 => generate_chamber(orientation, width),
        // 20 => generate_stairs(orientation, width),
        _ => generate_passage_1(orientation, width)
    }
}

fn generate_passage_1(orientation: Orientation, width: u8) -> Room {
    /*
        At this point I know exactly which passage I need to print.

        Passage 1
        ----------
        P1 Variant 1a
        #  #
        #  #
        #  #
        #  #
        #  #
        #  #

        P1 Variant 2a
        #    #
        #    #
        #    #
        #    #
        #    #
        #    #

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
        # # # # # #
                   
        # # # # # #

        P1 Variant 2b
        # # # # # #


        # # # # # #

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

    match orientation {
        Orientation::Vertical => {
            match width {
                5 => {
                    let passage_1: [[Tile; 3]; 6] = [
                        [
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile  { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile  { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile  { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile  { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile  { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                        [
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall },
                            Tile  { kind: TileKind::Floor, icon: Icon::Floor },
                            Tile  { kind: TileKind::Wall, icon: Icon::Wall }
                        ],
                    ];

                    let top_exit = RoomExit {
                        location: ExitLocation::Top,
                        leads_to: RoomKind::Chamber
                    };

                    let bottom_exit = RoomExit {
                        location: ExitLocation::Bottom,
                        leads_to: RoomKind::Chamber
                    };

                    Room {
                        grid: TileGrid::RectangleTall3x6(passage_1),
                        exits: Exits::Two([]),
                        longest_dimension: u8,
                        kind: RoomKind
                    }
                }
            }
        },
        Orientation::Horizontal => {

        }
    }
}