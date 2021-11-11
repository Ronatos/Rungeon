pub mod room;
pub mod tile;

use std::fmt;

/// A Grid is a 1-dimensional vector of modules, which may be Rooms or Tiles.
/// It is used to describe a room made up of tiles, or a map made up of rooms,
/// and can be displayed to the screen.
/// 
/// Getters and setters are used to ensure grid validity.
/// 
/// # Fields
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
    /// let my_grid = grid::new(2, vec![
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
    /// H E 
    /// Y 
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

/// The Display function tells std::fmt how to display a grid on the screen.
/// For each module, the the Display function is called, which in turn is made of a grid.
/// 
/// For example, if the top level grid is a map of an apartment with several rooms in it,
/// that map would be made up of a grid of rooms. Each of those rooms would be made of a map
/// of tiles. Display would first display room 1, and room 1 would first display tile 1.
/// 
/// A grid is displayed in the following order (grid lines are added for readability on a 3x3 grid):
/// 
/// 01 02 03 | 10 11 12 | 19 20 21
/// 04 05 06 | 13 14 15 | 22 23 24
/// 07 08 09 | 16 17 18 | 25 26 27
/// ---------+----------+---------
/// 28 29 30 | 37 38 39 | 46 47 48
/// 31 32 33 | 40 41 42 | 49 50 51
/// 34 35 36 | 43 44 45 | 52 53 54
/// ---------+----------+---------
/// 55 56 57 | 64 65 66 | 73 74 75
/// 58 59 60 | 67 68 69 | 76 77 78
/// 61 62 63 | 70 71 72 | 79 80 81
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, module) in self.modules.iter().enumerate() {
            if (i + 1) % self.columns == 0 {
                write!(f, "{}", "\n")?;
            }
            write!(f, "{}", module)?;
        }
        write!(f, "{}", "")
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

/// The Display function tells std::fmt how to display a module on the screen.
/// Modules must be "displayable" because the Grid Display method relies on displaying modules.
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Module::Room(room) => write!(f, "{}", room),
            Module::Tile(tile) => write!(f, "{}", tile)
        }
    }
}
