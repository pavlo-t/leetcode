#![allow(dead_code)]
/// Minimum Cost to Connect Sticks
/// ==============================
///
/// You have some number of sticks with positive integer lengths.
/// These lengths are given as an array `sticks`, where `sticks[i]` is the length of the `i`th stick.
///
/// You can connect any two sticks of lengths `x` and `y` into one stick by paying a cost of `x + y`.
/// You must connect all the sticks until there is only one stick remaining.
///
/// Return _the minimum cost of connecting all the given sticks into one stick in this way_.
///
/// __Constraints:__
///
/// - `1 <= sticks.length <= 10_000`
/// - `1 <= sticks[i] <= 10_000`
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/602/week-5-may-29th-may-31st/3759/
struct Solution;
impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap = sticks
            .into_iter()
            .map(|i| Reverse(i))
            .collect::<BinaryHeap<_>>();
        let mut result = 0;
        while heap.len() > 1 {
            let Reverse(a) = heap.pop().unwrap();
            let Reverse(b) = heap.pop().unwrap();
            let c = a + b;
            result += c;
            heap.push(Reverse(c))
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example 1:
    #[test]
    fn s_2_4_3_produces_14() {
        let sticks = vec![2, 4, 3];
        assert_eq!(Solution::connect_sticks(sticks), 14);
        // Explanation: You start with sticks = [2,4,3].
        // 1. Combine sticks 2 and 3 for a cost of 2 + 3 = 5. Now you have sticks = [5,4].
        // 2. Combine sticks 5 and 4 for a cost of 5 + 4 = 9. Now you have sticks = [9].
        // There is only one stick left, so you are done. The total cost is 5 + 9 = 14.
    }
    // Example 2:
    #[test]
    fn s_1_8_3_5_produces_30() {
        let sticks = vec![1, 8, 3, 5];
        assert_eq!(Solution::connect_sticks(sticks), 30);
        // Explanation: You start with sticks = [1,8,3,5].
        // 1. Combine sticks 1 and 3 for a cost of 1 + 3 = 4. Now you have sticks = [4,8,5].
        // 2. Combine sticks 4 and 5 for a cost of 4 + 5 = 9. Now you have sticks = [9,8].
        // 3. Combine sticks 9 and 8 for a cost of 9 + 8 = 17. Now you have sticks = [17].
        // There is only one stick left, so you are done. The total cost is 4 + 9 + 17 = 30.
    }
    // Example 3:
    #[test]
    fn s_5_produces_0() {
        let sticks = vec![5];
        assert_eq!(Solution::connect_sticks(sticks), 0);
        // Explanation: There is only one stick, so you don't need to do anything. The total cost is 0.
    }

    mod performance {
        use super::*;

        #[test]
        fn s_10k_1s_produces_1() {
            let sticks = vec![1; 10000];
            assert_eq!(Solution::connect_sticks(sticks), 133616);
        }
    }
}
