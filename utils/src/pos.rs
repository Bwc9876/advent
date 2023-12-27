/// Position utilities
use std::{
    fmt::{Debug, Display},
    ops::{Add, Mul, Neg, Sub},
    str::FromStr,
};

use crate::dir::{Direction, Movement, CARDINALS};

type CompType = isize;
type PositiveType = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A position in 2D space on an integer grid
/// This is meant to represent indices of a 2D array, so north is negative y
///
/// This is also used to represent a vector in 2D space at times
pub struct Position {
    pub x: CompType,
    pub y: CompType,
}

impl Position {
    /// Create a new position
    pub fn new(x: CompType, y: CompType) -> Self {
        Self { x, y }
    }

    /// Create a new position at 0, 0
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Create the unit vector position
    pub fn one() -> Self {
        Self { x: 1, y: 1 }
    }

    /// Get the position flipped over the line y = x
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let flipped = Position::new(1, 2).flip();
    /// assert_eq!(flipped, Position::new(2, 1));
    /// ```
    ///
    pub fn flip(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    /// Normalize a position to a unit vector
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let normalized = Position::new(1, 1).normalize();
    /// assert_eq!(normalized, Position::new(1, 1));
    ///
    /// let normalized = Position::new(50, -45).normalize();
    /// assert_eq!(normalized, Position::new(1, -1));
    ///
    /// let normalized = Position::new(-30, 0).normalize();
    /// assert_eq!(normalized, Position::new(-1, 0));
    /// ```
    ///
    pub fn normalize(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    /// Get the magnitude of a position
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let mag = Position::new(0, 1).magnitude();
    /// assert_eq!(mag, 1.0);
    ///
    /// let mag = Position::new(3, 4).magnitude();
    /// assert_eq!(mag, 5.0);
    /// ```
    ///
    pub fn magnitude(&self) -> f64 {
        (((self.x * self.x) + (self.y * self.y)) as f64).sqrt()
    }

    /// Get the absolute value of a position
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let abs = Position::new(-1, -1).abs();
    /// assert_eq!(abs, Position::new(1, 1));
    /// ```
    ///
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Sum the components of a position
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let sum = Position::new(1, 1).sum();
    /// assert_eq!(sum, 2);
    /// ```
    ///
    pub fn sum(&self) -> CompType {
        self.x + self.y
    }

    /// Get the difference between the components of a position
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let diff = Position::new(1, 1).diff();
    /// assert_eq!(diff, 0);
    /// ```
    ///
    pub fn diff(&self) -> CompType {
        self.x - self.y
    }

    /// Get the direction of one position relative to another
    ///
    /// This is the direction that the second position is from the first
    ///
    /// Meaning
    ///
    /// ```txt
    /// ...
    /// A.B
    /// ...
    ///
    /// A.get_dir(B) == Direction::East
    /// and
    /// B.get_dir(A) == Direction::West
    /// ```
    ///
    /// # Panics
    ///
    /// If the positions are the same or diagonal from each other
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let dir = Position::new(1, 1).get_dir(&Position::new(5, 1));
    /// assert_eq!(dir, Direction::East);
    ///
    /// let dir = Position::new(5, 1).get_dir(&Position::new(1, 1));
    /// assert_eq!(dir, Direction::West);
    ///
    /// let dir = Position::new(1, 1).get_dir(&Position::new(1, 5));
    /// assert_eq!(dir, Direction::South);
    /// ```
    ///
    pub fn get_dir(&self, other: &Self) -> Direction {
        other.sub(self).normalize().into()
    }

    /// Add two positions together
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let sum = Position::new(1, 1).add(&Position::new(5, 4));
    /// assert_eq!(sum, Position::new(6, 5));
    /// ```
    ///
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Get the difference between two positions
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let diff = Position::new(1, 1).sub(&Position::new(5, 5));
    /// assert_eq!(diff, Position::new(-4, -4));
    /// ```
    ///
    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Multiply two positions together
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let multiplied = Position::new(1, 1).multiply(&Position::new(5, 4));
    /// assert_eq!(multiplied, Position::new(5, 4));
    /// ```
    ///
    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    /// Get the dot product of two positions
    ///
    /// x1 * x2 + y1 * y2
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let dot = Position::new(1, 1).dot(&Position::new(5, 4));
    /// assert_eq!(dot, 9);
    /// ```
    ///
    pub fn dot(&self, other: &Self) -> CompType {
        self.multiply(other).sum()
    }

    /// Get the angle between two positions
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let angle = Position::new(0, 1).angle(&Position::new(1, 0));
    ///
    /// assert_eq!(angle, std::f64::consts::FRAC_PI_2);
    /// ```
    ///
    pub fn angle(&self, other: &Self) -> f64 {
        let dot = self.dot(other) as f64;
        let mag = self.magnitude() * other.magnitude();
        (dot / mag).acos()
    }

    /// Get the cross product of two positions
    ///
    /// x1 * y2 - y1 * x2
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let cross = Position::new(2, 3).cross(&Position::new(5, 4));
    /// assert_eq!(cross, -7);
    /// ```
    ///
    pub fn cross(&self, other: &Self) -> CompType {
        self.multiply(&other.flip()).diff()
    }

    /// Multiply a position by a scalar
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let multiplied = Position::new(1, 1).multiply_comp(5);
    /// assert_eq!(multiplied, Position::new(5, 5));
    /// ```
    ///
    pub fn multiply_comp(&self, other: CompType) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }

    /// Get the manhattan distance between two positions
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let distance = Position::new(1, 1).manhattan(&Position::new(5, 5));
    /// assert_eq!(distance, 8);
    /// ```
    ///
    pub fn manhattan(&self, other: &Self) -> CompType {
        self.sub(other).abs().sum()
    }

    /// Get the chebyshev distance between two positions
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let distance = Position::new(1, 1).chebyshev(&Position::new(5, 5));
    /// assert_eq!(distance, 4);
    /// ```
    ///
    pub fn chebyshev(&self, other: &Self) -> CompType {
        let diff = self.sub(other).abs();
        diff.x.max(diff.y)
    }

    /// Check if a component is within a range of 0..bound
    /// Note bound is exclusive
    fn check_comp(comp: CompType, bound: usize) -> bool {
        (0..(bound as isize)).contains(&comp)
    }

    /// Check if a position is within a range of (0..bound.0, 0..bound.1)
    /// Note bound is exclusive
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let checked = Position::new(0, 0).check((10, 10));
    /// assert!(checked);
    ///
    /// let checked = Position::new(50, 50).check((5, 5));
    /// assert_eq!(checked, false);
    /// ```
    ///
    pub fn check(&self, bounds: PositiveType) -> bool {
        Self::check_comp(self.x, bounds.0) && Self::check_comp(self.y, bounds.1)
    }

    /// Normalize a value to be within a range of 0..bound
    /// Note bound is exclusive
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let normalized = Position::bind_comp(-1, 10);
    /// assert_eq!(normalized, 9);
    ///
    /// let normalized = Position::bind_comp(-10, 5);
    /// assert_eq!(normalized, 0);
    ///
    /// let normalized = Position::bind_comp(10, 5);
    /// assert_eq!(normalized, 0);
    ///
    /// let normalized = Position::bind_comp(3, 6);
    /// assert_eq!(normalized, 3);
    /// ```
    ///
    pub fn bind_comp(comp: CompType, bound: usize) -> usize {
        let bound = bound as isize;
        if comp >= bound || comp.is_negative() {
            let ans = comp % bound;
            if ans.is_negative() {
                (ans + bound) as usize
            } else {
                ans as usize
            }
        } else {
            comp as usize
        }
    }

    /// Bind a position to be within ranges of (0..bound.0, 0..bound.1)
    /// Note bound is exclusive
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let bound = Position::new(-1, -1).bind((11, 11));
    /// assert_eq!(bound, (10, 10));
    ///
    /// let bound = Position::new(-10, -10).bind((6, 6));
    /// assert_eq!(bound, (2, 2));
    ///
    /// let bound = Position::new(10, 10).bind((6, 6));
    /// assert_eq!(bound, (4, 4));
    /// ```
    ///
    pub fn bind(&self, bounds: PositiveType) -> PositiveType {
        (
            Self::bind_comp(self.x, bounds.0),
            Self::bind_comp(self.y, bounds.1),
        )
    }

    /// Move a position by direction
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let moved = Position::new(0, 0).move_dir(Direction::North);
    /// assert_eq!(moved, Position::new(0, -1));
    ///
    /// let moved = Position::new(0, 0).move_dir(Direction::East);
    /// assert_eq!(moved, Position::new(1, 0));
    /// ```
    ///
    pub fn move_dir(&self, dir: impl Movement) -> Self {
        self.add(&dir.get_kernel())
    }

    /// Move a position by direction a certain number of times
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let moved = Position::new(0, 0).move_times(Direction::North, 5);
    /// assert_eq!(moved, Position::new(0, -5));
    /// ```
    ///
    pub fn move_times(&self, dir: impl Movement, times: usize) -> Self {
        self.add(&dir.get_kernel().multiply_comp(times as isize))
    }

    /// Move a position by direction,
    /// checking if it is within a range of (0..bound.0, 0..bound.1)
    ///
    /// # Returns
    ///
    /// * `Some(Position)` if the new position is within the bounds
    /// * `None` if the new position is outside the bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let moved = Position::new(0, 0).move_dir_checked(Direction::East, (10, 10));
    /// assert_eq!(moved.unwrap(), Position::new(1, 0));
    ///
    /// let moved = Position::new(40, 40).move_dir_checked(Direction::East, (40, 40));
    /// assert!(moved.is_none());
    /// ```
    ///
    pub fn move_dir_checked(&self, dir: impl Movement, bounds: PositiveType) -> Option<Self> {
        let new = self.move_dir(dir);
        if new.check(bounds) {
            Some(new)
        } else {
            None
        }
    }

    /// Move a position by direction a certain number of times,
    /// checking if it is within a range of (0..bound.0, 0..bound.1)
    ///
    /// # Returns
    ///
    /// * `Some(Position)` if the new position is within the bounds
    /// * `None` if the new position is outside the bounds
    ///
    pub fn move_times_checked(
        &self,
        dir: impl Movement,
        times: usize,
        bounds: PositiveType,
    ) -> Option<Self> {
        let new = self.move_times(dir, times);
        if new.check(bounds) {
            Some(new)
        } else {
            None
        }
    }

    /// Get all positions relative to this position by a list of directions
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let relatives = Position::new(0, 0).relatives(&[Direction::North, Direction::East]).collect::<Vec<_>>();
    /// assert_eq!(relatives, vec![(Position::new(0, -1), Direction::North), (Position::new(1, 0), Direction::East)]);
    /// ```
    ///
    pub fn relatives<'a, T: Movement>(
        self,
        kernels: &'a [T],
    ) -> impl Iterator<Item = (Self, T)> + 'a {
        kernels.into_iter().map(move |k| (self.move_dir(*k), *k))
    }

    /// Get all positions relative to this position by a list of directions,
    /// checking if they are within a range of (0..bound.0, 0..bound.1)
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let relatives = Position::new(0, 0).relatives_checked(&[Direction::North, Direction::East], (10, 10)).collect::<Vec<_>>();
    /// assert_eq!(relatives, vec![(Position::new(1, 0), Direction::East)]);
    /// ```
    ///
    pub fn relatives_checked<'a, T: Movement>(
        self,
        kernels: &'a [T],
        bounds: PositiveType,
    ) -> impl Iterator<Item = (Self, T)> + 'a {
        kernels
            .iter()
            .filter_map(move |k| self.move_dir_checked(*k, bounds).map(|p| (p, *k)))
    }

    /// Get all positions relative to this position by a list of directions,
    /// repeating each direction a certain number of times
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let relatives = Position::new(0, 0).relatives_expand_by(&[Direction::North, Direction::East], 2).collect::<Vec<_>>();
    /// let expected = vec![
    ///    ((Direction::North, 1), Position::new(0, -1)),
    ///    ((Direction::North, 2), Position::new(0, -2)),
    ///    ((Direction::East, 1), Position::new(1, 0)),
    ///    ((Direction::East, 2), Position::new(2, 0)),
    /// ];
    ///
    /// assert_eq!(relatives, expected);
    /// ```
    ///
    pub fn relatives_expand_by<'a, T: Movement>(
        self,
        kernels: &'a [T],
        times: usize,
    ) -> impl Iterator<Item = ((T, usize), Self)> + 'a {
        kernels
            .into_iter()
            .flat_map(move |k| (1..=times).map(move |t| ((*k, t), self.move_times(*k, t))))
    }

    /// Get all positions relative to this position by a list of directions,
    /// repeating each direction a certain number of times,
    /// checking if they are within a range of (0..bound.0, 0..bound.1)
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let relatives = Position::new(0, 0).relatives_expand_by_checked(&[Direction::North, Direction::East], 2, (10, 10)).collect::<Vec<_>>();
    /// let expected = vec![
    ///     ((Direction::East, 1), Position::new(1, 0)),
    ///     ((Direction::East, 2), Position::new(2, 0)),
    /// ];
    ///
    /// assert_eq!(relatives, expected);
    /// ```
    ///
    pub fn relatives_expand_by_checked<'a, T: Movement>(
        self,
        kernels: &'a [T],
        times: usize,
        bounds: PositiveType,
    ) -> impl Iterator<Item = ((T, usize), Self)> + 'a {
        kernels.into_iter().flat_map(move |k| {
            (1..=times)
                .filter_map(move |t| self.move_times_checked(*k, t, bounds).map(|p| ((*k, t), p)))
        })
    }

    /// Get all positions adjacent to this position
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let adjacents = Position::new(0, 0).adjacents().collect::<Vec<_>>();
    /// let expected = vec![
    ///    (Position::new(0, -1), Direction::North),
    ///    (Position::new(0, 1), Direction::South),
    ///    (Position::new(1, 0), Direction::East),
    ///    (Position::new(-1, 0), Direction::West),
    /// ];
    ///
    /// assert_eq!(adjacents, expected);
    /// ```
    ///
    pub fn adjacents(self) -> impl Iterator<Item = (Self, Direction)> {
        self.relatives(&CARDINALS)
    }

    /// Get all positions adjacent to this position
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let adjacents = Position::new(0, 0).adjacents_checked((2, 2)).collect::<Vec<_>>();
    /// let expected = vec![
    ///    (Position::new(0, 1), Direction::South),
    ///    (Position::new(1, 0), Direction::East),
    /// ];
    ///
    /// assert_eq!(adjacents, expected);
    /// ```
    ///
    pub fn adjacents_checked(
        self,
        bounds: PositiveType,
    ) -> impl Iterator<Item = (Self, Direction)> {
        self.relatives_checked(&CARDINALS, bounds)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::add(&self, &other)
    }
}

impl Add<&Position> for Position {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self::add(&self, other)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::sub(&self, &other)
    }
}

impl Sub<&Position> for Position {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self::sub(&self, other)
    }
}

impl Mul for Position {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::multiply(&self, &other)
    }
}

impl Mul<&Position> for Position {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        Self::multiply(&self, other)
    }
}

impl Mul<CompType> for Position {
    type Output = Self;

    fn mul(self, other: CompType) -> Self {
        Self::multiply_comp(&self, other)
    }
}

impl Mul<usize> for Position {
    type Output = Self;

    fn mul(self, other: usize) -> Self {
        Self::multiply_comp(&self, other as isize)
    }
}

impl Neg for Position {
    type Output = Self;

    fn neg(self) -> Self {
        self.multiply_comp(-1)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split.next().ok_or("No x")?.parse().expect("No x");
        let y = split.next().ok_or("No y")?.parse().expect("No y");
        Ok(Self { x, y })
    }
}

impl From<(CompType, CompType)> for Position {
    fn from((x, y): (CompType, CompType)) -> Self {
        Self { x, y }
    }
}

impl Into<(CompType, CompType)> for Position {
    fn into(self) -> (CompType, CompType) {
        (self.x, self.y)
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl Into<(usize, usize)> for Position {
    fn into(self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

impl From<Direction> for Position {
    fn from(dir: Direction) -> Self {
        dir.get_kernel()
    }
}
