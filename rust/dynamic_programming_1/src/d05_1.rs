#![allow(dead_code)]
/// 53. Maximum Subarray
/// ====================
///
/// Given an integer array `nums`,
/// find the contiguous subarray (containing at least one number)
/// which has the largest sum and return _its sum_.
///
/// A __subarray__ is a __contiguous__ part of an array.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `-10_000 <= nums[i] <= 10_000`
///
/// __Follow up__: If you have figured out the `O(n)` solution,
/// try coding another solution using the __divide and conquer approach__, which is more subtle.
///
/// https://leetcode.com/problems/maximum-subarray/
struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        println!("{:?}", nums.iter().take(32).collect::<Vec<_>>());
        fn rec(ns: &[i32]) -> i32 {
            match ns.len() {
                0 => i32::MIN,
                1 => ns[0],
                n => {
                    let m = n / 2;

                    let best_from_start = |(b, mut t): (i32, i32), &n| {
                        t += n;
                        (b.max(t), t)
                    };
                    let (l, _) = ns[..m].iter().rev().fold((0, 0), best_from_start);
                    let (r, _) = ns[m + 1..].iter().fold((0, 0), best_from_start);

                    rec(&ns[..m]).max(l + ns[m] + r).max(rec(&ns[m + 1..]))
                }
            }
        }
        rec(&nums)
    }

    pub fn max_sub_array_kadane(nums: Vec<i32>) -> i32 {
        let (mut best, mut curr) = (nums[0], nums[0]);
        for n in nums.into_iter().skip(1) {
            curr = n + curr.max(0);
            best = best.max(curr);
        }
        best
    }

    /// Approach 3: Divide and Conquer (Advanced)
    /// https://leetcode.com/problems/maximum-subarray/solution/
    pub fn max_sub_array_divide_and_conquer(nums: Vec<i32>) -> i32 {
        println!("max_sub_array({:?})", nums);
        fn dfs(l: usize, r: usize, ns: &[i32]) -> i32 {
            if l == r {
                ns[l]
            } else {
                let m = l + (r - l) / 2;

                let (mut lsb, mut lst) = (i32::MIN, 0);
                for i in (l..=m).rev() {
                    lst += ns[i];
                    lsb = lsb.max(lst);
                }

                let (mut rsb, mut rst) = (i32::MIN, 0);
                for i in m + 1..=r {
                    rst += ns[i];
                    rsb = rsb.max(rst);
                }

                dfs(l, m, ns).max(dfs(m + 1, r, ns)).max(lsb + rsb)
            }
        }
        dfs(0, nums.len() - 1, &nums)
    }
    /// Approach 2: Dynamic Programming, Kadane's Algorithm
    /// https://leetcode.com/problems/maximum-subarray/solution/
    pub fn max_sub_array_dp_kadane(nums: Vec<i32>) -> i32 {
        println!("max_sub_array({:?})", nums);
        let mut curr = nums[0];
        let mut result = curr;
        for n in nums.into_iter().skip(1) {
            curr = n.max(curr + n);
            result = result.max(curr);
        }
        result
    }
    /// Approach 1: Optimized Brute Force
    /// https://leetcode.com/problems/maximum-subarray/solution/
    pub fn max_sub_array_too_slow(nums: Vec<i32>) -> i32 {
        println!("max_sub_array({:?})", nums);
        let mut result = i32::MIN;
        for l in 0..nums.len() {
            let mut curr = 0;
            for r in l..nums.len() {
                curr += nums[r];
                result = result.max(curr);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_m2p1m3p4m1p2p1m5p4() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(nums), 6);
        // Explanation: [4,-1,2,1] has the largest sum = 6.
    }
    #[test]
    fn n_p5p4m1p7p8() {
        let nums = vec![5, 4, -1, 7, 8];
        assert_eq!(Solution::max_sub_array(nums), 23);
    }
    #[rustfmt::skip] #[test] fn n_p1() { assert_eq!(Solution::max_sub_array(vec![ 1]), 1); }
    #[rustfmt::skip] #[test] fn n_m1() { assert_eq!(Solution::max_sub_array(vec![-1]), -1); }

    #[rustfmt::skip] #[test] fn n_p1m1() { assert_eq!(Solution::max_sub_array(vec![ 1, -1]), 1); }
    #[rustfmt::skip] #[test] fn n_m1p1() { assert_eq!(Solution::max_sub_array(vec![-1,  1]), 1); }
    #[rustfmt::skip] #[test] fn n_p1p1() { assert_eq!(Solution::max_sub_array(vec![ 1,  1]), 2); }

    #[rustfmt::skip] #[test] fn n_m1m1m1() { assert_eq!(Solution::max_sub_array(vec![-1, -1, -1]), -1); }
    #[rustfmt::skip] #[test] fn n_m1p1m1() { assert_eq!(Solution::max_sub_array(vec![-1,  1, -1]),  1); }
    #[rustfmt::skip] #[test] fn n_p1m1p1() { assert_eq!(Solution::max_sub_array(vec![ 1, -1,  1]),  1); }
    #[rustfmt::skip] #[test] fn n_m1p1p1() { assert_eq!(Solution::max_sub_array(vec![-1,  1,  1]),  2); }
    #[rustfmt::skip] #[test] fn n_p1p1m1() { assert_eq!(Solution::max_sub_array(vec![ 1,  1, -1]),  2); }
    #[rustfmt::skip] #[test] fn n_p1p1p1() { assert_eq!(Solution::max_sub_array(vec![ 1,  1,  1]),  3); }

    #[test]
    fn n_100000x1() {
        let nums = vec![1; 100_000];
        assert_eq!(Solution::max_sub_array(nums), 100_000);
    }
}
