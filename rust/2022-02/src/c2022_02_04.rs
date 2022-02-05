#![allow(dead_code)]
/// 525. Contiguous Array
/// =====================
///
/// Given a binary array `nums`,
/// return _the maximum length of a contiguous subarray with an equal number of `0` and `1`_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `nums[i]` is either `0` or `1`
///
/// https://leetcode.com/problems/contiguous-array/
struct Solution;
impl Solution {
    pub fn find_max_length_my_binary_search_wrong(nums: Vec<i32>) -> i32 {
        fn can_get_len(len: usize, nums: &[i32]) -> bool {
            let len = len + len % 2;
            if len > nums.len() {
                false
            } else {
                let t = (len / 2) as i32;
                let mut s: i32 = nums.iter().take(len).sum();
                for r in len..nums.len() {
                    if s == t {
                        return true;
                    }
                    s += nums[r];
                    s -= nums[r - len];
                }
                s == t
            }
        }

        let (mut l, mut r) = (0, nums.len());
        println!("  can_get_len(794, &nums): {}", can_get_len(794, &nums));
        while l < r {
            let m = l + (r - l) / 2 + 1;
            print!(" l:{l},r:{r};  m:{m}");
            if can_get_len(m, &nums) {
                println!("   can_get: true");
                l = m;
            } else {
                println!("   can_get: false");
                r = m - 1;
            }
        }
        l as i32
    }

    pub fn find_max_length_brute_force(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut len = n - n % 2;
        while len > 0 {
            let target = (len / 2) as i32;
            for l in 0..n - len + 1 {
                let sum: i32 = nums[l..l + len].iter().sum();
                if sum == target {
                    return len as i32;
                }
            }
            len -= 2;
        }
        0
    }

    pub fn find_max_length_precalculate_sums(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut len = n - n % 2;
        let l_sums = (1..=n).fold(vec![0; n + 1], |mut result, i| {
            result[i] = result[i - 1] + nums[i - 1];
            result
        });
        while len > 0 {
            let target = (len / 2) as i32;
            for l in 0..n - len + 1 {
                let sum = l_sums[l + len] - l_sums[l];
                if sum == target {
                    return len as i32;
                }
            }
            len -= 2;
        }
        0
    }

    /// Approach #2 Using Extra Array [Accepted]
    /// https://leetcode.com/problems/contiguous-array/solution/
    pub fn find_max_length_leetcode_approach_2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut arr: Vec<i32> = vec![-2; n * 2 + 1];
        arr[n] = -1;
        let mut max_len = 0;
        let mut count = 0;
        for i in 0..n {
            count = count + if nums[i] == 0 { -1 } else { 1 };
            let arr_i = (count + n as i32) as usize;
            if arr[arr_i] >= -1 {
                max_len = max_len.max(i as i32 - arr[arr_i]);
            } else {
                arr[arr_i] = i as i32;
            }
        }
        max_len
    }

    /// Approach #3 Using HashMap [Accepted]
    /// https://leetcode.com/problems/contiguous-array/solution/
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let n = nums.len();
        let mut map = HashMap::new();
        map.insert(0, -1);
        let mut max_len = 0;
        let mut count = 0;
        for i in 0..n {
            count = count + if nums[i] == 0 { -1 } else { 1 };
            if let Some(c) = map.get(&count) {
                max_len = max_len.max(i as i32 - c);
            } else {
                map.insert(count, i as i32);
            }
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n0() {
        assert_eq!(Solution::find_max_length(vec![0]), 0);
    }
    #[test]
    fn n1() {
        assert_eq!(Solution::find_max_length(vec![1]), 0);
    }
    #[test]
    fn n00() {
        assert_eq!(Solution::find_max_length(vec![0, 0]), 0);
    }
    #[test]
    fn n11() {
        assert_eq!(Solution::find_max_length(vec![1, 1]), 0);
    }

    #[test]
    fn n01() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
        // Explanation: [0, 1] is the longest contiguous subarray with an equal number of 0 and 1.
    }
    #[test]
    fn n010() {
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
        // Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
    }

    #[test]
    fn test_551() {
        let n = vec![
            0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0,
            1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1,
            1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
            0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0,
            1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1,
            0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0,
            0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1,
            1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1,
            1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1,
            1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0,
            1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0,
            1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0,
            0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1,
            0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0,
            1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1,
            0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
            0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1,
            1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0,
            1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1,
            0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0,
            1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1,
            0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1,
            1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1,
            0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0,
            1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1,
            0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1,
            1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0,
            0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0,
            1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1,
            1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0,
            1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1,
            1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1,
        ];
        assert_eq!(Solution::find_max_length(n), 794);
    }

    //#[ignore]
    #[test]
    fn n_0_repeat_100000() {
        assert_eq!(Solution::find_max_length(vec![0; 100000]), 0);
    }
    //#[ignore]
    #[test]
    fn n_1_repeat_100000() {
        assert_eq!(Solution::find_max_length(vec![1; 100000]), 0);
    }
    #[test]
    fn n_01_repeat_50000() {
        let n = (0..100000).map(|i| i % 2).collect();
        assert_eq!(Solution::find_max_length(n), 100000);
    }
}
