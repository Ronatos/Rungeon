pub mod tile;
use std::fmt;

/// A Grid is a 1-dimensional vector of Nodes, which may be Rooms or Tiles.
/// 
/// # Fields
/// 
/// * `columns: usize` - The number of columns in the grid. Used to determine
/// the number and length of columns and rows.
/// * `nodes: Vec<Node>` - A vector of Nodes, which makes up the grid.
/// Nodes may either hold a Room or a Tile.
/// 
/// # Examples
/// 
/// ### Example 1
/// 
/// ```
/// Grid {
///     columns: 3,
///     nodes: vec![
///         Node::Tile(Tile {icon: "a"}), // Column 1
///         Node::Tile(Tile {icon: "b"}), // Column 2
///         Node::Tile(Tile {icon: "c"}), // Column 3
///         Node::Tile(Tile {icon: "d"}), // Column 1
///         Node::Tile(Tile {icon: "e"}), // Column 2
///         Node::Tile(Tile {icon: "f"}), // Column 3
///         Node::Tile(Tile {icon: "g"}), // Column 1
///         Node::Tile(Tile {icon: "h"}), // Column 2
///         Node::Tile(Tile {icon: "i"}), // Column 3
///     ]
/// }
/// ```
/// ### Output
/// 
/// ```
/// a b c
/// d e f
/// g h i
/// ```
#[derive(Clone)]
pub struct Grid {
    pub columns: usize,
    pub nodes: Vec<Node>
}

impl Grid {

    /// Constructs a new Grid structure with the given Node vector and
    /// number of columns.
    /// 
    /// # Arguments
    /// 
    /// * `num_columns: usize` - The number of columns the grid should have.
    /// Note that there is no 0th column. For example: a grid with 1 column and
    /// a vector length of 3 has 3 rows of 1 value each. Vector indexing is handled
    /// internally.
    /// * `nodes: Vec<Node>` - The vector of Nodes that make up the grid.
    /// 
    /// # Panics
    /// 
    /// * The length of the Node vector must be divisible by the number of columns.
    /// 
    /// # Examples
    /// 
    /// ### Example 1
    /// 
    /// ```
    /// let my_grid = grid::new(2, vec![
    ///         grid::Node::Tile(Tile { icon: "H " }),
    ///         grid::Node::Tile(Tile { icon: "E " }),
    ///         grid::Node::Tile(Tile { icon: "Y " })
    ///     ]
    /// );
    /// ```
    /// 
    /// ### Output:
    /// 
    /// ```
    /// H E 
    /// Y 
    /// ```
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

/// The Display function tells std::fmt how to display a grid on the screen.
/// For each Tile in the grid, the Display function is called.
/// 
/// A grid is displayed in the following order:
/// 
/// 01 02 03
/// 04 05 06
/// 07 08 09
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, node) in self.nodes.iter().enumerate() {
            if (i) % self.columns == 0 {
                write!(f, "{}", "\n")?;
            }
            write!(f, "{}", node)?;
        }
        write!(f, "{}", "")
    }
}

/// An enum specifying whether a Grid node is to be a Room or a Tile.
/// 
/// # Variants
/// 
/// * `Room(Grid)` - The Room variant has an associated Grid structure.
/// This is used to specify which room is being described.
/// * `Tile(Tile)` - The Tile variant has an associated Tile structure.
/// This is used to specify which tile is being described.
#[derive(Clone)]
pub enum Node {
    Room(Grid),
    Tile(tile::Tile)
}

/// The Display function tells std::fmt how to display a node on the screen.
/// Nodes must be "displayable" because the Grid Display method relies on displaying nodes.
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Room(grid) => write!(f, "{}", grid),
            Node::Tile(tile) => write!(f, "{}", tile)
        }
    }
}