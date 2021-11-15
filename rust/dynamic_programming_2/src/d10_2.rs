#![allow(dead_code)]
/// 368. Largest Divisible Subset
/// =============================
///
/// Given a set of __distinct__ positive integers `nums`,
/// return the largest subset `answer` such that every pair `(answer[i], answer[j])` of elements in this subset
/// satisfies `answer[i] % answer[j] == 0`, or `answer[j] % answer[i] == 0`.
///
/// If there are multiple solutions, return any of them.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 1000`
/// - `1 <= nums[i] <= 2_000_000_000`
/// - All the integers in nums are unique.
///
/// https://leetcode.com/problems/largest-divisible-subset/
struct Solution;
impl Solution {
    pub fn largest_divisible_subset_rec(nums: Vec<i32>) -> Vec<i32> {
        println!("largest_divisible_subset({:?})", nums);
        fn clone_pushed(n: i32, v: &Vec<i32>) -> Vec<i32> {
            let mut v = v.clone();
            v.push(n);
            v
        }
        fn rec(i: usize, s: Vec<i32>, ns: &[i32]) -> Vec<i32> {
            if i == ns.len() {
                s
            } else if s.iter().all(|&p| p % ns[i] == 0 || ns[i] % p == 0) {
                let pick = rec(i + 1, clone_pushed(ns[i], &s), ns);
                let skip = rec(i + 1, s, ns);
                if pick.len() >= skip.len() {
                    pick
                } else {
                    skip
                }
            } else {
                rec(i + 1, s, ns)
            }
        }
        rec(0, vec![], &nums)
    }
    pub fn largest_divisible_subset_backtracking_search(nums: Vec<i32>) -> Vec<i32> {
        println!("largest_divisible_subset({:?})", nums);
        fn bts(i: usize, ns: &[i32], s: &mut Vec<i32>, rsf: &mut Vec<i32>) {
            if i == ns.len() {
                if s.len() > rsf.len() {
                    rsf.clear();
                    rsf.extend_from_slice(&s);
                }
            } else if s.iter().all(|&p| p % ns[i] == 0 || ns[i] % p == 0) {
                let n = ns[i];
                s.push(n);
                bts(i + 1, ns, s, rsf);
                s.pop();
                bts(i + 1, ns, s, rsf);
            } else {
                bts(i + 1, ns, s, rsf);
            }
        }

        let n = nums.len();
        let mut result = Vec::with_capacity(n);
        bts(0, &nums, &mut result.clone(), &mut result);
        result
    }
    /// https://www.geeksforgeeks.org/largest-divisible-subset-array/
    pub fn largest_divisible_subset_geeksforgeeks(mut nums: Vec<i32>) -> Vec<i32> {
        println!("largest_divisible_subset({:?})", nums);
        // We first sort the array so that all divisors of a number are before it.
        nums.sort_unstable();
        let n = nums.len();
        // To store count of divisors of all elements
        let mut div_count = vec![1; n];
        // To store previous divisor in result
        let mut prev = vec![usize::MAX; n];
        // To store index of largest element in maximum size subset
        let mut max_ind = 0;
        // In i'th iteration, we find length of chain ending with arr[i]
        for i in 1..n {
            // Consider every smaller element as previous element.
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    if div_count[i] < div_count[j] + 1 {
                        div_count[i] = div_count[j] + 1;
                        prev[i] = j;
                    }
                }
            }
            // Update last index of largest subset if size of current subset is more.
            if div_count[max_ind] < div_count[i] {
                max_ind = i;
            }
        }

        let mut result = Vec::new();
        while max_ind < n {
            result.push(nums[max_ind]);
            max_ind = prev[max_ind];
        }
        result.sort_unstable();
        result
    }
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        println!("largest_divisible_subset({:?})", nums);
        nums.sort_unstable();
        let n = nums.len();
        let mut dp = vec![(1, usize::MAX); n];
        let mut best = 0;
        for r in 1..n {
            for l in 0..r {
                if nums[r] % nums[l] == 0 && dp[r].0 < dp[l].0 + 1 {
                    dp[r] = (dp[l].0 + 1, l);
                }
            }
            if dp[best].0 < dp[r].0 {
                best = r;
            }
        }
        let mut result = vec![];
        while best < n {
            result.push(nums[best]);
            best = dp[best].1;
        }
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_2()   { assert_eq!(Solution::largest_divisible_subset(vec![2]    ), vec![2]); }
    #[rustfmt::skip] #[test] fn n_23()  { assert_eq!(Solution::largest_divisible_subset(vec![2,3]  ), vec![2]); }
    #[rustfmt::skip] #[test] fn n_32()  { assert_eq!(Solution::largest_divisible_subset(vec![3,2]  ), vec![2]); }
    #[rustfmt::skip] #[test] fn n_24()  { assert_eq!(Solution::largest_divisible_subset(vec![2,4]  ), vec![2,4]); }
    #[rustfmt::skip] #[test] fn n_42()  { assert_eq!(Solution::largest_divisible_subset(vec![4,2]  ), vec![2,4]); }
    #[rustfmt::skip] #[test] fn n_239() { assert_eq!(Solution::largest_divisible_subset(vec![2,3,9]), vec![3,9]); }
    #[test]
    fn n_123() {
        let n = vec![1, 2, 3];
        let e = [1, 2];
        assert_eq!(Solution::largest_divisible_subset(n), e);
        // Explanation: [1,3] is also accepted.
    }
    #[test]
    fn n_1248() {
        let n = vec![1, 2, 4, 8];
        let e = [1, 2, 4, 8];
        assert_eq!(Solution::largest_divisible_subset(n), e);
    }

    #[test]
    fn test_48() {
        let n = vec![
            1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
            131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432,
            67108864, 134217728, 268435456, 536870912, 1073741824,
        ];
        let e = n.clone();
        assert_eq!(Solution::largest_divisible_subset(n), e);
    }

    #[test]
    fn n_1to1000() {
        let n = (1..=1000).collect();
        let e = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
        assert_eq!(Solution::largest_divisible_subset(n), e);
    }
}
