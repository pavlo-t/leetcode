#![allow(dead_code)]
/// Partition to K Equal Sum Subsets
/// ================================
///
/// Given an integer array `nums` and an integer `k`,
/// return `true` if it is possible to divide this array into `k` non-empty subsets whose sums are all equal.
///
/// __Constraints:__
///
/// - `1 <= k <= nums.length <= 16`
/// - `1 <= nums[i] <= 10_000`
/// - The frequency of each element is in the range `[1, 4]`.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/640/week-5-september-29th-september-30th/3993/
struct Solution;
impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        println!("can_partition_k_subsets({:?}, {})", nums, k);

        #[rustfmt::skip]
        let (total, max) = nums.iter().fold((0, 0), |(total, max), &n| (total + n, max.max(n)));
        let subset_sum = total / k;
        if total % k != 0 || subset_sum < max {
            false
        } else {
            fn bts(sums: &mut Vec<i32>, seen: &mut Vec<bool>, nums: &[i32], t: i32) -> bool {
                //println!("bts({:?}, {:?})", sums, seen);
                if let Some((i, _)) = seen.iter().enumerate().find(|(_, &s)| !s) {
                    seen[i] = true;
                    for j in 0..sums.len() {
                        if sums[j] + nums[i] <= t {
                            sums[j] += nums[i];
                            if bts(sums, seen, nums, t) {
                                return true;
                            }
                            sums[j] -= nums[i];
                            if sums[j] == 0 {
                                break;
                            }
                        }
                    }
                    seen[i] = false;
                    false
                } else {
                    sums.iter().all(|&s| s == t)
                }
            }

            nums.sort_unstable_by(|a, b| b.cmp(a));
            bts(
                &mut vec![0; k as usize],
                &mut vec![false; nums.len()],
                &nums,
                subset_sum,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_4_3_2_3_5_2_1_k_4() {
        let n = vec![4, 3, 2, 3, 5, 2, 1];
        let k = 4;
        assert!(Solution::can_partition_k_subsets(n, k));
        // Explanation: It's possible to divide it into 4 subsets (5), (1, 4), (2,3), (2,3) with equal sums.
    }
    #[test]
    fn n_1_2_3_4_k_3() {
        let n = vec![1, 2, 3, 4];
        let k = 3;
        assert!(!Solution::can_partition_k_subsets(n, k));
    }
    #[test]
    fn n_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_k_8() {
        let n = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let k = 8;
        assert!(Solution::can_partition_k_subsets(n, k));
    }
}
