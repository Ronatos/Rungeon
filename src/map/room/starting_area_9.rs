use crate::dice;
use crate::grid::Grid as Grid;
use crate::grid::Node as Node;
use crate::tile::Tile as Tile;
use crate::tile::TileIcon as TileIcon;
use crate::tile::TileKind as TileKind;

// https://github.com/Ronatos/rungeon/wiki/Room#starting-area-9
pub fn new() -> Grid {
    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });

    let d4 = dice::roll(4);
    if d4 == 1 {
        Grid::new(6, vec![
            wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
            wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
            floor.clone(),floor.clone(),floor.clone(), floor.clone(), wall.clone(), wall.clone(),
            floor.clone(),floor.clone(),floor.clone(), floor.clone(), wall.clone(), wall.clone(),
            wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
            wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
        ])
    }
    else if d4 == 2 {
        Grid::new(6, vec![
            wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
            wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
            wall.clone(), wall.clone(), floor.clone(), floor.clone(), floor.clone(),floor.clone(),
            wall.clone(), wall.clone(), floor.clone(), floor.clone(), floor.clone(),floor.clone(),
            wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
            wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
        ])
    }
    else if d4 == 3 {
        Grid::new(6, vec![
            wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),
            floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),
            wall.clone(), wall.clone(), floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(), wall.clone(), floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        ])
    }
    else {
        Grid::new(6, vec![
            wall.clone(), wall.clone(), floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(), wall.clone(), floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),
            floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),
            wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        ])
    }
}