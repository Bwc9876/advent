use std::{fmt::Debug, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        self.start <= other.start && other.end < self.end
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

impl<T: Copy + Clone + Debug + Ord> Into<Range<T>> for BetterRange<T> {
    fn into(self) -> Range<T> {
        self.start..self.end
    }
}

impl<T: Copy + Clone + Debug + Ord> From<Range<T>> for BetterRange<T> {
    fn from(range: Range<T>) -> Self {
        Self::new(range.start, range.end)
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

impl<T: Copy + Clone + Debug + Ord> std::ops::BitAnd for BetterRange<T>
where
    T: std::ops::BitAnd<Output = T>,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.start.max(rhs.start), self.end.min(rhs.end))
    }
}

impl<T: Copy + Clone + Debug + Ord> std::ops::BitOr for BetterRange<T>
where
    T: std::ops::BitOr<Output = T>,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.start.min(rhs.start), self.end.max(rhs.end))
    }
}
