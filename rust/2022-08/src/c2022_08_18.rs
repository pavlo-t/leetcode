#![allow(dead_code)]
//! \#1338. Reduce Array Size to The Half
//! =====================================
//!
//! <https://leetcode.com/problems/reduce-array-size-to-the-half>
//!
//! You are given an integer array `arr`.
//! You can choose a set of integers and remove all the occurrences of these integers in the array.
//!
//! Return _the minimum size of the set so that __at least__ half of the integers of the array are removed_.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_18::*;
//! let arr = vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7];
//! assert_eq!(Solution::min_set_size(arr), 2);
//! ```
//!
//! __Explanation:__ Choosing `{3,7}` will make the new array `[5,5,5,2,2]` which has size `5`
//! (i.e equal to half of the size of the old array).
//! Possible sets of size `2` are `{3,5}`, `{3,2}`, `{5,2}`.
//! Choosing set `{2,7}` is not possible as it will make the new array `[3,3,3,3,5,5,5]`
//! which has a size greater than half of the size of the old array.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_18::*;
//! let arr = vec![7, 7, 7, 7, 7, 7];
//! assert_eq!(Solution::min_set_size(arr), 1);
//! ```
//!
//! __Explanation:__ The only possible set you can choose is `{7}`. This will make the new array empty.
//!
//! ##### Constraints
//!
//! - `2 <= arr.length <= 100_000`
//! - `arr.length` is even.
//! - `1 <= arr[i] <= 100_000`

pub struct Solution;
impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        use std::collections::{BinaryHeap, HashMap};

        let counts = arr.iter().fold(HashMap::new(), |mut counts, &num| {
            *counts.entry(num).or_insert(0) += 1;
            counts
        });

        let mut heap = counts.values().fold(BinaryHeap::new(), |mut heap, &count| {
            heap.push(count);
            heap
        });

        let mut target = arr.len() as i32 / 2;
        let mut took = 0;
        while let Some(count) = heap.pop() {
            target -= count;
            took += 1;
            if target <= 0 {
                return took;
            }
        }

        unreachable!();
    }
}
