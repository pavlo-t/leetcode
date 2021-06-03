#![allow(dead_code)]
/// Minimum Operations to Make Array Equal
/// ======================================
///
/// You have an array `arr` of length `n` where `arr[i] = (2 * i) + 1` for all valid values of `i`
/// (i.e. `0 <= i < n`).
///
/// In one operation, you can select two indices `x` and `y` where `0 <= x, y < n`
/// and subtract `1` from `arr[x]` and add `1` to `arr[y]`
/// (i.e. perform `arr[x] -= 1` and `arr[y] += 1`).
/// The goal is to make all the elements of the array __equal__.
/// It is __guaranteed__ that all the elements of the array can be made equal using some operations.
///
/// Given an integer `n`, the length of the array,
/// return _the minimum number of operations_ needed to make all the elements of arr equal.
///
/// __Constraints:__
///
/// - `1 <= n <= 10^4`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/593/week-1-april-1st-april-7th/3698/
struct Solution;
impl Solution {
    /// ```text
    /// n=5: S=(5-1)+(5-3)      = 6=2*3=(5-1)/2*(5+1)/2=(5*5-5+5-1)/4=(5*5-1)/4=5*5/4-1/4=5*5//4
    /// n=7: S=(7-1)+(7-3)+(7-5)=12=3*4=(7-1)/2*(7+1)/2=(7*7-7+7-1)/4=(7*7-1)/4=7*7/4-1/4=7*7//4
    /// n=6: S=(6-1)+(6-3)+(6-5)      = 9=3*3=(6/2)*(6/2) = 6*6/4
    /// n=8: S=(8-1)+(8-3)+(8-5)+(8-7)=16=4*4=(8/2)*(8/2) = 8*8/4
    ///
    /// for any even n: (n-1)+(n-3)+..+(n-(n-1))=(n/2)*(n/2) = n*n/4
    /// for any odd n: (n-1)+..+(n-n)=((n-1)/2)*((n+1)/2)=(n*n-n+n-1)/4=(n*n-1)/4=n*n/4-1/4 = n*n//4
    /// ```
    pub fn min_operations(n: i32) -> i32 {
        n * n / 4
    }
    pub fn min_operations_my_brute_force(n: i32) -> i32 {
        (1..n).step_by(2).fold(0, |acc, i| acc + n - i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n1_produces_0() {
        assert_eq!(Solution::min_operations(1), 0);
        // [1]
    }
    #[test]
    fn n2_produces_1() {
        assert_eq!(Solution::min_operations(2), 1);
        // [1,3] [2,2]
    }
    #[test]
    fn n3_produces_2() {
        assert_eq!(Solution::min_operations(3), 2);
        // [1,3,5] [2,3,4] [3,3,3]
    }
    #[test]
    fn n4_produces_4() {
        assert_eq!(Solution::min_operations(4), 4);
        // [1,3,5,7] [2,3,5,6] [3,3,5,5] [4,3,5,4] [4,4,4,4]
    }
    #[test]
    fn n5_produces_6() {
        assert_eq!(Solution::min_operations(5), 6);
        // [1,3,5,7,9] [2,3,5,7,8] [3,3,5,7,7] [4,3,5,7,6] [5,3,5,7,5] [5,4,5,6,5] [5,5,5,5,5]
    }
    #[test]
    fn n6_produces_9() {
        assert_eq!(Solution::min_operations(6), 9);
        // [1,3,5,7,9,11] [2,3,5,7,9,10] [3,3,5,7,9,9] [4,3,5,7,9,8] [5,3,5,7,9,7]
        //  [6,3,5,7,9,6] [6,4,5,7,8,6] [6,5,5,7,7,6] [6,6,5,7,6,6] [6,6,6,6,6,6]
    }
    #[test]
    fn n7_produces_12() {
        assert_eq!(Solution::min_operations(7), 12);
    }
    #[test]
    fn n8_produces_16() {
        assert_eq!(Solution::min_operations(8), 16);
    }
    #[test]
    fn n10000_produces_25_000_000() {
        assert_eq!(Solution::min_operations(10_000), 25_000_000);
    }
}
