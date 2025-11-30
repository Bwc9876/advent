/// Gets the number of digits in a given number mathematically
///
/// # Examples
///
/// ```
/// use utils::prelude::*;
///
/// let my_num = 2226;
/// assert_eq!(num_digits(my_num), 4);
/// ```
///
/// ```
/// use utils::prelude::*;
///
/// let my_num = 120938747;
/// assert_eq!(num_digits(my_num), 9);
/// ```
///
/// ```
/// use utils::prelude::*;
///
/// assert_eq!(num_digits(0), 1);
/// ```
///
pub fn num_digits(num: usize) -> usize {
    num.checked_ilog10().map(|x| x as usize).unwrap_or(0) + 1
}

/// Split a given number at a specific digit, this digit will be included in the right-hand side
/// and excluded in the left.
///
/// If the split is invalid, zero may be returned on either side of the result.
///
/// # Examples
///
/// ```
/// use utils::prelude::*;
///
/// let my_num = 123456;
/// let (left, right) = split_num_at(my_num, 3);
/// assert_eq!(left, 123);
/// assert_eq!(right, 456);
/// ```
///
pub fn split_num_at(num: usize, idx: u32) -> (usize, usize) {
    let div = 10_usize.pow(idx);
    (num / div, num % div)
}

/// Get the digit at the (starting from the right) given index
pub fn digit_at(num: usize, idx: usize) -> usize {
    let div = 10_usize.pow(idx as u32);
    num / div % 10
}

/// Iterator (from right to left)
struct Digits {
    num: usize,
    size: usize,
    curr_digit: usize,
}

impl Digits {
    pub fn new(num: usize) -> Self {
        Self {
            num,
            size: num_digits(num),
            curr_digit: 0,
        }
    }
}

impl Iterator for Digits {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_digit == self.size - 1 {
            None
        } else {
            let res = Some(digit_at(self.num, self.curr_digit));
            self.curr_digit += 1;
            res
        }
    }
}

/// Split the given number once in the middle, see [[split_num_at]] for caveats.
///
/// # Examples
///
/// ```
/// use utils::prelude::*;
///
/// let my_num = 55556666;
/// let (left, right) = split_num_once(my_num);
/// assert_eq!(left, 5555);
/// assert_eq!(right, 6666);
/// ```
///
pub fn split_num_once(num: usize) -> (usize, usize) {
    let digits = num_digits(num);
    split_num_at(num, (digits / 2) as u32)
}
