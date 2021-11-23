#![allow(dead_code)]
/// 952. Largest Component Size by Common Factor
/// ============================================
///
/// You are given an integer array of unique positive integers `nums`.
/// Consider the following graph:
///
/// - There are `nums.length` nodes, labeled `nums[0]` to `nums[nums.length - 1]`,
/// - There is an undirected edge between `nums[i]` and `nums[j]`
///   if `nums[i]` and `nums[j]` share a common factor greater than `1`.
///
/// Return _the size of the largest connected component in the graph_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 20_000`
/// - `1 <= nums[i] <= 100_000`
/// - All the values of `nums` are __unique__.
///
/// https://leetcode.com/problems/largest-component-size-by-common-factor/
struct Solution;
impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        println!("largest_component_size({:?})", nums);
        let max = nums.iter().max().unwrap().to_owned() as usize;
        let mut nums_factors = vec![vec![]; max + 1];
        for i in 2..=max {
            if nums_factors[i].is_empty() {
                for j in (i..=max).step_by(i) {
                    nums_factors[j].push(i);
                }
            }
        }
        let mut factor_idxs = vec![vec![]; max as usize + 1];
        for (i, &n) in nums.iter().enumerate() {
            for &f in nums_factors[n as usize].iter() {
                factor_idxs[f].push(i);
            }
        }
        fn max_island(
            i: usize,
            ns: &[i32],
            nums_factors: &mut Vec<Vec<usize>>,
            factor_idxs: &mut Vec<Vec<usize>>,
            seen: &mut Vec<bool>,
        ) -> i32 {
            seen[i] = true;
            let mut result = 1;
            let n = ns[i] as usize;
            nums_factors.push(vec![]);
            let factors = nums_factors.swap_remove(n);
            for f in factors {
                factor_idxs.push(vec![]);
                let idxs = factor_idxs.swap_remove(f);
                for j in idxs {
                    if !seen[j] {
                        result += max_island(j, ns, nums_factors, factor_idxs, seen);
                    }
                }
            }
            result
        }

        let mut result = 0;
        let mut seen = vec![false; nums.len()];
        for i in 0..nums.len() {
            result = result.max(max_island(
                i,
                &nums,
                &mut nums_factors,
                &mut factor_idxs,
                &mut seen,
            ));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_4_6_15_35() {
        let n = vec![4, 6, 15, 35];
        assert_eq!(Solution::largest_component_size(n), 4);
    }
    #[test]
    fn n_20_50_9_63() {
        let n = vec![20, 50, 9, 63];
        assert_eq!(Solution::largest_component_size(n), 2);
    }
    #[test]
    fn n_2_3_6_7_4_12_21_39() {
        let n = vec![2, 3, 6, 7, 4, 12, 21, 39];
        assert_eq!(Solution::largest_component_size(n), 8);
    }

    /// If getting stack overflow:
    /// Add `RUST_MIN_STACK=67108864` to env:
    /// RUST_MIN_STACK=67108864 cargo test --lib c2021_11_23
    #[test]
    fn n_80001_to_100000() {
        let n = (80001..=100_000).collect();
        assert_eq!(Solution::largest_component_size(n), 18_245);
    }
    #[test]
    fn test_103_n_2_to_40000_by_2() {
        let n = (2..=40_000).step_by(2).collect();
        assert_eq!(Solution::largest_component_size(n), 20_000);
    }
}
