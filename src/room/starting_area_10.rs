use crate::grid;
use grid::Grid as Grid;
use grid::Node as Node;
use grid::tile::Tile as Tile;
use grid::tile::TileIcon as TileIcon;
use grid::tile::TileKind as TileKind;

/// Starting Area 10
/// 
/// Base Shape
/// # #     # #
/// # #     # #
///            
///            
/// # #     # #
/// # #     # #
pub fn new() -> Grid {
    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });

    Grid::new(6, vec![
        wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
        floor.clone(),floor.clone(),floor.clone(), floor.clone(), floor.clone(), floor.clone(),
        floor.clone(),floor.clone(),floor.clone(), floor.clone(), floor.clone(), floor.clone(),
        wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), floor.clone(), floor.clone(), wall.clone(), wall.clone(),
    ])
}