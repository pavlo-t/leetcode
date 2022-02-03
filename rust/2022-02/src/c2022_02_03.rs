#![allow(dead_code)]
/// 454. 4Sum II
/// ============
///
/// Given four integer arrays `nums1`, `nums2`, `nums3`, and `nums4` all of length `n`,
/// return the number of tuples `(i, j, k, l)` such that:
///
/// - `0 <= i, j, k, l < n`
/// - `nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0`
///
/// __Constraints:__
///
/// - `n == nums1.length`
/// - `n == nums2.length`
/// - `n == nums3.length`
/// - `n == nums4.length`
/// - `1 <= n <= 200`
/// - `-2**28 <= nums1[i], nums2[i], nums3[i], nums4[i] <= 2**28`
///
/// https://leetcode.com/problems/4sum-ii/
struct Solution;
impl Solution {
    pub fn four_sum_count_brute_force(
        ns1: Vec<i32>,
        ns2: Vec<i32>,
        ns3: Vec<i32>,
        ns4: Vec<i32>,
    ) -> i32 {
        let mut result = 0;
        for &n1 in &ns1 {
            for &n2 in &ns2 {
                for &n3 in &ns3 {
                    for &n4 in &ns4 {
                        if n1 + n2 + n3 + n4 == 0 {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }

    pub fn four_sum_count_1_hash_map(
        ns1: Vec<i32>,
        ns2: Vec<i32>,
        ns3: Vec<i32>,
        ns4: Vec<i32>,
    ) -> i32 {
        use std::collections::HashMap;

        let ns4 = ns4.into_iter().fold(HashMap::new(), |mut rsf, n| {
            *rsf.entry(n).or_insert(0) += 1;
            rsf
        });

        let mut result = 0;
        for &n1 in &ns1 {
            for &n2 in &ns2 {
                for &n3 in &ns3 {
                    if let Some(count) = ns4.get(&(0 - n1 - n2 - n3)) {
                        result += count;
                    }
                }
            }
        }
        result
    }

    pub fn four_sum_count(ns1: Vec<i32>, ns2: Vec<i32>, ns3: Vec<i32>, ns4: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut ns34: HashMap<i32, i32> = HashMap::new();
        for n3 in ns3 {
            for &n4 in &ns4 {
                *ns34.entry(n3 + n4).or_default() += 1;
            }
        }

        let mut result = 0;
        for &n1 in &ns1 {
            for &n2 in &ns2 {
                if let Some(count) = ns34.get(&(0 - n1 - n2)) {
                    result += count;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n0_n0_n0_p1() {assert_eq!(Solution::four_sum_count(vec![0],vec![0],vec![0],vec![1]), 0);}
    #[rustfmt::skip] #[test] fn n0_n0_n0_n0() {assert_eq!(Solution::four_sum_count(vec![0],vec![0],vec![0],vec![0]), 1);}

    #[test]
    fn p1p2_m2m1_m1p2_n0p2() {
        let n1 = vec![1, 2];
        let n2 = vec![-2, -1];
        let n3 = vec![-1, 2];
        let n4 = vec![0, 2];
        assert_eq!(Solution::four_sum_count(n1, n2, n3, n4), 2);
        // Explanation:
        // The two tuples are:
        // 1. (0, 0, 0, 1) -> nums1[0] + nums2[0] + nums3[0] + nums4[1] = 1 + (-2) + (-1) + 2 = 0
        // 2. (1, 1, 0, 0) -> nums1[1] + nums2[1] + nums3[0] + nums4[0] = 2 + (-1) + (-1) + 0 = 0
    }

    #[test]
    fn ns_0until200_x_4() {
        let n1 = (0..200).collect();
        let n2 = (0..200).collect();
        let n3 = (0..200).collect();
        let n4 = (0..200).collect();
        assert_eq!(Solution::four_sum_count(n1, n2, n3, n4), 1);
    }
}
