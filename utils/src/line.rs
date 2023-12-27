use std::ops::Neg;

use crate::{dir::Direction, pos::Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A line between two points.
///
/// This line is represented by two [Position]s.
///
/// # Examples
///
/// ```
/// use utils::prelude::*;
///
/// let line = Line::new(Position::new(0, 0), Position::new(1, 1));
/// assert_eq!(line.get_slope(), 1.0);
/// assert_eq!(line.get_intercept(), 0.0);
/// ```
///
pub struct Line(Position, Position);

impl Line {
    /// Create a new line between two points.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let line = Line::new(Position::new(0, 0), Position::new(1, 1));
    /// assert_eq!(line.end().x, 1);
    /// ```
    ///
    pub fn new(start: Position, end: Position) -> Self {
        Self(start, end)
    }

    /// Create a new line from a starting point and a direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let line = Line::from_dir(Position::new(0, 0), Direction::East);
    /// assert_eq!(line.end().x, 1);
    /// ```
    ///
    pub fn from_dir(start: Position, dir: Direction) -> Self {
        Self(start, start.move_dir(dir))
    }

    /// Get the linear slope of the line.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let line = Line::new(Position::new(0, 0), Position::new(1, 1));
    /// assert_eq!(line.get_slope(), 1.0);
    ///
    /// let line = Line::new(Position::new(0, 0), Position::new(2, 1));
    /// assert_eq!(line.get_slope(), 0.5);
    /// ```
    ///
    pub fn get_slope(&self) -> f64 {
        let dx = self.1.x - self.0.x;
        let dy = self.1.y - self.0.y;
        dy as f64 / dx as f64
    }

    /// Get the y-intercept of the line.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let line = Line::new(Position::new(0, 0), Position::new(1, 1));
    /// assert_eq!(line.get_intercept(), 0.0);
    ///
    /// let line = Line::new(Position::new(0, 5), Position::new(2, 1));
    /// assert_eq!(line.get_intercept(), 5.0);
    /// ```
    ///
    pub fn get_intercept(&self) -> f64 {
        let slope = self.get_slope();
        self.0.y as f64 - slope * self.0.x as f64
    }

    /// Check that the given point is *after* the start position on the line.
    ///
    /// Note this doesn't check if the point is on the line, just that it is
    /// on the same side of the line as the end point.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let line = Line::new(Position::new(0, 0), Position::new(2, 2));
    /// assert_eq!(line.check_after(&Position::new(1, 1)), true);
    ///
    /// let line = Line::new(Position::new(0, 0), Position::new(2, 2));
    /// assert_eq!(line.check_after(&Position::new(-1, -1)), false);
    /// ```
    ///
    pub fn check_after(&self, pos: &Position) -> bool {
        let relative = pos.sub(&self.0);
        let d = self.1.sub(&self.0);
        relative.normalize() == d.normalize()
    }

    /// Get the intersection point between this line and another.
    ///
    /// Pass `check_after` as `true` to ensure that the intersection point is
    /// after the start of both lines.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let line1 = Line::new(Position::new(2, -2), Position::new(-2, 2));
    /// let line2 = Line::new(Position::new(2, 2), Position::new(-2, -2));
    /// assert_eq!(line1.get_intersection(&line2, true), Some(Position::new(0, 0)));
    ///
    /// let line1 = Line::new(Position::new(5, 0), Position::new(6, 8));
    /// let line2 = Line::new(Position::new(0, 1), Position::new(-4, -3));
    /// assert_eq!(line1.get_intersection(&line2, true), None);
    /// ```
    ///
    pub fn get_intersection(&self, other: &Self, check_after: bool) -> Option<Position> {
        let slope = self.get_slope();
        let intercept = self.get_intercept();
        let other_slope = other.get_slope();
        let other_intercept = other.get_intercept();

        if slope == other_slope {
            return None;
        }

        let x = (other_intercept - intercept) / (slope - other_slope);
        let y = slope * x + intercept;

        let point = Position::new(x as isize, y as isize);

        if !check_after || self.check_after(&point) && other.check_after(&point) {
            Some(point)
        } else {
            None
        }
    }

    pub fn start(&self) -> Position {
        self.0
    }

    pub fn end(&self) -> Position {
        self.1
    }
}

impl Neg for Line {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.1, self.0)
    }
}
