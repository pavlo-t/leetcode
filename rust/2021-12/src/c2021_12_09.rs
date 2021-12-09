#![allow(dead_code)]
/// 1306. Jump Game III
/// ===================
///
/// Given an array of non-negative integers `arr`, you are initially positioned at `start` index of the array.
/// When you are at index `i`, you can jump to `i + arr[i]` or `i - arr[i]`,
/// check if you can reach to __any__ index with value `0`.
///
/// Notice that you can not jump outside of the array at any time.
///
/// __Constraints:__
///
/// - `1 <= arr.length <= 50_000`
/// - `0 <= arr[i] < arr.length`
/// - `0 <= start < arr.length`
///
/// https://leetcode.com/problems/jump-game-iii/
struct Solution;
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        use std::iter::once;

        let (n, start) = (arr.len(), start as usize);
        let mut visited = vec![false; n];
        let mut stack = vec![start];
        visited[start] = true;

        while let Some(i) = stack.pop() {
            let v = arr[i] as usize;
            if v == 0 {
                return true;
            }
            for j in once(i.wrapping_sub(v)).chain(once(i + v)) {
                if j < n && !visited[j] {
                    visited[j] = true;
                    stack.push(j);
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a4230312_s5() {
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
        // Explanation:
        // All possible ways to reach at index 3 with value 0 are:
        // index 5 -> index 4 -> index 1 -> index 3
        // index 5 -> index 6 -> index 4 -> index 1 -> index 3
    }
    #[test]
    fn a4230312_s0() {
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0));
        // Explanation:
        // One possible way to reach at index 3 with value 0 is:
        // index 0 -> index 4 -> index 1 -> index 3
    }
    #[test]
    fn a30212_s2() {
        assert!(!Solution::can_reach(vec![3, 0, 2, 1, 2], 2));
        // Explanation: There is no way to reach at index 1 with value 0.
    }
}
