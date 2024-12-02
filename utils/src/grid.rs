use crate::{
    dir::{Direction, Movement, CARDINALS},
    pos::Position,
};

/// A 2D integer grid of values.
///
/// This grid is represented by a vector of vectors.
///
/// # Examples
///
/// ```
/// use utils::prelude::*;
///
/// let data = vec![
///    vec![1, 2, 3],
///    vec![4, 5, 6],
///    vec![7, 8, 9],
/// ];
///
/// let grid = Grid::new(data);
///
/// assert_eq!(grid.get(Position::new(0, 0)), Some(&1));
/// assert_eq!(grid.get(Position::new(1, 1)), Some(&5));
/// assert_eq!(grid.get(Position::new(2, 2)), Some(&9));
/// ```
///
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// Create a new grid from a vector of vectors.
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    /// Parse a grid from a string, this will convert each character into `T` via `From<char>`.
    ///
    /// Use the `tiles!` macro to easily create an enum that implements `From<char>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// enum Tile {
    ///    Floor,
    ///    Wall,
    /// }
    ///
    /// impl From<char> for Tile {
    ///   fn from(c: char) -> Self {
    ///      match c {
    ///         '.' => Self::Floor,
    ///         '#' => Self::Wall,
    ///         _ => panic!("Invalid tile {c}"),
    ///      }
    ///   }
    /// }
    ///
    /// let input = ".#.\n#.#\n.#.";
    /// let grid = Grid::<Tile>::parse(input);
    ///
    /// assert_eq!(grid.get(Position::new(0, 0)), Some(&Tile::Floor));
    /// assert_eq!(grid.get(Position::new(1, 0)), Some(&Tile::Wall));
    /// assert_eq!(grid.get(Position::new(2, 0)), Some(&Tile::Floor));
    /// assert_eq!(grid.get(Position::new(1, 1)), Some(&Tile::Floor));
    /// ```
    ///
    /// Using `tiles!`...
    ///
    /// ```
    /// use utils::prelude::*;
    /// use utils::tiles;
    ///
    /// tiles!(Tile, [
    ///    '.' => Floor,
    ///    '#' => Wall,
    /// ]);
    ///
    /// let input = ".#.\n#.#\n.#.";
    /// let grid = Grid::<Tile>::parse(input);
    ///
    /// assert_eq!(grid.get(Position::new(0, 0)), Some(&Tile::Floor));
    /// assert_eq!(grid.get(Position::new(1, 0)), Some(&Tile::Wall));
    /// assert_eq!(grid.get(Position::new(2, 0)), Some(&Tile::Floor));
    /// assert_eq!(grid.get(Position::new(1, 1)), Some(&Tile::Floor));
    /// ```
    ///
    pub fn parse(input: &str) -> Self
    where
        T: From<char>,
    {
        let data = input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();
        Self::new(data)
    }

    /// Return the width of the grid.
    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    /// Return the height of the grid.
    pub fn height(&self) -> usize {
        self.data.len()
    }

    /// Get the size of the grid.
    pub fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    /// Get the bounds of the grid.
    ///
    /// (This is the same as `self.size()` with -1 added to each component)
    pub fn bounds(&self) -> (usize, usize) {
        (self.width() - 1, self.height() - 1)
    }

    /// Get a value from the grid at the given position.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let data = vec![
    ///    vec![1, 2, 3],
    ///    vec![4, 5, 6],
    ///    vec![7, 8, 9],
    /// ];
    ///
    /// let grid = Grid::new(data);
    /// assert_eq!(grid.get(Position::new(0, 0)), Some(&1));
    /// assert_eq!(grid.get(Position::new(1, 1)), Some(&5));
    /// assert_eq!(grid.get(Position::new(2, 2)), Some(&9));
    /// assert_eq!(grid.get(Position::new(3, 3)), None);
    /// ```
    ///
    pub fn get(&self, pos: Position) -> Option<&T> {
        self.data
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
    }

    /// Get a value from the grid at the given position,
    /// panicking if the position is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let data = vec![
    ///   vec![1, 2, 3],
    ///   vec![4, 5, 6],
    ///   vec![7, 8, 9],
    /// ];
    ///
    /// let grid = Grid::new(data);
    ///
    /// assert_eq!(grid.unsafe_get(Position::new(0, 0)), &1);
    /// assert_eq!(grid.unsafe_get(Position::new(1, 1)), &5);
    /// ```
    ///
    pub fn unsafe_get(&self, pos: Position) -> &T {
        &self.data[pos.y as usize][pos.x as usize]
    }

    /// Get the value at the given position, wrapping around the grid if necessary.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let data = vec![
    ///    vec![1, 2, 3],
    ///    vec![4, 5, 6],
    ///    vec![7, 8, 9],
    /// ];
    ///
    /// let grid = Grid::new(data);
    /// assert_eq!(grid.get_wrapped(Position::new(0, 0)), &1);
    /// assert_eq!(grid.get_wrapped(Position::new(1, 1)), &5);
    /// assert_eq!(grid.get_wrapped(Position::new(2, 2)), &9);
    /// assert_eq!(grid.get_wrapped(Position::new(3, 3)), &1);
    /// assert_eq!(grid.get_wrapped(Position::new(-1, -1)), &9);
    /// ```
    ///
    pub fn get_wrapped(&self, pos: Position) -> &T {
        let wrapped_pos = pos.bind(self.size());
        &self.data[wrapped_pos.1][wrapped_pos.0]
    }

    /// Iterate over a row of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let data = vec![
    ///    vec![1, 2, 3],
    ///    vec![4, 5, 6],
    ///    vec![7, 8, 9],
    /// ];
    ///
    /// let grid = Grid::new(data);
    ///
    /// assert_eq!(grid.iter_row(0).unwrap().collect::<Vec<_>>(), vec![&1, &2, &3]);
    /// assert_eq!(grid.iter_row(1).unwrap().sum::<usize>(), 4+5+6);
    /// assert!(grid.iter_row(8).is_none());
    /// ```
    ///
    pub fn iter_row(&self, row: usize) -> Option<impl Iterator<Item = &T>> {
        self.data.get(row).map(|row| row.iter())
    }

    /// Iterate over a column of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    ///
    /// let data = vec![
    ///    vec![1, 2, 3],
    ///    vec![4, 5, 6],
    ///    vec![7, 8, 9],
    /// ];
    ///
    /// let grid = Grid::new(data);
    ///
    /// assert_eq!(grid.iter_col(0).unwrap().collect::<Vec<_>>(), vec![&1, &4, &7]);
    /// assert_eq!(grid.iter_col(1).unwrap().sum::<usize>(), 2+5+8);
    /// assert!(grid.iter_col(8).is_none());
    /// ```
    ///
    pub fn iter_col(&self, col: usize) -> Option<impl Iterator<Item = &T>> {
        if col > self.width() {
            return None;
        }
        Some(self.data.iter().filter_map(move |row| row.get(col)))
    }

    /// Get a row of the grid.
    ///
    /// This is the same as `self.iter_row(row).map(|iter| iter.collect())`.
    pub fn get_row(&self, y: usize) -> Option<Vec<&T>> {
        self.iter_row(y).map(|iter| iter.collect())
    }

    /// Get a column of the grid.
    ///
    /// This is the same as `self.iter_col(col).map(|iter| iter.collect())`.
    pub fn get_col(&self, x: usize) -> Option<Vec<&T>> {
        self.iter_col(x).map(|iter| iter.collect())
    }

    /// Iterate over all rows of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    ///
    /// let data = vec![
    ///    vec![1, 2, 3],
    ///    vec![4, 5, 6],
    ///    vec![7, 8, 9],
    /// ];
    ///
    /// let grid = Grid::new(data);
    ///
    /// assert_eq!(grid.iter_rows().enumerate().filter_map(|(y, row)| row.collect::<Vec<_>>().get(y).copied()).sum::<usize>(), 1+5+9);
    /// ```
    ///
    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        self.data.iter().map(|row| row.iter())
    }

    /// Iterate over all columns of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    ///
    /// let data = vec![
    ///    vec![1, 2, 3],
    ///    vec![4, 5, 6],
    ///    vec![7, 8, 9],
    /// ];
    ///
    /// let grid = Grid::new(data);
    ///
    /// assert_eq!(grid.iter_cols().enumerate().filter_map(|(x, col)| col.collect::<Vec<_>>().get(x).copied()).sum::<usize>(), 1+5+9);
    /// ```
    ///
    pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.width()).map(move |col| self.iter_col(col).unwrap())
    }

    /// Iterate over all elements of the grid.
    ///
    /// This also yields the position of each element for easy access.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let data = vec![
    ///    vec![1, 2, 3],
    ///    vec![4, 5, 6],
    ///    vec![7, 8, 9],
    /// ];
    ///
    /// let grid = Grid::new(data);
    ///
    /// assert_eq!(grid.iter().map(|(_, v)| v).sum::<usize>(), 1+2+3+4+5+6+7+8+9);
    /// ```
    ///
    pub fn iter(&self) -> impl Iterator<Item = (Position, &T)> {
        self.data.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, col)| (Position::new(x as isize, y as isize), col))
        })
    }

    /// Get all positions relative to the given position in the grid based off the given kernels.
    ///
    /// This will automatically filter out any positions that are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let data = vec![
    ///    vec![1, 2, 3],
    ///    vec![4, 5, 6],
    ///    vec![7, 8, 9],
    /// ];
    ///
    /// let grid = Grid::new(data);
    ///
    /// let pos = Position::new(1, 1);
    /// let kernels = &[
    ///   Direction::North,
    ///   Direction::East,
    /// ];
    ///
    /// let mut relatives = grid.relatives(pos, kernels);
    ///
    /// assert_eq!(relatives.next(), Some((Direction::North, Position::new(1, 0), &2)));
    /// assert_eq!(relatives.next(), Some((Direction::East, Position::new(2, 1), &6)));
    /// ```
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let data = vec![
    ///   vec![1, 2, 3],
    ///   vec![4, 5, 6],
    ///   vec![7, 8, 9],
    /// ];
    ///
    /// let grid = Grid::new(data);
    ///
    /// let pos = Position::new(1, 0);
    /// let kernels = &[
    ///  Direction::North, // This will be filtered out, as (1, -1) is out of bounds
    ///  Direction::East,
    /// ];
    ///
    /// let mut relatives = grid.relatives(pos, kernels);
    ///
    /// assert_eq!(relatives.next(), Some((Direction::East, Position::new(2, 0), &3)));
    /// ```
    ///
    pub fn relatives<'a, M: Movement>(
        &'a self,
        pos: Position,
        kernels: &'a [M],
    ) -> impl Iterator<Item = (M, Position, &'a T)> + 'a {
        pos.relatives(kernels)
            .filter_map(move |(pos, dir)| self.get(pos).map(|v| (dir, pos, v)))
    }

    /// Get all positions relative to the given position in the grid based off the given kernels.
    ///
    /// Wraps around the grid if necessary.
    ///
    pub fn relatives_wrapped<'a, M: Movement>(
        &'a self,
        pos: Position,
        kernels: &'a [M],
    ) -> impl Iterator<Item = (M, Position, &'a T)> + 'a {
        pos.relatives(kernels)
            .map(move |(pos, dir)| (dir, pos, self.get_wrapped(pos)))
    }

    /// Get all positions relative to the given position in the grid based off the given kernels,
    /// applying the kernel multiple times.
    ///
    /// This will automatically filter out any positions that are out of bounds.
    ///
    pub fn relatives_expand_by<'a, M: Movement>(
        &'a self,
        pos: Position,
        kernels: &'a [M],
        expand: usize,
    ) -> impl Iterator<Item = ((M, usize), Position, &'a T)> + 'a {
        pos.relatives_expand_by(kernels, expand)
            .filter_map(move |(dir, pos)| self.get(pos).map(|v| (dir, pos, v)))
    }

    /// Get all positions relative to the given position in the grid based off the given kernels,
    /// applying the kernel multiple times.
    ///
    /// Wraps around the grid if necessary.
    ///
    pub fn relatives_expand_by_wrapped<'a, M: Movement>(
        &'a self,
        pos: Position,
        kernels: &'a [M],
        expand: usize,
    ) -> impl Iterator<Item = ((M, usize), Position, &'a T)> + 'a {
        pos.relatives_expand_by(kernels, expand)
            .map(move |(dir, pos)| (dir, pos, self.get_wrapped(pos)))
    }

    /// Like [Grid::relatives] but with `kernels` set to the four cardinal directions.
    pub fn adjacent(&self, pos: Position) -> impl Iterator<Item = (Direction, Position, &T)> {
        self.relatives(pos, &CARDINALS)
    }

    /// Like [Grid::relatives_wrapped] but with `kernels` set to the four cardinal directions.
    pub fn adjacent_wrapped(
        &self,
        pos: Position,
    ) -> impl Iterator<Item = (Direction, Position, &T)> {
        self.relatives_wrapped(pos, &CARDINALS)
    }

    /// Like [Grid::relatives_expand_by] but with `kernels` set to the four cardinal directions.
    pub fn adjacent_expand_by(
        &self,
        pos: Position,
        expand: usize,
    ) -> impl Iterator<Item = ((Direction, usize), Position, &T)> {
        self.relatives_expand_by(pos, &CARDINALS, expand)
    }

    /// Like [Grid::relatives_expand_by_wrapped] but with `kernels` set to the four cardinal directions.
    pub fn adjacent_expand_by_wrapped(
        &self,
        pos: Position,
        expand: usize,
    ) -> impl Iterator<Item = ((Direction, usize), Position, &T)> {
        self.relatives_expand_by_wrapped(pos, &CARDINALS, expand)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Grid");
        for (y, row) in self.data.iter().enumerate() {
            debug.field(&format!("row_{}", y), row);
        }
        debug.finish()
    }
}

/// Utilities for making tiles of a grid.
pub mod tiles {
    use crate::{dir::Movement, pos::Position};

    use super::Grid;

    #[macro_export]
    /// Create an enum that implements `From<char>`.
    ///
    /// There are three versions of this macro:
    ///
    /// ## 1. Simple
    ///
    /// Create a simple enum that implements `From<char>`, with the specific characters mapping to specific variants.
    /// Also will make the implementation panic if an invalid character is given.
    ///
    /// ```
    /// use utils::prelude::*;
    /// use utils::tiles;
    ///
    /// tiles!(Tile, [
    ///   '.' => Floor,
    ///   '#' => Wall,
    /// ]);
    ///
    /// assert_eq!(Tile::from('.'), Tile::Floor);
    /// assert_eq!(Tile::from('#'), Tile::Wall);
    /// ```
    ///
    /// ## 2. With Extra Variants
    ///
    /// Create an enum that implements `From<char>`, with the specific characters mapping to specific variants.
    /// Also allows for extra variants to be added, which won't be mapped to any characters.
    ///
    /// ```
    /// use utils::prelude::*;
    /// use utils::tiles;
    ///
    /// tiles!(Tile, [
    ///   '.' => Floor,
    ///   '#' => Wall,
    ///  ], [
    ///   Empty,
    ///  ]);
    ///
    /// assert_eq!(Tile::from('.'), Tile::Floor);
    /// assert_eq!(Tile::from('#'), Tile::Wall);
    /// let empty = Tile::Empty;
    /// ```
    ///
    /// The extra variants can also have fields.
    ///
    /// ```
    /// use utils::prelude::*;
    /// use utils::tiles;
    ///
    /// tiles!(Tile, [
    ///   '.' => Floor,
    ///   '#' => Wall,
    ///  ], [
    ///   Door(bool),
    /// ]);
    ///
    /// assert_eq!(Tile::from('.'), Tile::Floor);
    /// assert_eq!(Tile::from('#'), Tile::Wall);
    /// let door = Tile::Door(true);
    /// ```
    ///
    /// ## 3. With Extra Variants and Extra Logic for Invalid Characters
    ///
    /// Create an enum that implements `From<char>`, with the specific characters mapping to specific variants.
    /// Also allows for extra variants to be added, which won't be mapped to any characters.
    /// Also allows for extra logic to be added for invalid characters.
    ///
    /// ```
    /// use utils::prelude::*;
    /// use utils::tiles;
    ///
    /// tiles!(Tile, [
    ///   '.' => Floor,
    ///   '#' => Wall,
    ///  ], [
    ///   Slope(Direction)
    /// ], |c| {
    ///  match c {
    ///   '>' => Tile::Slope(Direction::East),
    ///   '<' => Tile::Slope(Direction::West),
    ///   _ => panic!("Invalid tile {c}"),
    /// }
    /// });
    ///
    /// assert_eq!(Tile::from('.'), Tile::Floor);
    /// assert_eq!(Tile::from('#'), Tile::Wall);
    /// assert_eq!(Tile::from('>'), Tile::Slope(Direction::East));
    /// ```
    ///
    macro_rules! tiles {
        ($name:ident, [$($char:pat => $v_name:ident$(,)?)*]) => {
            tiles!($name, [$($char => $v_name,)*], [], |c| { panic!("Invalid tile {c}") });
        };

        ($name:ident, [$($char:pat => $v_name:ident$(,)?)*], [$($e_name:ident$(($($i_name:ty$(,)?)*))?$(,)?)*]) => {
            tiles!($name, [$($char => $v_name,)*], [$($e_name$(($($i_name,)*))?,)*], |c| { panic!("Invalid tile {c}") });
        };

        ($name:ident, [$($char:pat => $v_name:ident$(,)?)*], [$($e_name:ident$(($($i_name:ty$(,)?)*))?$(,)?)*], $default:expr) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum $name {
                $($v_name,)*
                $($e_name$(($($i_name,)*))?,)*
            }

            impl From<char> for $name {
                fn from(c: char) -> Self {
                    match c {
                        $($char => Self::$v_name,)*
                        _ => ($default)(c),
                    }
                }
            }
        };
    }

    /// Simple tile that holds a number value.
    pub struct NumberTile {
        pub value: isize,
    }

    impl From<char> for NumberTile {
        fn from(c: char) -> Self {
            Self {
                value: c.to_digit(10).unwrap() as isize,
            }
        }
    }

    /// A tile that represents some kind of movement to another position in the grid.
    pub trait DirectedTile<T: Movement>: Copy + Clone {
        /// Get the next direction from the previous direction and position.
        fn next_dir(&self, previous_dir: T, pos: Position) -> Option<T>;

        /// Get the next position and position from the previous direction and position.
        fn next_pos(&self, previous_dir: T, pos: Position) -> Option<(T, Position)> {
            self.next_dir(previous_dir, pos)
                .map(|d| (d, pos.move_dir(d)))
        }
    }

    /// A tile that can be used in a flood fill.
    pub trait FillableTile: Copy + Clone {
        /// Check if the tile can be filled.
        fn get_next_tiles(&self, pos: Position, grid: &Grid<Self>) -> Vec<Position>;
    }
}

/// Utilities for traversing a grid.
pub mod cursors {

    use std::{
        collections::{HashSet, VecDeque},
        hash::Hasher,
    };

    use super::{
        tiles::{DirectedTile, FillableTile},
        *,
    };

    #[derive(Clone, Copy)]
    /// A cursor for traversing a grid.
    ///
    /// This cursor holds a position and a direction which represents the current position in the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let data = vec![
    ///  vec![1, 2, 3],
    ///  vec![4, 5, 6],
    ///  vec![7, 8, 9],
    /// ];
    ///
    /// let grid = Grid::new(data);
    ///
    /// let mut cursor = GridCursor::zero(&grid);
    ///
    /// assert_eq!(cursor.get(), Some(&1));
    /// cursor.move_forward();
    /// assert_eq!(cursor.get(), Some(&2));
    /// cursor.turn(true);
    /// cursor.move_forward();
    /// assert_eq!(cursor.get(), Some(&5));
    /// ```
    ///
    pub struct GridCursor<'a, T, D: Movement> {
        grid: &'a Grid<T>,
        pos: Position,
        dir: D,
    }

    impl<'a, T> GridCursor<'a, T, Direction> {
        /// Create a new cursor at position (0, 0) facing east.
        pub fn zero(grid: &'a Grid<T>) -> Self {
            Self {
                grid,
                pos: Position::new(0, 0),
                dir: Direction::East,
            }
        }

        /// Turn the cursor 90 degrees clockwise or counter-clockwise.
        pub fn turn(&mut self, clockwise: bool) {
            self.dir = self.dir.ninety_deg(clockwise);
        }

        /// Turn the cursor 180 degrees.
        pub fn turn_around(&mut self) {
            self.dir = self.dir.opposite();
        }
    }

    impl<T, D: Movement> PartialEq for GridCursor<'_, T, D> {
        fn eq(&self, other: &Self) -> bool {
            self.pos == other.pos && self.dir == other.dir
        }
    }

    impl<T, D: Movement> std::hash::Hash for GridCursor<'_, T, D> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.pos.hash(state);
            self.dir.hash(state);
        }
    }

    impl<'a, T, D: Movement> GridCursor<'a, T, D> {
        /// Create a new cursor at the given position and direction.
        pub fn new(grid: &'a Grid<T>, pos: Position, dir: D) -> Self {
            Self { grid, pos, dir }
        }

        /// Move the cursor forward one step in the direction it is facing.
        pub fn move_forward(&mut self) {
            self.pos = self.pos.move_dir(self.dir);
        }

        /// Get the value at the current position of the cursor.
        pub fn get(&self) -> Option<&T> {
            self.grid.get(self.pos)
        }

        /// Move the cursor forward one step in the direction it is facing and get the value at the new position.
        pub fn advance_get(&mut self) -> Option<&T> {
            self.move_forward();
            self.get()
        }
    }

    impl<T: std::fmt::Debug, D: Movement> std::fmt::Debug for GridCursor<'_, T, D> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("GridCursor")
                .field("pos", &self.pos)
                .field("dir", &self.dir)
                .field("value", &self.get())
                .finish()
        }
    }

    /// A cursor for traversing a grid with a direction.
    ///
    /// This cursor will follow the direction of the tile it is currently on.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    /// use utils::tiles;
    ///
    /// tiles!(Tile, [
    ///  '>' => Right,
    ///  '<' => Left,
    ///  '^' => Up,
    ///  'v' => Down,
    /// ]);
    ///
    /// impl DirectedTile<Direction> for Tile {
    ///     fn next_dir(&self, previous_dir: Direction, pos: Position) -> Option<Direction> {
    ///         match self {
    ///             Tile::Right => Some(Direction::East),
    ///             Tile::Left => Some(Direction::West),
    ///             Tile::Up => Some(Direction::North),
    ///             Tile::Down => Some(Direction::South),
    ///             _ => None,
    ///         }
    ///     }
    /// }
    ///
    /// let data = vec![
    ///     vec![Tile::Right, Tile::Right, Tile::Down],
    ///     vec![Tile::Up, Tile::Left, Tile::Down],
    ///     vec![Tile::Up, Tile::Left, Tile::Left],
    /// ];
    ///
    /// let grid = Grid::new(data);
    ///
    /// let mut cursor = DirectedCursor::new(&grid, Position::new(0, 0), Direction::East);
    ///
    /// let path = cursor.map(|(p, _, _)| p).take(8).collect::<Vec<_>>();
    ///
    /// assert_eq!(path, vec![
    ///    Position::new(1, 0),
    ///    Position::new(2, 0),
    ///    Position::new(2, 1),
    ///    Position::new(2, 2),
    ///    Position::new(1, 2),
    ///    Position::new(0, 2),
    ///    Position::new(0, 1),
    ///    Position::new(0, 0),
    /// ]);
    /// ```
    ///
    pub struct DirectedCursor<'a, T: DirectedTile<D>, D: Movement>(GridCursor<'a, T, D>);

    impl<'a, T: DirectedTile<D>, D: Movement> DirectedCursor<'a, T, D> {
        /// Create a new cursor at the given position and direction.
        /// Note this starting position will *not* be included in the iterator.
        pub fn new(grid: &'a Grid<T>, pos: Position, dir: D) -> Self {
            let initial_cursor = GridCursor::new(grid, pos, dir);
            Self(initial_cursor)
        }
    }

    impl<T: DirectedTile<D>, D: Movement> Iterator for DirectedCursor<'_, T, D> {
        type Item = (Position, D, T);

        fn next(&mut self) -> Option<Self::Item> {
            let current_val = self.0.get().cloned();
            current_val.and_then(|tile| {
                tile.next_pos(self.0.dir, self.0.pos).map(|(dir, pos)| {
                    self.0.dir = dir;
                    self.0.pos = pos;
                    (self.0.pos, self.0.dir, tile)
                })
            })
        }
    }

    /// A cursor that flood fills a grid.
    ///
    /// This cursor will flood fill the grid from the given position,
    /// using [FillableTile::get_next_tiles] to determine which tiles to fill.
    ///
    /// Setting `wrapped` to true will make the cursor wrap around the grid if necessary.
    /// Note this can lead to infinite loops if you don't have something to stop the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    /// use utils::tiles;
    ///
    /// tiles!(Tile, [
    /// '.' => Floor,
    /// '#' => Wall,
    /// ]);
    ///
    /// impl FillableTile for Tile {
    ///    fn get_next_tiles(&self, pos: Position, grid: &Grid<Self>) -> Vec<Position> {
    ///      match self {
    ///         Tile::Floor => grid.adjacent(pos).filter(|(_, _, t)| t == &&Tile::Floor).map(|(_, p, _)| p).collect(),
    ///         _ => vec![],
    ///      }
    ///    }
    /// }
    ///
    /// let data = vec![
    ///   vec![Tile::Floor, Tile::Floor, Tile::Floor],
    ///   vec![Tile::Floor, Tile::Wall, Tile::Floor],
    ///   vec![Tile::Floor, Tile::Floor, Tile::Wall],
    /// ];
    ///
    /// let grid = Grid::new(data);
    ///
    /// let mut cursor = FloodFillCursor::new(&grid, Position::new(0, 0), true);
    ///
    /// let path = cursor.collect::<Vec<_>>();
    ///
    /// assert_eq!(path, vec![
    ///   Position::new(0, 0),
    ///   Position::new(0, 1),
    ///   Position::new(1, 0),
    ///   Position::new(0, 2),
    ///   Position::new(2, 0),
    ///   Position::new(1, 2),
    ///   Position::new(2, 1),
    /// ]);
    /// ```
    ///
    pub struct FloodFillCursor<'a, T: FillableTile> {
        grid: &'a Grid<T>,
        visited: HashSet<Position>,
        queue: VecDeque<Position>,
        wrapped: bool,
    }

    impl<'a, T: FillableTile> FloodFillCursor<'a, T> {
        /// Create a new cursor at the given position.
        pub fn new(grid: &'a Grid<T>, pos: Position, wrapped: bool) -> Self {
            let mut visited = HashSet::new();
            visited.insert(pos);
            let mut queue = VecDeque::new();
            queue.push_back(pos);
            Self {
                grid,
                visited,
                queue,
                wrapped,
            }
        }
    }

    impl<T: FillableTile> std::fmt::Debug for FloodFillCursor<'_, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("FloodFillCursor")
                .field("visited", &self.visited)
                .field("queue", &self.queue)
                .finish()
        }
    }

    impl<T: FillableTile> Iterator for FloodFillCursor<'_, T> {
        type Item = Position;

        fn next(&mut self) -> Option<Self::Item> {
            let pos = self.queue.pop_front()?;
            let tile = if self.wrapped {
                self.grid.get_wrapped(pos)
            } else {
                self.grid.get(pos)?
            };
            for next_pos in tile.get_next_tiles(pos, self.grid) {
                if self.visited.insert(next_pos) {
                    self.queue.push_back(next_pos);
                }
            }
            Some(pos)
        }
    }
}
