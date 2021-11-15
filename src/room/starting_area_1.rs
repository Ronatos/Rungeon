mod dice;

mod grid;
use grid::Grid as Grid;
use grid::Node as GridNode;
use grid::container::Container as Container;
use grid::tile::Tile as Tile;
use grid::tile::TileIcon as TileIcon;
use grid::tile::TileKind as TileKind;

pub fn new() -> Grid {
    let mut rng = rand::thread_rng();
    let wall = GridNode::Tile(Tile {kind: TileKind::Wall, icon: TileIcon::Wall});
    let floor = GridNode::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

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
        let column = rng.gen_range(2..6);
        let top_row = 0;
        let bottom_row = 1;

        starting_area1[top_row][column] = floor.clone();
        starting_area1[bottom_row][column] = floor.clone();
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_left: (usize, usize) = (rng.gen_range(2..5), 1);
        let coordinate_bottom_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 1);
        let coordinate_top_left: (usize, usize) = (coordinate_bottom_left.0, 0);
        let coordinate_top_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 0);

        // starting_area1[row][column]
        starting_area1[coordinate_top_left.1][coordinate_top_left.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_top_left.1][coordinate_top_left.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_top_right.1][coordinate_top_right.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_top_right.1][coordinate_top_right.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = tile::Icon::Floor;
    }

    // Bottom passage
    if rng.gen_range(1..13) <= 2 { // This will be a 5ft wide passage
        // (column, row)
        let coordinate_bottom: (usize, usize) = (rng.gen_range(2..6), 7);
        let coordinate_top: (usize, usize) = (coordinate_bottom.0, 6);

        // starting_area1[row][column]
        starting_area1[coordinate_top.1][coordinate_top.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_top.1][coordinate_top.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_bottom.1][coordinate_bottom.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_bottom.1][coordinate_bottom.0].icon = tile::Icon::Floor;
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_left: (usize, usize) = (rng.gen_range(2..5), 7);
        let coordinate_bottom_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 7);
        let coordinate_top_left: (usize, usize) = (coordinate_bottom_left.0, 6);
        let coordinate_top_right: (usize, usize) = (coordinate_bottom_left.0 + 1, 6);

        // starting_area1[row][column]
        starting_area1[coordinate_top_left.1][coordinate_top_left.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_top_left.1][coordinate_top_left.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_top_right.1][coordinate_top_right.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_top_right.1][coordinate_top_right.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = tile::Icon::Floor;
    }

    // Left passage
    if rng.gen_range(1..13) <= 2 { // This will be a 5ft wide passage
        // (column, row)
        let coordinate_right: (usize, usize) = (1, rng.gen_range(2..6));
        let coordinate_left: (usize, usize) = (0, coordinate_right.1);

        // starting_area1[row][column]
        starting_area1[coordinate_left.1][coordinate_left.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_left.1][coordinate_left.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_right.1][coordinate_right.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_right.1][coordinate_right.0].icon = tile::Icon::Floor;
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_right: (usize, usize) = (1, rng.gen_range(3..6));
        let coordinate_bottom_left: (usize, usize) = (0, coordinate_bottom_right.1);
        let coordinate_top_left: (usize, usize) = (0, coordinate_bottom_right.1 - 1);
        let coordinate_top_right: (usize, usize) = (1, coordinate_bottom_right.1 - 1);

        // starting_area1[row][column]
        starting_area1[coordinate_top_left.1][coordinate_top_left.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_top_left.1][coordinate_top_left.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_top_right.1][coordinate_top_right.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_top_right.1][coordinate_top_right.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = tile::Icon::Floor;
    }

    // Right passage
    if rng.gen_range(1..13) <= 2 { // This will be a 5ft wide passage
        // (column, row)
        let coordinate_right: (usize, usize) = (7, rng.gen_range(2..6));
        let coordinate_left: (usize, usize) = (6, coordinate_right.1);

        // starting_area1[row][column]
        starting_area1[coordinate_left.1][coordinate_left.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_left.1][coordinate_left.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_right.1][coordinate_right.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_right.1][coordinate_right.0].icon = tile::Icon::Floor;
    }
    else { // This will be a 10ft wide passage
        // (column, row)
        let coordinate_bottom_right: (usize, usize) = (7, rng.gen_range(3..6));
        let coordinate_bottom_left: (usize, usize) = (6, coordinate_bottom_right.1);
        let coordinate_top_left: (usize, usize) = (6, coordinate_bottom_right.1 - 1);
        let coordinate_top_right: (usize, usize) = (7, coordinate_bottom_right.1 - 1);

        // starting_area1[row][column]
        starting_area1[coordinate_top_left.1][coordinate_top_left.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_top_left.1][coordinate_top_left.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_top_right.1][coordinate_top_right.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_top_right.1][coordinate_top_right.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_bottom_left.1][coordinate_bottom_left.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_bottom_left.1][coordinate_bottom_left.0].icon = tile::Icon::Floor;
        starting_area1[coordinate_bottom_right.1][coordinate_bottom_right.0].kind = tile::TileKind::Floor;
        starting_area1[coordinate_bottom_right.1][coordinate_bottom_right.0].icon = tile::Icon::Floor;
    }
}