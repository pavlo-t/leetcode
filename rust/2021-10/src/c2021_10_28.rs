#![allow(dead_code)]
/// 15. 3Sum
/// ========
///
/// Given an integer array nums, return all the triplets `[nums[i], nums[j], nums[k]]`
/// such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`.
///
/// Notice that the solution set must not contain duplicate triplets.
///
/// __Constraints:__
///
/// - `0 <= nums.length <= 3000`
/// - `-100_000 <= nums[i] <= 100_000`
///
/// https://leetcode.com/problems/3sum/
struct Solution;
impl Solution {
    /// Approach 3: "No-Sort"
    /// https://leetcode.com/problems/3sum/solution/
    pub fn three_sum_no_sort(nums: Vec<i32>) -> Vec<Vec<i32>> {
        println!("three_sum({:?})", nums);
        use std::collections::HashSet;
        let n = nums.len();
        let mut seen = HashSet::new();
        let mut results = HashSet::new();
        let mut duplicates = HashSet::new();
        for l in 0..n.saturating_sub(2) {
            if duplicates.insert(nums[l]) {
                seen.clear();
                for r in l + 1..n {
                    let complement = -(nums[l] + nums[r]);
                    if seen.contains(&complement) {
                        let mut v = vec![nums[l], complement, nums[r]];
                        v.sort_unstable();
                        results.insert(v);
                    } else {
                        seen.insert(nums[r]);
                    }
                }
            }
        }
        results.into_iter().collect()
    }
    /// Approach 2: Hashset
    /// https://leetcode.com/problems/3sum/solution/
    pub fn three_sum_hashset(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        println!("three_sum({:?})", nums);
        use std::collections::HashSet;
        let n = nums.len();
        nums.sort_unstable();
        let mut seen = HashSet::new();
        let mut results = vec![];
        for l in 0..n.saturating_sub(2) {
            if nums[l] > 0 {
                break;
            }
            if l > 0 && nums[l - 1] == nums[l] {
                continue;
            }
            seen.clear();
            let mut r = l + 1;
            while r < n {
                let complement = -(nums[l] + nums[r]);
                if seen.contains(&complement) {
                    results.push(vec![nums[l], complement, nums[r]]);
                    while r + 1 < n && nums[r] == nums[r + 1] {
                        r += 1;
                    }
                } else {
                    seen.insert(nums[r]);
                }
                r += 1;
            }
        }
        results
    }
    /// Approach 1: Two Pointers
    /// https://leetcode.com/problems/3sum/solution/
    pub fn three_sum_2_pointers(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        println!("three_sum({:?})", nums);
        let n = nums.len();
        nums.sort_unstable();
        let mut results = vec![];
        for l in 0..n.saturating_sub(2) {
            if nums[l] > 0 {
                break;
            }
            if l > 0 && nums[l - 1] == nums[l] {
                continue;
            }
            let (mut m, mut r) = (l + 1, n - 1);
            while m < r {
                match (nums[l] + nums[r] + nums[m]).signum() {
                    -1 => m += 1,
                    1 => r -= 1,
                    _ => {
                        results.push(vec![nums[l], nums[m], nums[r]]);
                        m += 1;
                        while m < r && nums[m - 1] == nums[m] {
                            m += 1;
                        }
                    }
                }
            }
        }
        results
    }

    /// 22:16-22:43
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        println!("three_sum({:?})", nums);
        use std::collections::HashSet;
        let n = nums.len();
        nums.sort_unstable();
        let mut results = HashSet::new();
        let mut duplicates = HashSet::new();
        for l in 0..n.saturating_sub(2) {
            if duplicates.insert(nums[l]) {
                for m in l + 1..n - 1 {
                    let t = -(nums[l] + nums[m]);
                    if let Ok(r) = nums[m + 1..].binary_search(&t) {
                        results.insert(vec![nums[l], nums[m], nums[m + 1 + r]]);
                    }
                }
            }
        }
        results.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_m1p0p1p2m1m4() {
        let n = vec![-1, 0, 1, 2, -1, -4];
        let e = [[-1, -1, 2], [-1, 0, 1]];
        let mut r = Solution::three_sum(n);
        r.sort_unstable();
        assert_eq!(r, e);
    }
    #[test]
    fn n_empty() {
        let e: Vec<Vec<i32>> = vec![];
        let mut r = Solution::three_sum(vec![]);
        r.sort_unstable();
        assert_eq!(r, e);
    }
    #[test]
    fn n_0() {
        let e: Vec<Vec<i32>> = vec![];
        let mut r = Solution::three_sum(vec![0]);
        r.sort_unstable();
        assert_eq!(r, e);
    }
    #[test]
    fn n_m1p0p1p2m1m4m1p2() {
        let n = vec![-1, 0, 1, 2, -1, -4, -1, 2];
        let e = [[-4, 2, 2], [-1, -1, 2], [-1, 0, 1]];
        let mut r = Solution::three_sum(n);
        r.sort_unstable();
        assert_eq!(r, e);
    }

    #[test]
    fn n_000() {
        let mut r = Solution::three_sum(vec![0, 0, 0]);
        r.sort_unstable();
        assert_eq!(r, [[0, 0, 0]]);
    }

    #[test]
    fn n_3000x0() {
        let n = vec![0; 3000];
        let e = [[0, 0, 0]];
        let mut r = Solution::three_sum(n);
        r.sort_unstable();
        assert_eq!(r, e);
    }
}
