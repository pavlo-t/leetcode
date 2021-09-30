#![allow(dead_code)]
/// Jump Game VI
/// ============
///
/// You are given a __0-indexed__ integer array `nums` and an integer `k`.
///
/// You are initially standing at index `0`.
/// In one move, you can jump at most `k` steps forward without going outside the boundaries of the array.
/// That is, you can jump from index `i` to any index in the range `[i + 1, min(n - 1, i + k)]` __inclusive__.
///
/// You want to reach the last index of the array (index `n - 1`).
/// Your __score__ is the __sum__ of all `nums[j]` for each index `j` you visited in the array.
///
/// Return _the __maximum score__ you can get_.
///
/// __Constraints:__
///
/// - `1 <= nums.length, k <= 100_000`
/// - `-10_000 <= nums[i] <= 10_000`
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/604/week-2-june-8th-june-14th/3773/
struct Solution;
impl Solution {
    /// https://leetcode.com/problems/jump-game-vi/discuss/978563/Python3-range-max-via-priority-queue-and-mono-queue
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::VecDeque;

        let mut queue: VecDeque<(i32, usize)> = VecDeque::new();

        for i in (0..nums.len()).rev() {
            if !queue.is_empty() && queue[0].1 - i > k as usize { queue.pop_front(); }
            let result = nums[i] + queue.front().unwrap_or(&(0, 0)).0;
            while !queue.is_empty() && queue.back().unwrap().0 <= result { queue.pop_back(); }
            queue.push_back((result, i))
        }

        queue.pop_back().unwrap().0
    }

    /// https://leetcode.com/problems/jump-game-vi/solution/
    ///
    /// Approach 1: Dynamic Programming + Deque
    pub fn max_result_leetcode(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::VecDeque;

        let n = nums.len();
        let k = k as usize;
        let mut score = vec![0; n];
        score[0] = nums[0];
        let mut dq = VecDeque::new();
        dq.push_back(0);

        for i in 1..n {
            if i >= k {
                while !dq.is_empty() && dq.front().unwrap().to_owned() < i - k {
                    dq.pop_front();
                }
            }
            score[i] = score[dq.front().unwrap().to_owned()] + nums[i];
            while !dq.is_empty() && score[i] >= score[dq.back().unwrap().to_owned()] {
                dq.pop_back();
            }
            dq.push_back(i);
        }

        score.last().unwrap().to_owned()
    }
    pub fn max_result_my_dp_2_still_not_good(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;

        let k = k as usize;
        let mut dp = nums.clone();
        let mut heap = BinaryHeap::new();
        heap.push(dp[dp.len() - 1]);

        for i in (0..dp.len() - 1).rev() {
            let j = i + k;
            let m = if (j + 1) < dp.len() && heap.peek().unwrap().to_owned() == dp[j + 1] {
                heap.pop();
                while !dp[i + 1..=j].contains(heap.peek().unwrap()) {
                    heap.pop();
                }
                heap.peek().unwrap().to_owned()
            } else {
                heap.peek().unwrap().to_owned()
            };
            dp[i] += m;
            heap.push(dp[i]);
        }
        dp[0]
    }

    pub fn max_result_my_dp_1_time_limit_exception(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = nums.clone();
        for i in (0..dp.len() - 1).rev() {
            let mut m = -10_001;
            for j in i + 1..dp.len().min(i + k + 1) {
                m = m.max(dp[j])
            }
            dp[i] += m;
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nums_1_m1_m2_4_m7_3_k_2_produces_7() {
        let nums = vec![1, -1, -2, 4, -7, 3];
        // nums = [1, -1, -2, 4, -7, 3]
        // dp   = [7,  6,  5, 7, -4, 3]
        // heap = [                , 3]
        let k = 2;
        assert_eq!(Solution::max_result(nums, k), 7);
        // Explanation:
        // You can choose your jumps forming the subsequence [1,-1,4,3] (underlined above).
        // The sum is 7.
    }
    #[test]
    fn nums_10_m5_m2_4_0_3_k_3_produces_17() {
        let nums = vec![10, -5, -2, 4, 0, 3];
        let k = 3;
        assert_eq!(Solution::max_result(nums, k), 17);
        // Explanation:
        // You can choose your jumps forming the subsequence [10,4,3] (underlined above).
        // The sum is 17.
    }
    #[test]
    fn nums_1_m5_m20_4_m1_3_m6_m3_k_2_produces_0() {
        let nums = vec![1, -5, -20, 4, -1, 3, -6, -3];
        // nums = [1, -5, -20, 4, -1, 3, -6, -3];
        // dp   = [0, -1, -16, 4, -1, 0, -9, -3];
        let k = 2;
        assert_eq!(Solution::max_result(nums, k), 0);
    }

    /// test 58
    #[test]
    fn nums_100_m1_m100_m1_100_k_2_produces_198() {
        let nums = vec![100, -1, -100, -1, 100];
        let k = 2;
        assert_eq!(Solution::max_result(nums, k), 198);
    }

    #[test]
    fn nums_100k_1_k_100k_produces_100k() {
        let nums = vec![1; 100_000];
        let k = 100_000;
        assert_eq!(Solution::max_result(nums, k), 100_000);
    }
    #[test]
    fn nums_100k_m1_k_100k_produces_m2() {
        let nums = vec![-1; 100_000];
        let k = 100_000;
        assert_eq!(Solution::max_result(nums, k), -2);
    }
}
