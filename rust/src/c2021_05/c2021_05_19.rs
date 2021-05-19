/// Minimum Moves to Equal Array Elements II
/// ========================================
///
/// Given an integer array `nums` of size `n`,
/// return _the minimum number of moves required to make all array elements equal_.
///
/// In one move, you can increment or decrement an element of the array by `1`.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `-1_000_000_000 <= nums[i] <= 1_000_000_000`
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/600/week-3-may-15th-may-21st/3748/
struct Solution;
impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let median = nums[nums.len() / 2];
        nums.into_iter().fold(0, |s, i| s + (median - i).abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_ns_1_2_3_produces_2() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::min_moves2(nums), 2);
        // Explanation:
        // Only two moves are needed (remember each move increments or decrements one element):
        // [1,2,3]  =>  [2,2,3]  =>  [2,2,2]
    }
    #[test]
    fn example2_ns_1_10_2_9_produces_16() {
        let nums = vec![1, 10, 2, 9];
        assert_eq!(Solution::min_moves2(nums), 16);
    }

    #[test]
    fn ns_1_produces_0() {
        assert_eq!(Solution::min_moves2(vec![1]), 0);
    }
    #[test]
    fn ns_1_1_produces_0() {
        assert_eq!(Solution::min_moves2(vec![1, 1]), 0);
    }
    #[test]
    fn ns_1_2_produces_1() {
        assert_eq!(Solution::min_moves2(vec![1, 2]), 1);
    }
    #[test]
    fn ns_1_4_produces_3() {
        assert_eq!(Solution::min_moves2(vec![1, 4]), 3);
    }
    #[test]
    fn ns_1_5_produces_4() {
        assert_eq!(Solution::min_moves2(vec![1, 5]), 4);
    }
    #[test]
    fn ns_1_1_2_produces_1() {
        assert_eq!(Solution::min_moves2(vec![1, 1, 2]), 1);
    }
    #[test]
    fn ns_2_1_2_produces_1() {
        assert_eq!(Solution::min_moves2(vec![2, 1, 2]), 1);
    }
    #[test]
    fn ns_1_1_20_produces_19() {
        assert_eq!(Solution::min_moves2(vec![1, 1, 20]), 19);
    }
    #[test]
    fn ns_1_2_3_4_9_produces_10() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3, 4, 9]), 10);
    }
    #[test]
    fn ns_1_1_1_1_1_2_20_produces_20() {
        assert_eq!(Solution::min_moves2(vec![1, 1, 1, 1, 1, 2, 20]), 20);
    }
}
