#![allow(dead_code, unused_imports)]

/// ### 5619. Minimum Incompatibility
///
/// https://leetcode.com/contest/weekly-contest-218/problems/minimum-incompatibility/
struct Solution;

impl Solution {
    /// https://leetcode.com/problems/minimum-incompatibility/discuss/961470/Java-backtrack-solution-with-optimizations
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashSet;

        struct Backtrack {
            nums: Vec<i32>,
            rsf: i32,
            bucket_size: usize,
        }
        impl Backtrack {
            fn new(nums: Vec<i32>, k: i32) -> Backtrack {
                let bucket_size = nums.len() / k as usize;
                Backtrack { nums, rsf: std::i32::MAX, bucket_size }
            }

            fn backtrack(&mut self, idx: usize, sets: Vec<HashSet<i32>>, acc: i32) {
                if idx >= self.nums.len() {
                    self.rsf = self.rsf.min(acc);
                    return;
                }

                let mut acc = acc;
                let mut visited = Vec::new();

                for (si, set) in sets.iter().enumerate() {
                    if set.contains(&self.nums[idx]) || set.len() == self.bucket_size || visited.contains(set) {
                        continue;
                    }

                    let impact = Backtrack::compute_impact(set, self.nums[idx]);
                    acc += impact;
                    if acc < self.rsf {
                        let mut sets = sets.clone();
                        sets[si].insert(self.nums[idx]);
                        self.backtrack(idx + 1, sets, acc);
                    }
                    acc -= impact;
                    visited.push(set.clone());
                }
            }

            fn compute_impact(set: &HashSet<i32>, number: i32) -> i32 {
                match set.iter().fold((std::i32::MAX, std::i32::MIN),
                                      |(min, max), &v| (min.min(v), max.max(v)),
                ) {
                    (std::i32::MAX, _) => 0,
                    (min, _) if number < min => min - number,
                    (_, max) if number > max => number - max,
                    _ => 0,
                }
            }
        }

        let mut backtrack = Backtrack::new(nums, k);
        backtrack.backtrack(0, vec![HashSet::new(); k as usize], 0);
        match backtrack.rsf {
            std::i32::MAX => -1,
            v => v
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 1, 4];
        let k = 2;
        let expected = 4;
        assert_eq!(Solution::minimum_incompatibility(nums, k), expected);
        // Explanation: The optimal distribution of subsets is [1,2] and [1,4].
        // The incompatibility is (2-1) + (4-1) = 4.
        // Note that [1,1] and [2,4] would result in a smaller sum, but the first subset contains 2 equal elements.
    }

    #[test]
    fn example2() {
        let nums = vec![6, 3, 8, 1, 3, 1, 2, 2];
        let k = 4;
        let expected = 6;
        assert_eq!(Solution::minimum_incompatibility(nums, k), expected);
        // Explanation: The optimal distribution of subsets is [1,2], [2,3], [6,8], and [1,3].
        // The incompatibility is (2-1) + (3-2) + (8-6) + (3-1) = 6.
    }

    #[test]
    fn example3() {
        let nums = vec![5, 3, 3, 6, 3, 3];
        let k = 3;
        let expected = -1;
        assert_eq!(Solution::minimum_incompatibility(nums, k), expected);
        // Explanation: It is impossible to distribute nums into 3 subsets where no two elements are equal in the same subset.
    }

    #[test]
    fn test_16nums_k02() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let k = 2;
        let expected = 14;
        assert_eq!(Solution::minimum_incompatibility(nums, k), expected);
    }

    #[test]
    fn test_16nums_k04() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let k = 4;
        let expected = 12;
        assert_eq!(Solution::minimum_incompatibility(nums, k), expected);
    }

    #[test]
    fn test_16nums_k08() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let k = 8;
        let expected = 8;
        assert_eq!(Solution::minimum_incompatibility(nums, k), expected);
    }

    #[test]
    fn test_16nums_k16() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let k = 16;
        let expected = 0;
        assert_eq!(Solution::minimum_incompatibility(nums, k), expected);
    }
}