#![allow(dead_code)]
/// Beautiful Arrangement II
/// ========================
///
/// Given two integers `n` and `k`, you need to construct a list which contains `n` different
/// positive integers ranging from `1` to `n` and obeys the following requirement:
///
/// Suppose this list is `[a1, a2, a3, ... , an]`, then the list
/// `[|a1 - a2|, |a2 - a3|, |a3 - a4|, ... , |an-1 - an|]` has exactly `k` distinct integers.
///
/// If there are multiple answers, print any of them.
///
/// __Note:__
///
/// - The `n` and `k` are in the range `1 <= k < n <= 10_000`.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/594/week-2-april-8th-april-14th/3705/
struct Solution;
impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut result = (1..(n - k)).collect::<Vec<_>>();
        for i in 0..=k {
            if i % 2 == 0 {
                result.push(n - k + i / 2);
            } else {
                result.push(n - i / 2);
            }
        }
        result
    }

    pub fn construct_array_my(n: i32, mut k: i32) -> Vec<i32> {
        let mut result = vec![1];
        let mut l = 2;
        let mut r = n;
        for i in 1..n as usize {
            if (r - result[i - 1]).abs() == k {
                result.push(r);
                r -= 1;
                k -= 1;
            } else {
                result.push(l);
                if (l - result[i - 1]).abs() == k {
                    k -= 1;
                }
                l += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::construct_array(3, 1), [1, 2, 3]);
        // Explanation:
        // The [1, 2, 3] has three different positive integers ranging from 1 to 3,
        // and the [1, 1] has exactly 1 distinct integer: 1.
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::construct_array(3, 2), [1, 3, 2]);
        // Explanation:
        // The [1, 3, 2] has three different positive integers ranging from 1 to 3,
        // and the [2, 1] has exactly 2 distinct integers: 1 and 2.
    }

    #[test]
    fn n2k1() {
        assert_eq!(Solution::construct_array(2, 1), [1, 2]);
    }
    #[test]
    fn n4k1() {
        assert_eq!(Solution::construct_array(4, 1), [1, 2, 3, 4]);
    }
    #[test]
    fn n4k2() {
        assert_eq!(Solution::construct_array(4, 2), [1, 2, 4, 3]);
    }
    #[test]
    fn n4k3() {
        assert_eq!(Solution::construct_array(4, 3), [1, 4, 2, 3]);
    }
    #[test]
    fn n9k8() {
        assert_eq!(Solution::construct_array(9, 8), [1, 9, 2, 8, 3, 7, 4, 6, 5]);
    }
    #[test]
    fn n10k9() {
        assert_eq!(
            Solution::construct_array(10, 9),
            [1, 10, 2, 9, 3, 8, 4, 7, 5, 6]
        );
    }
    #[test]
    fn n10k8() {
        assert_eq!(
            Solution::construct_array(10, 8),
            [1, 2, 10, 3, 9, 4, 8, 5, 7, 6]
        );
    }

    mod performance {
        use super::*;

        #[test]
        fn n10000k9999() {
            let mut e = Vec::new();
            let mut l = 1;
            let mut r = 10_000;
            for i in 1..=10000 {
                if i % 2 == 1 {
                    e.push(l);
                    l += 1;
                } else {
                    e.push(r);
                    r -= 1;
                }
            }
            assert_eq!(Solution::construct_array(10_000, 9_999), e);
        }
    }
}
