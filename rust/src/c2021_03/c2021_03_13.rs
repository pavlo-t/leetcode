#![allow(dead_code)]

/// # Binary Trees With Factors
///
/// Given an array of unique integers, `arr`, where each integer `arr[i]` is strictly greater than `1`.
///
/// We make a binary tree using these integers, and each number may be used for any number of times.
/// Each non-leaf node's value should be equal to the product of the values of its children.
///
/// Return _the number of binary trees we can make_.
/// The answer may be too large so return the answer __modulo__ `10^9 + 7`.
///
/// __Constraints:__
///
/// - `1 <= arr.length <= 1000`
/// - `2 <= arr[i] <= 10^9`
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3670/
struct Solution;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        const M: i64 = 1_000_000_007;
        arr.sort_unstable();
        let mut ways: HashMap<i32, i64> = HashMap::with_capacity(arr.len());
        let mut result = 0;
        for &n in &arr {
            let mut wn = 1;
            let lim = (n as f64).sqrt();
            for &a in &arr {
                if a as f64 > lim {
                    break;
                } else if let (b, 0) = (n / a, n % a) {
                    if a == b {
                        wn += ways[&a].pow(2);
                    } else if let Some(wb) = ways.get(&b) {
                        wn += ways[&a] * wb * 2;
                    }
                }
            }
            ways.insert(n, wn);
            result = (result + wn) % M;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_a24_produces_3() {
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 4]), 3);
        // Explanation: We can make these trees: [2], [4], [4, 2, 2]
    }
    #[test]
    fn example2_a2_4_5_10_produces_7() {
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 4, 5, 10]), 7);
        // Explanation: We can make these trees: [2], [4], [5], [10], [4, 2, 2], [10, 2, 5], [10, 5, 2].
    }

    #[test]
    fn a2_produces_1() {
        assert_eq!(Solution::num_factored_binary_trees(vec![2]), 1);
    }
    #[test]
    fn a2_3_produces_2() {
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 3]), 2);
    }
}
