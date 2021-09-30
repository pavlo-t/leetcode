#![allow(dead_code)]
/// Three Equal Parts
/// =================
///
/// You are given an array `arr` which consists of only zeros and ones,
/// divide the array into __three non-empty parts__ such that all of these parts represent the same binary value.
///
/// If it is possible, return any `[i, j]` with `i + 1 < j`, such that:
///
/// - `arr[0], arr[1], ..., arr[i]` is the first part,
/// - `arr[i + 1], arr[i + 2], ..., arr[j - 1]` is the second part, and
/// - `arr[j], arr[j + 1], ..., arr[arr.length - 1]` is the third part.
/// - All three parts have equal binary values.
///
/// If it is not possible, return `[-1, -1]`.
///
/// Note that the entire part is used when considering what binary value it represents.
/// For example, `[1,1,0]` represents `6` in decimal, not `3`.
/// Also, leading zeros __are allowed__, so `[0,1,1]` and `[1,1]` represent the same value.
///
/// __Constraints:__
///
/// - `3 <= arr.length <= 30_000`
/// - `arr[i]` is `0` or `1`
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/610/week-3-july-15th-july-21st/3817/
struct Solution;
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        fn is_valid(a: &[i32], i: usize, j: usize) -> bool {
            let (mut i1, mut i2, mut i3) = (i, j - 1, a.len() - 1);
            while i1 > 0 && i2 > i + 1 && i3 > j {
                if a[i1] != a[i2] || a[i1] != a[i3] {
                    return false;
                }
                i1 -= 1;
                i2 -= 1;
                i3 -= 1;
            }
            if a[i1] != a[i2] || a[i1] != a[i3] {
                return false;
            }
            if i1 != 0 && !a[..i1].iter().all(|&n| n == 0) {
                return false;
            }
            if i2 != i + 1 && !a[i + 1..i2].iter().all(|&n| n == 0) {
                return false;
            }
            if i3 != j && !a[j..i3].iter().all(|&n| n == 0) {
                return false;
            }
            true
        }
        fn get_i_j_from_lsums(lsums: Vec<i32>, tzs: usize) -> Option<(usize, usize)> {
            let n = lsums.len();
            if lsums[n - 1] == 0 {
                Some((0, n - 1))
            } else {
                let add_tzs = |i, si| {
                    Some(i + tzs)
                        .filter(|&i| i < n && (i > 0 && lsums[i - 1] == si) || lsums[i] == si)
                };
                lsums
                    .last()
                    .and_then(|total| if total % 3 == 0 { Some(total) } else { None })
                    .map(|total| (total / 3, total / 3 * 2))
                    .and_then(|(si, sj)| lsums.iter().position(|&n| n == si).map(|i| (i, si, sj)))
                    .and_then(|(i, si, sj)| add_tzs(i, si).map(|i| (i, sj)))
                    .and_then(|(i, sj)| lsums.iter().position(|&n| n == sj).map(|j| (i, j + 1, sj)))
                    .and_then(|(i, j, sj)| add_tzs(j, sj).map(|j| (i, j)))
            }
        }
        let mut lsums = arr.clone();
        for i in 1..arr.len() {
            lsums[i] += lsums[i - 1];
        }
        let mut trailing_zeros = 0;
        let mut i = arr.len() - 1;
        while i > 0 && arr[i] == 0 {
            trailing_zeros += 1;
            i -= 1;
        }

        get_i_j_from_lsums(lsums, trailing_zeros)
            .filter(|&(i, j)| is_valid(&arr, i, j))
            .map(|(i, j)| vec![i as i32, j as i32])
            .unwrap_or(vec![-1, -1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_1_0_1_0_1_produces_0_3() {
        let arr = vec![1, 0, 1, 0, 1];
        //let arr = vec![1, 0, 1, 0, 1];
        //let sss = vec![1, 1, 2, 2, 3];
        assert_eq!(Solution::three_equal_parts(arr), [0, 3]);
    }
    #[test]
    fn a_1_1_0_1_1_produces_m1m1() {
        let arr = vec![1, 1, 0, 1, 1];
        assert_eq!(Solution::three_equal_parts(arr), [-1, -1]);
    }
    #[test]
    fn a_1_1_0_0_1_produces_0_2() {
        let arr = vec![1, 1, 0, 0, 1];
        assert_eq!(Solution::three_equal_parts(arr), [0, 2]);
    }
    #[test]
    fn a_0_1_0_1_1_produces_1_4() {
        let arr = vec![0, 1, 0, 1, 1];
        assert_eq!(Solution::three_equal_parts(arr), [1, 4]);
    }
    #[test]
    fn a_1_0_1_0_1_0_produces_1_4() {
        let arr = vec![1, 0, 1, 0, 1, 0];
        assert_eq!(Solution::three_equal_parts(arr), [1, 4]);
    }
    #[test]
    fn a_0_1_0_1_0_1_0_produces_2_5() {
        let arr = vec![0, 1, 0, 1, 0, 1, 0];
        assert_eq!(Solution::three_equal_parts(arr), [2, 5]);
    }
    #[test]
    fn a_1_1_1_produces_0_2() {
        let arr = vec![1, 1, 1];
        assert_eq!(Solution::three_equal_parts(arr), [0, 2]);
    }
    #[test]
    fn a_0_0_0_0_0_produces_0_1() {
        let arr = vec![0, 0, 0, 0, 0];
        assert_eq!(Solution::three_equal_parts(arr), [0, 4]);
    }
}
