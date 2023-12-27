/// Module containing utilities related to direction and movement.
use crate::pos::Position;

/// Trait used to define an object that can be used to move around a grid.
///
/// This is meant for complex scenarios where you want to move around a grid in a non-standard way.
/// By implementing this trait you can use various methods from the [Position] struct to move around.
///
/// # Implementing
///
/// Implementing this trait requires you to define a `get_kernel` method that returns a `Position`.
/// This position is used to move around the grid by applying it to the current position.
///
/// # Examples
///
/// ```
/// use utils::prelude::*;
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// struct RightBy(usize);
///
/// impl Movement for RightBy {
///    fn get_kernel(&self) -> Position {
///       Position::new(self.0 as isize, 0)
///    }
/// }
///
/// let pos = Position::new(0, 0);
/// assert_eq!(pos.move_dir(RightBy(1)), Position::new(1, 0));
/// ```
///
/// # See also
///
/// - [Direction] is a simple implementation of this trait.
/// - [Position] is the main user of this trait.
///
pub trait Movement: std::fmt::Debug + Copy + Clone + PartialEq + std::hash::Hash {
    fn get_kernel(&self) -> Position;
}

/// The four cardinal directions.
/// Useful for iterating over all four directions.
pub const CARDINALS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// The four cardinal directions.
/// This is a simple implementation of the [Movement] trait.
///
/// # Examples
///
/// ```
/// use utils::prelude::*;
///
/// let pos = Position::new(0, 0);
/// assert_eq!(pos.move_dir(Direction::North), Position::new(0, -1));
/// assert_eq!(pos.move_dir(Direction::South), Position::new(0, 1));
/// assert_eq!(pos.move_dir(Direction::East), Position::new(1, 0));
/// assert_eq!(pos.move_dir(Direction::West), Position::new(-1, 0));
/// ```
///
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Returns the direction that is opposite to the current one.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// assert_eq!(Direction::North.opposite(), Direction::South);
    /// assert_eq!(Direction::South.opposite(), Direction::North);
    /// assert_eq!(Direction::East.opposite(), Direction::West);
    /// assert_eq!(Direction::West.opposite(), Direction::East);
    /// ```
    ///
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    /// Returns the direction that is 90 degrees to the current one.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// assert_eq!(Direction::North.ninety_deg(true), Direction::East);
    /// assert_eq!(Direction::North.ninety_deg(false), Direction::West);
    ///
    /// assert_eq!(Direction::South.ninety_deg(true), Direction::West);
    /// assert_eq!(Direction::South.ninety_deg(false), Direction::East);
    /// ```
    ///
    pub fn ninety_deg(&self, clockwise: bool) -> Self {
        match (self, clockwise) {
            (Self::North, true) => Self::East,
            (Self::North, false) => Self::West,
            (Self::South, true) => Self::West,
            (Self::South, false) => Self::East,
            (Self::East, true) => Self::South,
            (Self::East, false) => Self::North,
            (Self::West, true) => Self::North,
            (Self::West, false) => Self::South,
        }
    }
}

impl From<Position> for Direction {
    fn from(pos: Position) -> Self {
        let pos = pos.normalize();
        match (pos.x, pos.y) {
            (0, -1) => Self::North,
            (0, 1) => Self::South,
            (1, 0) => Self::East,
            (-1, 0) => Self::West,
            _ => panic!("Invalid position"),
        }
    }
}

impl Movement for Direction {
    fn get_kernel(&self) -> Position {
        match self {
            Direction::North => Position::new(0, -1),
            Direction::South => Position::new(0, 1),
            Direction::East => Position::new(1, 0),
            Direction::West => Position::new(-1, 0),
        }
    }
}
