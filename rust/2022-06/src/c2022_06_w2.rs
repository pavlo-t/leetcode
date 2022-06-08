#![allow(dead_code)]
/// \#1151. Minimum Swaps to Group All 1's Together
/// ===============================================
///
/// Given a binary array `data`,
/// return the minimum number of swaps required to
/// group all `1`’s present in the array together
/// in __any place__ in the array.
///
/// __Constraints:__
///
/// - `1 <= data.length <= 100_000`
/// - `data[i]` is either `0` or `1`.
///
/// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together
struct Solution;
impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let ones = data.iter().filter(|&&n| n == 1).count();
        let mut zeroes = data.iter().take(ones).filter(|&&n| n == 0).count() as i32;
        let mut result = zeroes;
        for r in ones..data.len() {
            let l = r - ones;
            zeroes += data[l] - data[r];
            result = result.min(zeroes);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_10101() {
        let d = vec![1, 0, 1, 0, 1];
        assert_eq!(Solution::min_swaps(d), 1);
        // Explanation: There are 3 ways to group all 1's together:
        // [1,1,1,0,0] using 1 swap.
        // [0,1,1,1,0] using 2 swaps.
        // [0,0,1,1,1] using 1 swap.
        // The minimum is 1.
    }
    #[test]
    fn d_00010() {
        let d = vec![0, 0, 0, 1, 0];
        assert_eq!(Solution::min_swaps(d), 0);
        // Explanation: Since there is only one 1 in the array, no swaps are needed.
    }
    #[test]
    fn d_10101001101() {
        let d = vec![1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1];
        assert_eq!(Solution::min_swaps(d), 3);
        // Explanation: One possible solution that uses 3 swaps is [0,0,0,0,0,1,1,1,1,1,1].
    }
}
