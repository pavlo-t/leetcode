#![allow(dead_code)]
/// Global and Local Inversions
/// ===========================
///
/// We have some permutation `A` of `[0, 1, ..., N - 1]`, where `N` is the length of `A`.
///
/// The number of global inversions is the number of `i < j` with `0 <= i < j < N` and `A[i] > A[j]`.
///
/// The number of local inversions is the number of `i` with `0 <= i < N` and `A[i] > A[i+1]`.
///
/// Return `true` if and only if the number of global inversions is equal to the number of local inversions.
///
/// __Note:__
///
/// - `A` will be a permutation of `[0, 1, ..., A.length - 1]`.
/// - `A` will have length in range `[1, 5000]`.
/// - The time limit for this problem has been reduced.
///
/// https://leetcode.com/explore/challenge/card/april-leetcoding-challenge-2021/593/week-1-april-1st-april-7th/3697/
struct Solution;
impl Solution {
    pub fn is_ideal_permutation(a: Vec<i32>) -> bool {
        for i in 0..a.len() {
            if (a[i] - i as i32).abs() > 1 {
                return false;
            }
        }
        true
    }

    pub fn is_ideal_permutation_my(a: Vec<i32>) -> bool {
        for i in 0..a.len() {
            let iv = i as i32;
            if a[i] != iv && a[i] != iv + 1 && a[i] != iv - 1 {
                return false;
            }
        }
        true
    }

    pub fn is_ideal_permutation_v1(a: Vec<i32>) -> bool {
        for i in 0..a.len() {
            if a[i] == i as i32 {
                continue;
            }
            if a[i] == (i + 1) as i32 || (i > 0 && a[i] == (i - 1) as i32) {
                continue;
            }
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_a102_is_ideal() {
        let a = vec![1, 0, 2];
        assert!(Solution::is_ideal_permutation(a));
        // Explanation: There is 1 global inversion, and 1 local inversion.
    }
    #[test]
    fn example2_a120_is_not_ideal() {
        let a = vec![1, 2, 0];
        assert!(!Solution::is_ideal_permutation(a));
        // Explanation: There are 2 global inversions, and 1 local inversion.
    }

    #[test]
    fn a0_is_ideal() {
        let a = vec![0];
        assert!(Solution::is_ideal_permutation(a));
    }
    #[test]
    fn a012354_is_ideal() {
        let a = vec![0, 1, 2, 3, 5, 4];
        assert!(Solution::is_ideal_permutation(a));
    }
    #[test]
    fn a012453_is_not_ideal() {
        let a = vec![0, 1, 2, 4, 5, 3];
        assert!(!Solution::is_ideal_permutation(a));
    }
}
