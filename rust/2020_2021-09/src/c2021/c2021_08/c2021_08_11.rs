#![allow(dead_code)]
/// Array of Doubled Pairs
/// ======================
///
/// Given an array of integers `arr` of even length,
/// return `true` if and only if it is possible to reorder it
/// such that `arr[2 * i + 1] = 2 * arr[2 * i]` for every `0 <= i < len(arr) / 2`.
///
/// arr[j] * 2 = arr[k]; j = 2 * i, k = 2 * i + 1
/// i = 0, j = 0, k = 1
/// i = 1, j = 2, k = 3
/// i = 2, j = 4, k = 5
/// i = 3, j = 6, k = 7
/// i = 4, j = 8, k = 9
///
/// __Constraints:__
///
/// - `0 <= arr.length <= 30_000`
/// - `arr.length` is even.
/// - `-100_000 <= arr[i] <= 100_000`
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/614/week-2-august-8th-august-14th/3877/
struct Solution;
impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;

        fn can(arr: &[i32], counts: &mut HashMap<i32, usize>) -> bool {
            let mut n = arr.len();
            let mut dec = |i| counts.get_mut(&i).filter(|c| c > &&mut 0).map(|c| *c -= 1);
            for &i in arr {
                if dec(i).is_some() {
                    if dec(i * 2).is_none() {
                        return false;
                    } else if n == 0 {
                        return true;
                    } else {
                        n -= 2;
                    }
                }
            }
            true
        }

        let mut pos = vec![];
        let mut neg = vec![];
        let mut counts: HashMap<i32, usize> = HashMap::new();
        for i in arr {
            if i < 0 {
                neg.push(i);
            } else {
                pos.push(i);
            }
            *counts.entry(i).or_default() += 1;
        }
        pos.sort_unstable();
        neg.sort_unstable();
        neg.reverse();

        can(&pos, &mut counts) && can(&neg, &mut counts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_p3p1p3p6_produces_false() {
        let arr = vec![3, 1, 3, 6];
        assert!(!Solution::can_reorder_doubled(arr));
    }
    #[test]
    fn a_p2p1p2p6_produces_false() {
        let arr = vec![2, 1, 2, 6];
        assert!(!Solution::can_reorder_doubled(arr));
    }
    #[test]
    fn a_p4m2p2m4_produces_true() {
        let arr = vec![4, -2, 2, -4];
        assert!(Solution::can_reorder_doubled(arr));
        // Explanation: We can take two groups, [-2,-4] and [2,4] to form [-2,-4,2,4] or [2,4,-2,-4].
    }
    #[test]
    fn a_p1p2p4p16p8p4_produces_false() {
        let arr = vec![1, 2, 4, 16, 8, 4];
        assert!(!Solution::can_reorder_doubled(arr));
    }

    #[test]
    fn a_empty_produces_true() {
        let arr = vec![];
        assert!(Solution::can_reorder_doubled(arr));
    }

    #[test]
    fn a_m1p2_produces_false() {
        let arr = vec![-1, 2];
        assert!(!Solution::can_reorder_doubled(arr));
    }
}
