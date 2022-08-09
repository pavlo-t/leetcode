#![allow(dead_code)]
//! \#823. Binary Trees With Factors
//! ================================
//!
//! <https://leetcode.com/problems/binary-trees-with-factors>
//!
//! Given an array of unique integers, `arr`, where each integer `arr[i]` is strictly greater than `1`.
//!
//! We make a binary tree using these integers, and each number may be used for any number of times.
//! Each non-leaf node's value should be equal to the product of the values of its children.
//!
//! Return _the number of binary trees we can make_.
//! The answer may be too large so return the answer __modulo__ `1_000_000_007`.
//!
//! ##### Constraints
//!
//! - `1 <= arr.length <= 1000`
//! - `2 <= arr[i] <= 1_000_000_000`
//! - All the values of `arr` are __unique__.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_09::Solution;
//! let arr = vec![2, 4];
//! assert_eq!(Solution::num_factored_binary_trees(arr), 3);
//! ```
//!
//! __Explanation:__ We can make these trees: `[2]`, `[4]`, `[4, 2, 2]`
//!
//! ##### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_09::Solution;
//! let arr = vec![2, 4, 5, 10];
//! assert_eq!(Solution::num_factored_binary_trees(arr), 7);
//! ```
//!
//! __Explanation:__ We can make these trees: `[2]`, `[4]`, `[5]`, `[10]`, `[4, 2, 2]`, `[10, 2, 5]`, `[10, 5, 2]`.

pub struct Solution;
impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        const MOD: i64 = 1_000_000_007;

        let mut available_trees: HashMap<i64, i64> = HashMap::new();

        arr.sort_unstable();

        arr.iter()
            .map(|&num| num as i64)
            .fold(0i64, |result, root| {
                let count = arr
                    .iter()
                    .map(|&l| l as i64)
                    .take_while(|&l| l * l <= root)
                    .filter(|&l| root % l == 0)
                    .fold(1i64, |count, l| {
                        let count_l = available_trees.get(&l).unwrap();
                        let r = root / l;
                        if l == r {
                            (count + count_l * count_l) % MOD
                        } else if let Some(&count_r) = available_trees.get(&r) {
                            (count + count_l * count_r * 2) % MOD
                        } else {
                            count
                        }
                    });
                available_trees.insert(root, count);
                (result + count) % MOD
            }) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `[2]`
    #[rustfmt::skip] #[test] fn arr_2() { assert_eq!(Solution::num_factored_binary_trees(vec![2]), 1); }

    /// `[2],[4], [4,2,2]`
    #[rustfmt::skip] #[test] fn arr_2_4() { assert_eq!(Solution::num_factored_binary_trees(vec![2,4]), 3); }

    /// `[2],[3],[4], [4,2,2]`
    #[rustfmt::skip] #[test] fn arr_2_3_4() { assert_eq!(Solution::num_factored_binary_trees(vec![2,3,4]), 4); }

    /// `[2],[3],[5]`
    #[rustfmt::skip] #[test] fn arr_2_3_5() { assert_eq!(Solution::num_factored_binary_trees(vec![2,3,5]), 3); }

    /// `[2],[3],[6], [6,2,3],[6,3,2]`
    #[rustfmt::skip] #[test] fn arr_2_3_6() { assert_eq!(Solution::num_factored_binary_trees(vec![2,3,6]), 5); }

    /// `[2],[3],[7]`
    #[rustfmt::skip] #[test] fn arr_2_3_7() { assert_eq!(Solution::num_factored_binary_trees(vec![2,3,7]), 3); }

    /// `[2],[4],[8], [4,2,2],[8,2,4],[8,4,2], [8,2,4,_,_,2,2],[8,4,2,2,2,_,_]`
    #[rustfmt::skip] #[test] fn arr_2_4_8() { assert_eq!(Solution::num_factored_binary_trees(vec![2,4,8]), 8); }

    /// `[2],[4],[5],[10], [4,2,2],[10,2,5],[10,5,2]`
    #[rustfmt::skip] #[test] fn arr_2_4_5_10() { assert_eq!(Solution::num_factored_binary_trees(vec![2,4,5,10]), 7); }

    #[test]
    fn arr_2_4_8_16() {
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 4, 8, 16]), 23);
    }
    #[test]
    fn arr_2_4_8_16_32() {
        let nums = vec![2, 4, 8, 16, 32];
        assert_eq!(Solution::num_factored_binary_trees(nums), 74);
    }
    #[test]
    fn arr_2_4_8_16_32_64() {
        let nums = vec![2, 4, 8, 16, 32, 64];
        assert_eq!(Solution::num_factored_binary_trees(nums), 262);
    }

    #[test]
    fn arr_2_to_1001() {
        let nums = (2..1002).collect();
        assert_eq!(Solution::num_factored_binary_trees(nums), 769_795);
    }
}
