#![allow(dead_code)]
//! \# 307. Range Sum Query - Mutable
//! =================================
//!
//! <https://leetcode.com/problems/range-sum-query-mutable>
//!
//! Given an integer array `nums`, handle multiple queries of the following types:
//!
//! 1. __Update__ the value of an element in `nums`.
//! 2. Calculate the __sum__ of the elements of `nums` between indices `left` and `right` __inclusive__
//!    where `left <= right`.
//!
//! Implement the `NumArray` class:
//!
//! - `NumArray(int[] nums)` Initializes the object with the integer array `nums`.
//! - `void update(int index, int val)` __Updates__ the value of `nums[index]` to be `val`.
//! - `int sumRange(int left, int right)` Returns the __sum__ of the elements of `nums` between indices
//!   `left` and `right` __inclusive__ (i.e. `nums[left] + nums[left + 1] + ... + nums[right]`).
//!
//! __Constraints:__
//!
//! - `1 <= nums.length <= 30_000`
//! - `-100 <= nums[i] <= 100`
//! - `0 <= index < nums.length`
//! - `-100 <= val <= 100`
//! - `0 <= left <= right < nums.length`
//! - At most `30_000` calls will be made to `update` and `sumRange`.
//!
//! # Examples
//!
//! ```
//! # use c2022_07::c2022_07_31::v4::NumArray;
//! let num_array = NumArray::new(vec![1, 3, 5]);
//! assert_eq!(num_array.sum_range(0, 0), 1);
//! assert_eq!(num_array.sum_range(1, 1), 3);
//! assert_eq!(num_array.sum_range(2, 2), 5);
//! assert_eq!(num_array.sum_range(0, 1), 1 + 3);
//! assert_eq!(num_array.sum_range(1, 2), 3 + 5);
//! assert_eq!(num_array.sum_range(0, 2), 1 + 3 + 5);
//! num_array.update(1, 2);
//! assert_eq!(num_array.sum_range(0, 0), 1);
//! assert_eq!(num_array.sum_range(1, 1), 2);
//! assert_eq!(num_array.sum_range(2, 2), 5);
//! assert_eq!(num_array.sum_range(0, 1), 1 + 2);
//! assert_eq!(num_array.sum_range(1, 2), 2 + 5);
//! assert_eq!(num_array.sum_range(0, 2), 1 + 2 + 5);
//! ```
//!
//! ```
//! # use c2022_07::c2022_07_31::v4::NumArray;
//! let num_array = NumArray::new(vec![1; 30_000]);
//! for i in 0..30_000 {
//!     assert_eq!(num_array.sum_range(0, i), i + 1);
//! }
//! ```
//!
//! ```
//! # use c2022_07::c2022_07_31::v4::NumArray;
//! let num_array = NumArray::new(vec![1; 30_000]);
//! for i in 0..30_000 {
//!     num_array.update(i, 2);
//! }
//! assert_eq!(num_array.sum_range(0, 29_999), 60_000);
//! ```

/// Using a Segment Tree
///
/// - `O(n)` memory
/// - `O(n)` time `new`
/// - `O(log n)` time `update`
/// - `O(log n)` time `sum_range`
pub mod v4 {
    use std::cell::RefCell;

    pub struct NumArray {
        len: usize,
        segment_tree: RefCell<Vec<i32>>,
    }
    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            let len = nums.len();
            let max_size = 2usize.pow((len as f64).log2().ceil() as u32 + 1) - 1;
            let segment_tree = RefCell::new(vec![0; max_size]);
            let result = Self { len, segment_tree };

            result.fill_segment_tree(0, (0, len - 1), &nums);

            result
        }

        pub fn update(&self, index: i32, val: i32) {
            println!("update({index}, {val})");
            let curr = self.sum_range(index, index);
            let diff = val - curr;
            self.update_rec(0, (0, self.len - 1), index as usize, diff);
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            self.sum_range_rec(0, (0, self.len - 1), (left as usize, right as usize))
        }

        fn fill_segment_tree(&self, i: usize, segment: (usize, usize), nums: &[i32]) -> i32 {
            self.segment_tree.borrow_mut()[i] = if segment.0 == segment.1 {
                nums[segment.0]
            } else {
                let (l, r) = segment;
                let mid = l + (r - l) / 2;
                let l_sum = self.fill_segment_tree(i * 2 + 1, (l, mid), nums);
                let r_sum = self.fill_segment_tree(i * 2 + 2, (mid + 1, r), nums);
                l_sum + r_sum
            };
            self.segment_tree.borrow()[i]
        }

        fn sum_range_rec(&self, i: usize, segment: (usize, usize), query: (usize, usize)) -> i32 {
            if query.0 <= segment.0 && query.1 >= segment.1 {
                self.segment_tree.borrow()[i]
            } else if query.0 > segment.1 || query.1 < segment.0 {
                0
            } else {
                let (l, r) = segment;
                let mid = l + (r - l) / 2;
                let l_sum = self.sum_range_rec(i * 2 + 1, (l, mid), query);
                let r_sum = self.sum_range_rec(i * 2 + 2, (mid + 1, r), query);
                l_sum + r_sum
            }
        }

        fn update_rec(&self, i: usize, segment: (usize, usize), index: usize, diff: i32) {
            if index >= segment.0 && index <= segment.1 {
                self.segment_tree.borrow_mut()[i] += diff;
                if segment.0 != segment.1 {
                    let (l, r) = segment;
                    let mid = l + (r - l) / 2;
                    self.update_rec(i * 2 + 1, (l, mid), index, diff);
                    self.update_rec(i * 2 + 2, (mid + 1, r), index, diff);
                }
            }
        }
    }
}

/// Using a Binary Indexed Tree aka Fenwick Tree
///
/// - `O(n)` memory
/// - `O(n * log n)` time `new`
/// - `O(log n)` time `update`
/// - `O(log n)` time `sum_range`
pub mod v3 {
    use std::cell::RefCell;

    pub struct NumArray {
        bit: RefCell<Vec<i32>>,
    }
    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            let bit =
                nums.iter()
                    .enumerate()
                    .fold(vec![0; nums.len() + 1], |mut bit, (i, &num)| {
                        Self::add(&mut bit, i, num);
                        bit
                    });
            let bit = RefCell::new(bit);

            Self { bit }
        }

        pub fn update(&self, index: i32, val: i32) {
            let diff = val - self.sum_range(index, index);
            Self::add(&mut self.bit.borrow_mut(), index as usize, diff);
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            self.sum(right as usize + 1) - self.sum(left as usize)
        }

        fn add(bit: &mut Vec<i32>, mut i: usize, val: i32) {
            i += 1;
            while i < bit.len() {
                bit[i] += val;
                i += (i as i32 & -(i as i32)) as usize;
            }
        }

        fn sum(&self, mut i: usize) -> i32 {
            let bit = self.bit.borrow();
            let mut result = 0;
            while i >= 1 {
                result += bit[i];
                i -= (i as i32 & -(i as i32)) as usize;
            }
            result
        }
    }
}

/// Precalculate prefix sums
///
/// - `O(n)` memory
/// - `O(n)` time `new`
/// - `O(n)` time `update`
/// - `O(1)` time `sum_range`
pub mod v2 {
    use std::cell::RefCell;

    pub struct NumArray {
        sums: RefCell<Vec<i32>>,
    }
    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            let sums = nums.into_iter().fold(vec![0], |mut acc, num| {
                acc.push(num + acc.last().unwrap());
                acc
            });

            Self {
                sums: RefCell::new(sums),
            }
        }

        pub fn update(&self, index: i32, val: i32) {
            let (i, len) = (index as usize, self.sums.borrow().len());
            let diff = self.sums.borrow()[i + 1] - self.sums.borrow()[i] - val;
            let mut sums = self.sums.borrow_mut();
            for j in i + 1..len {
                sums[j] -= diff;
            }
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            self.sums.borrow()[right as usize + 1] - self.sums.borrow()[left as usize]
        }
    }
}

/// Brute force
///
/// - `O(n)` memory
/// - `O(1)` time `new`
/// - `O(1)` time `update`
/// - `O(n)` time `sum_range`
pub mod v1 {
    use std::cell::RefCell;

    pub struct NumArray {
        nums: RefCell<Vec<i32>>,
    }
    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            Self {
                nums: RefCell::new(nums),
            }
        }

        pub fn update(&self, index: i32, val: i32) {
            self.nums.borrow_mut()[index as usize] = val;
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            self.nums.borrow()[left as usize..=right as usize]
                .iter()
                .sum()
        }
    }
}
