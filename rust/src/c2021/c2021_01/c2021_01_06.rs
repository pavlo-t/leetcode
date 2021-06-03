#![allow(dead_code)]

/// ### Kth Missing Positive Number
/// https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3594/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        if k < arr[0] {
            return k;
        }

        let mut l = 0;
        let mut r = arr.len() - 1;

        while l <= r {
            let m = l + (r - l) / 2;
            if (arr[m] - (m as i32) - 1) < k {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        l as i32 + k
    }

    pub fn find_kth_positive_iteration(arr: Vec<i32>, k: i32) -> i32 {
        let mut i = 0;
        while i < arr.len() && (arr[i] - i as i32 - 1) < k {
            i += 1;
        }
        i as i32 + k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_a2_3_4_7_11_k5_is_9() {
        let arr = vec![2, 3, 4, 7, 11];
        let k = 5;
        assert_eq!(Solution::find_kth_positive(arr, k), 9);
        // Explanation: The missing positive integers are [1,5,6,8,9,10,12,13,...]. The 5th missing positive integer is 9.
    }

    #[test]
    fn example2_a1_2_3_4_k2_is_6() {
        let arr = vec![1, 2, 3, 4];
        let k = 2;
        assert_eq!(Solution::find_kth_positive(arr, k), 6);
        // Explanation: The missing positive integers are [5,6,7,...]. The 2nd missing positive integer is 6.
    }

    #[test]
    fn a1_k1_is_2() {
        let arr = vec![1];
        let k = 1;
        assert_eq!(Solution::find_kth_positive(arr, k), 2);
    }

    #[test]
    fn a1to1000_k1_is_2000() {
        let arr = (1..=1000).collect();
        let k = 1000;
        assert_eq!(Solution::find_kth_positive(arr, k), 2000);
    }

    #[test]
    fn a1001to2000_k1000_is_1000() {
        let arr = (1001..=2000).collect();
        let k = 1000;
        assert_eq!(Solution::find_kth_positive(arr, k), 1000);
    }
}
