#![allow(dead_code)]
/// Construct Target Array With Multiple Sums
/// =========================================
///
/// You are given an array `target` of `n` integers.
/// From a starting array `arr` consisting of `n` `1`'s, you may perform the following procedure:
///
/// - let `x` be the sum of all elements currently in your array.
/// - choose index `i`, such that `0 <= i < n` and set the value of `arr` at index `i` to `x`.
/// - You may repeat this procedure as many times as needed.
///
/// Return _`true` if it is possible to construct the `target` array from `arr`,
/// otherwise, return `false`_.
///
/// __Constraints:__
///
/// - `1 <= target.length <= 50_000`
/// - `1 <= target[i] <= 10^9`
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3737/
struct Solution;
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            target[0] == 1
        } else {
            use std::collections::BinaryHeap;

            let mut sum = target.iter().sum::<i32>();
            let mut heap = target
                .into_iter()
                .filter(|i| i > &1)
                .collect::<BinaryHeap<_>>();

            while let Some(max) = heap.pop() {
                if max == 1 {
                    return true;
                }

                let rest = sum - max;
                if rest == 1 {
                    return true;
                }
                let rem = max % rest;
                if rem < 1 || rem == max {
                    return false;
                }
                heap.push(rem);
                sum = sum - max + rem;
            }
            unreachable!()
        }
    }

    /// https://leetcode.com/problems/construct-target-array-with-multiple-sums/solution/
    pub fn is_possible_leetcode(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            target[0] == 1
        } else {
            use std::collections::BinaryHeap;

            let mut sum = target.iter().sum::<i32>();
            let mut heap = target.into_iter().collect::<BinaryHeap<_>>();

            while let Some(max) = heap.pop() {
                if max == 1 {
                    return true;
                }

                let rest = sum - max;
                if rest == 1 {
                    return true;
                }
                let rem = max % rest;
                if rem < 1 || rem == max {
                    return false;
                }
                heap.push(rem);
                sum = sum - max + rem;
            }
            unreachable!()
        }
    }

    /// https://en.wikipedia.org/wiki/Pseudo-polynomial_time
    pub fn is_possible_my_pseudo_polynomial(target: Vec<i32>) -> bool {
        use std::collections::BinaryHeap;

        let mut sum = target.iter().sum::<i32>();
        let mut heap = target.into_iter().collect::<BinaryHeap<_>>();

        while let Some(max) = heap.pop() {
            if max == 1 {
                return true;
            }
            let rem = 2 * max - sum;
            if rem < 1 {
                return false;
            }
            heap.push(rem);
            sum = max;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_t_9_3_5_is_possible() {
        // 3 5 9 = 17; 17 - 9 = 8
        // 3 5 1 = 9; 9 - 5 = 4
        // 3 1 1 = 5; 5 - 3 = 2
        // 1 1 1 = 5; 5 - 3 = 2
        let target = vec![9, 3, 5];
        assert!(Solution::is_possible(target));
        // Output: true
        // Explanation: Start with arr = [1, 1, 1]
        // [1, 1, 1], sum = 3 choose index 1
        // [1, 3, 1], sum = 5 choose index 2
        // [1, 3, 5], sum = 9 choose index 0
        // [9, 3, 5] Done
    }
    #[test]
    fn example2_t_1_1_1_2_is_not_possible() {
        let target = vec![1, 1, 1, 2];
        assert!(!Solution::is_possible(target));
        // Explanation: Impossible to create target array from [1,1,1,1].
    }
    #[test]
    fn example3_t_8_5_is_possible() {
        // 5 8 = 13; 13 - 8 = 5
        // 5 3 = 8; 8 - 5 = 3
        // 2 3 = 5; 5 - 3 = 2
        // 2 1 = 5; 5 - 3 = 2
        // 1 1 = 5; 5 - 3 = 2
        let target = vec![8, 5];
        assert!(Solution::is_possible(target));
    }

    #[test]
    fn test15_t_2_is_not_possible() {
        let target = vec![2];
        assert!(!Solution::is_possible(target));
    }

    #[test]
    fn t_1_2_is_possible() {
        assert!(Solution::is_possible(vec![1, 2]));
    }
    #[test]
    fn t_1_1_3_is_possible() {
        assert!(Solution::is_possible(vec![1, 1, 3]));
    }
    #[test]
    fn t_1_5_is_possible() {
        assert!(Solution::is_possible(vec![1, 5]));
        // [1,1] = 2
        // [1,2] = 3
    }
    #[test]
    fn t_1_1_6_is_not_possible() {
        assert!(!Solution::is_possible(vec![1, 1, 6]));
    }
    #[test]
    fn t_1_1_7_is_possible() {
        assert!(Solution::is_possible(vec![1, 1, 7]));
    }
    #[test]
    fn t_9_17_29_is_possible() {
        // 9 17 29 = 55; 29 - (9 + 17) = 29 - 26
        // 9 17  3 = 29; 17 - (9 + 3) = 17 - 12
        // 9  5  3 = 17; 9 - (5 + 3) = 9 - 8
        // 1  5  3 =  9; 5 - (3 + 1) = 5 - 4
        // 1  1  3 =  5; 3 - (1 + 1) = 3 - 2
        // 1  1  1
        let target = vec![9, 17, 29];
        assert!(Solution::is_possible(target));
        // 1  1  1 = 3
        // 1  1  3 = 5
        // 1  5  3 = 9
        // 9  5  3 = 17
        // 9 17  3 = 29
        // 9 17 29
    }

    #[test]
    fn t_1_10pow9_is_possible() {
        let target = vec![1, 1_000_000_000];
        assert!(Solution::is_possible(target));
    }
    #[test]
    fn t_1_1_10pow9min1_is_possible() {
        let target = vec![1, 999_999_999, 1];
        assert!(Solution::is_possible(target));
    }
}
