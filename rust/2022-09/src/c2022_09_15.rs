#![allow(dead_code)]
//! \#2007. Find Original Array From Doubled Array
//! ==============================================
//!
//! <https://leetcode.com/problems/find-original-array-from-doubled-array>
//!
//! An integer array `original` is transformed into a __doubled__ array `changed`
//! by appending __twice the value__ of every element in `original`,
//! and then randomly __shuffling__ the resulting array.
//!
//! Given an array `changed`, _return `original` if `changed` is a __doubled__ array.
//! If `changed` is not a __doubled__ array, return an empty array.
//! The elements in `original` may be returned in __any__ order_.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_09::c2022_09_15::*;
//! let changed = vec![1, 3, 4, 2, 6, 8];
//! let mut original = Solution::find_original_array(changed);
//! original.sort_unstable();
//! assert_eq!(original, [1, 3, 4]);
//! ```
//!
//! __Explanation:__ One possible original array could be `[1, 3, 4]`:
//!
//! - Twice the value of `1` is `1 * 2 = 2`.
//! - Twice the value of `3` is `3 * 2 = 6`.
//! - Twice the value of `4` is `4 * 2 = 8`.
//!
//! Other original arrays could be `[4, 3, 1]` or `[3, 1, 4]`.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_09::c2022_09_15::*;
//! let changed = vec![6, 3, 0, 1];
//! const EMPTY: Vec<i32> = vec![];
//! assert_eq!(Solution::find_original_array(changed), EMPTY);
//! ```
//!
//! __Explanation:__ changed is not a doubled array.
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_09::c2022_09_15::*;
//! let changed = vec![1];
//! const EMPTY: Vec<i32> = vec![];
//! assert_eq!(Solution::find_original_array(changed), EMPTY);
//! ```
//!
//! __Explanation:__ changed is not a doubled array.
//!
//! ##### Constraints
//!
//! - `1 <= changed.length <= 100_000`
//! - `0 <= changed[i] <= 100_000`

pub struct Solution;
impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut original = vec![];
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for &key in &changed {
            counts.entry(key).and_modify(|curr| *curr += 1).or_insert(1);
        }

        if let Some(mut count) = counts.remove(&0) {
            if count % 2 == 0 {
                while count > 0 {
                    original.push(0);
                    count -= 2;
                }
            } else {
                return vec![];
            }
        }

        let mut keys = counts.keys().map(|&key| key).collect::<Vec<_>>();
        keys.sort_unstable();

        for key in keys {
            let mut c1 = *counts.get(&key).unwrap();
            if c1 > 0 {
                if let Some(c2) = counts.get_mut(&(key * 2)).filter(|c2| c2 >= &mut &mut c1) {
                    *c2 -= c1;
                } else {
                    return vec![];
                }
                while c1 > 0 {
                    original.push(key);
                    c1 -= 1;
                }
            }
        }

        original
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: Vec<i32> = vec![];

    #[test]
    fn c_1_3_4_2_6_8() {
        let changed = vec![1, 3, 4, 2, 6, 8];
        let mut original = Solution::find_original_array(changed);
        original.sort_unstable();
        assert_eq!(original, [1, 3, 4]);
    }

    #[test]
    fn c_6_3_0_1() {
        let changed = vec![6, 3, 0, 1];
        assert_eq!(Solution::find_original_array(changed), EMPTY);
    }

    #[test]
    fn c_0_0_1_2_3_6() {
        let changed = vec![0, 0, 1, 2, 3, 6];
        let mut original = Solution::find_original_array(changed);
        original.sort_unstable();
        assert_eq!(original, [0, 1, 3]);
    }

    #[test]
    fn c_1() {
        let changed = vec![1];
        assert_eq!(Solution::find_original_array(changed), EMPTY);
    }
}
