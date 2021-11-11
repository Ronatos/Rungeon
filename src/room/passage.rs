use crate::grid::room as room;

fn generate_passage(entrance_location: room::ExitLocation, previous_room_longest_dimension: u8, previous_room_kind: room::RoomKind) -> room::Room {
    let mut rng = rand::thread_rng();

    let width = match previous_room_kind {
        room::RoomKind::Passage => {
            // width must be at most 10
            if rng.gen_range(1..13) <= 2 {
                5
            }
            else {
                10
            }
        },
        room::RoomKind::Chamber => {
            let max_width = previous_room_longest_dimension - 5;
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
        # # #
        #   #
        #   #
        #   #
        #   #
        #   # 
        #   #

        #   #
        #   #
        #   #
        #   #
        #   #
        #   # 
        # # #

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