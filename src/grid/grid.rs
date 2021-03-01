use crate::grid::{room, tile};

/// A Grid is a 1-dimensional vector of modules, which may be Rooms or Tiles.
/// It is used to describe a room made up of tiles, or a map made up of rooms,
/// and can be displayed to the screen.
/// 
/// # Arguments
/// 
/// * `columns: usize` - The number of columns in the grid. Used to determine
/// the number and length of columns and rows.
/// * `modules: Vec<Module>` - A vector of Modules, which makes up the grid.
/// Vectors require a type to be specified at compile time. In order to create
/// a generic grid structure to be used by either a Room or a Map, an enum,
/// Modules, must be used as a middleman.
/// 
/// # Examples
/// 
/// ### Example 1
/// 
/// ```
/// Grid {
///     columns: 3,
///     modules: vec![
///         Module::Tile(Tile {icon: "a"}), // Column 1
///         Module::Tile(Tile {icon: "b"}), // Column 2
///         Module::Tile(Tile {icon: "c"}), // Column 3
///         Module::Tile(Tile {icon: "d"}), // Column 1
///         Module::Tile(Tile {icon: "e"}), // Column 2
///         Module::Tile(Tile {icon: "f"}), // Column 3
///         Module::Tile(Tile {icon: "g"}), // Column 1
///         Module::Tile(Tile {icon: "h"}), // Column 2
///         Module::Tile(Tile {icon: "i"}), // Column 3
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
struct Grid {
    columns: usize,
    modules: Vec<Module>
}

impl Grid {

    /// Constructs a new Grid structure with the given Module vector and
    /// number of columns.
    /// 
    /// # Arguments
    /// 
    /// * `num_columns: usize` - The number of columns the grid should have.
    /// Note that there is no 0th column. For example: a grid with 1 column and
    /// a vector length of 3 has 3 rows of 1 value each. Vector indexing is handled
    /// internally.
    /// * `modules: Vec<Module>` - The vector of Modules that make up the grid.
    /// 
    /// # Panics
    /// 
    /// * The length of the Module vector must be divisible by the number of columns.
    /// 
    /// # Examples
    /// 
    /// ### Example 1
    /// 
    /// ```
    /// let my_grid = grid::new(
    ///     3,
    ///     vec![
    ///         grid::Module::Tile(Tile { icon: "H " }),
    ///         grid::Module::Tile(Tile { icon: "E " }),
    ///         grid::Module::Tile(Tile { icon: "Y " })
    ///     ]
    /// );
    /// ```
    /// 
    /// ### Output:
    /// 
    /// ```
    /// H E Y 
    /// ```
    pub fn new(num_columns: usize, modules: Vec<Module>) -> Grid {
        let num_modules = modules.len();
        if num_modules % num_columns != 0 {
            panic!("Grid with {} columns is incompatible with vector length {}.", num_columns, num_modules);
        }

        Grid {
            columns: num_columns,
            modules
        }
    }
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
            },
            TileGrid::RectangleTall3x7(grid) => {
                for row in grid {
                    for element in row {
                        write!(f, "{}", element)?;
                    }
                    write!(f, "{}", "\n");
                }
                write!(f, "{}", "")
            },
            TileGrid::RectangleTall4x7(grid) => {
                for row in grid {
                    for element in row {
                        write!(f, "{}", element)?;
                    }
                    write!(f, "{}", "\n");
                }
                write!(f, "{}", "")
            },
            TileGrid::RectangleWide7x3(grid) => {
                for row in grid {
                    for element in row {
                        write!(f, "{}", element)?;
                    }
                    write!(f, "{}", "\n");
                }
                write!(f, "{}", "")
            },
            TileGrid::RectangleWide7x4(grid) => {
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

/// An enum specifying whether a Grid module is to be a Room or a Tile.
/// 
/// # Variants
/// 
/// * `Room(Room)` - The Room variant has an associated Room structure.
/// This is used to specify which room is being described.
/// * `Tile(Tile)` - The Tile variant has an associated Tile structure.
/// This is used to specify which tile is being described.
pub enum Module {
    Room(room::Room),
    Tile(tile::Tile)
}
