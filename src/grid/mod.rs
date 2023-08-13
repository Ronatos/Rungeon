pub mod map;
pub mod room;
pub mod tile;

use std::fmt;

// https://github.com/Ronatos/rungeon/wiki/Grid#gridgrid
#[derive(Clone)]
pub struct Grid {
    pub columns: usize,
    pub nodes: Vec<Node>
}

// https://github.com/Ronatos/rungeon/wiki/Grid#gridgrid
impl Grid {
    pub fn new(num_columns: usize, nodes: Vec<Node>) -> Grid {
        let num_nodes = nodes.len();
        if num_nodes % num_columns != 0 {
            panic!("Grid with {} columns is incompatible with vector length {}.", num_columns, num_nodes);
        }

        Grid {
            columns: num_columns,
            nodes
        }
    }
}

// https://github.com/Ronatos/rungeon/wiki/Grid#gridgriddisplay
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, node) in self.nodes.iter().enumerate() {
            if (i) % self.columns == 0 {
                write!(f, "{}", "\n")?;
            }
            write!(f, "{}", node)?;
        }
        write!(f, "{}", "") // Remove the \n---------- when you're done debugging
    }
}

// https://github.com/Ronatos/rungeon/wiki/Grid#gridgridnode
#[derive(Clone)]
pub enum Node {
    Room(Grid),
    Tile(tile::Tile)
}

// https://github.com/Ronatos/rungeon/wiki/Grid#gridgridnodedisplay
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Room(grid) => write!(f, "{}", grid),
            Node::Tile(tile) => write!(f, "{}", tile)
        }
    }
}