use crate::grid::Grid as Grid;
use crate::grid::Node as Node;
use crate::grid::room::place_door as place_door;
use crate::grid::room::Wall as Wall;
use crate::grid::tile::Tile as Tile;
use crate::grid::tile::TileIcon as TileIcon;
use crate::grid::tile::TileKind as TileKind;

use rand::Rng;

// https://github.com/Ronatos/rungeon/wiki/Room#starting-area-3
pub fn new() -> Grid {
    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });

    let mut starting_area3 = Grid::new(12, vec![
        wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone()
    ]);

    // Repeat exactly 3 times.
    // 1. Select a wall from the remaining empty walls at random.
    // 2. Build a door
    // 3. Remove whichever wall is selected from the list of available walls.

    let mut exits_to_build = 3;
    let mut empty_walls = vec![Wall::North, Wall::South, Wall::East, Wall::West];
    let mut rng = rand::thread_rng();
    while exits_to_build > 0 {
        let wall_index = rng.gen_range(0..empty_walls.len());
        let wall_selection = &empty_walls[wall_index];

        match wall_selection {
            Wall::North => {
                starting_area3 = place_door(starting_area3, Wall::North);
            },
            Wall::South => {
                starting_area3 = place_door(starting_area3, Wall::South);
            },
            Wall::East => {
                starting_area3 = place_door(starting_area3, Wall::East);
            },
            Wall::West => {
                starting_area3 = place_door(starting_area3, Wall::West);
            }
        }

        empty_walls.remove(wall_index);
        exits_to_build = exits_to_build - 1;
    }

    starting_area3
}