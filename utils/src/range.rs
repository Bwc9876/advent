use std::{fmt::Debug, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
/// Represents a range of values.
///
/// End is exclusive.
pub struct BetterRange<T: Copy + Clone + Debug> {
    pub start: T,
    pub end: T,
}

impl<T: Copy + Clone + Debug> BetterRange<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

pub enum RangeSplitBehavior {
    IncludeLower,
    IncludeUpper,
    Exclude,
}

impl<T: Copy + Clone + Debug + Ord> BetterRange<T> {
    /// Checks if the range contains the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let range = BetterRange::new(0, 10);
    /// assert!(range.contains(&5));
    /// assert!(!range.contains(&10));
    /// ```
    ///
    pub fn contains(&self, value: &T) -> bool {
        self.start <= *value && *value < self.end
    }

    /// Checks if the range contains the given range.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let range = BetterRange::new(0, 10);
    /// assert!(range.contains_range(&BetterRange::new(5, 7)));
    /// assert!(!range.contains_range(&BetterRange::new(5, 15)));
    /// ```
    ///
    pub fn contains_range(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Merge this range with another. If the ranges do not intersect, [None] is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let a = BetterRange::new(1, 5);
    /// let b = BetterRange::new(6, 11);
    /// let c = BetterRange::new(4, 8);
    /// let d = BetterRange::new(3, 4);
    ///
    ///
    /// assert!(a.merge(&b).is_none(), "Should not merge");
    /// assert!(d.merge(&b).is_none(), "Should not merge");
    /// assert_eq!(a.merge(&c).unwrap(), BetterRange::new(1, 8), "1-5 & 4-8 = 1-8");
    /// assert_eq!(c.merge(&a).unwrap(), BetterRange::new(1, 8), "4-8 & 1-5 = 1-8");
    /// assert_eq!(a.merge(&d).unwrap(), a, "1-5 & 3-4 = 1-5");
    /// assert_eq!(d.merge(&a).unwrap(), a, "3-4 & 1-5 = 1-5");
    /// assert_eq!(c.merge(&d).unwrap(), BetterRange::new(3, 8), "4-8 & 3-4 = 3-8");
    /// assert_eq!(d.merge(&c).unwrap(), BetterRange::new(3, 8), "3-4 & 4-8 = 3-8");
    /// ```
    pub fn merge(&self, other: &Self) -> Option<Self> {
        if self.contains_range(other) {
            Some(*self)
        } else if other.contains_range(self) {
            Some(*other)
        } else if self.contains(&other.start) || self.end == other.start {
            Some(Self::new(self.start, other.end))
        } else if self.contains(&other.end) || other.end == self.start {
            Some(Self::new(other.start, self.end))
        } else {
            None
        }
    }

    /// Split the range at the given value.
    ///
    /// The behaviour determines if the value is included in the lower range, upper range, or neither.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let range = BetterRange::new(0, 10);
    ///
    /// let (lower, upper) = range.split(&5, RangeSplitBehavior::IncludeLower);
    /// assert_eq!(lower, Some(BetterRange::new(0, 6)));
    /// assert_eq!(upper, Some(BetterRange::new(6, 10)));
    /// ```
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let range = BetterRange::new(0, 10);
    ///
    /// let (lower, upper) = range.split(&5, RangeSplitBehavior::IncludeUpper);
    /// assert_eq!(lower, Some(BetterRange::new(0, 5)));
    /// assert_eq!(upper, Some(BetterRange::new(5, 10)));
    /// ```
    ///
    /// ```
    /// use utils::prelude::*;
    ///
    /// let range = BetterRange::new(0, 10);
    ///
    /// let (lower, upper) = range.split(&5, RangeSplitBehavior::Exclude);
    /// assert_eq!(lower, Some(BetterRange::new(0, 5)));
    /// assert_eq!(upper, Some(BetterRange::new(6, 10)));
    /// ```
    ///
    pub fn split(&self, value: &T, behaviour: RangeSplitBehavior) -> (Option<Self>, Option<Self>)
    where
        T: std::ops::Add<usize, Output = T> + std::ops::Sub<usize, Output = T>,
    {
        if self.contains(value) {
            match behaviour {
                RangeSplitBehavior::IncludeLower => (
                    Some(Self::new(self.start, *value + 1)),
                    Some(Self::new(*value + 1, self.end)),
                ),
                RangeSplitBehavior::IncludeUpper => (
                    Some(Self::new(self.start, *value)),
                    Some(Self::new(*value, self.end)),
                ),
                RangeSplitBehavior::Exclude => (
                    Some(Self::new(self.start, *value)),
                    Some(Self::new(*value + 1, self.end)),
                ),
            }
        } else {
            (None, None)
        }
    }
}

impl<T: Copy + Clone + Debug + Ord> From<BetterRange<T>> for Range<T> {
    fn from(val: BetterRange<T>) -> Self {
        val.start..val.end
    }
}

impl<T: Copy + Clone + Debug + Ord> From<Range<T>> for BetterRange<T> {
    fn from(val: Range<T>) -> Self {
        BetterRange::new(val.start, val.end)
    }
}

impl<T: Copy + Clone + Debug + Ord> std::ops::Add<usize> for BetterRange<T>
where
    T: std::ops::Add<usize, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.start + rhs, self.end + rhs)
    }
}

impl<T: Copy + Clone + Debug + Ord> std::ops::Sub<usize> for BetterRange<T>
where
    T: std::ops::Sub<usize, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::new(self.start - rhs, self.end - rhs)
    }
}
