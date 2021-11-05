#![allow(dead_code)]
/// 441. Arranging Coins
/// ====================
///
/// You have `n` coins and you want to build a staircase with these coins.
/// The staircase consists of `k` rows where the `i`th row has exactly `i` coins.
/// The last row of the staircase __may be__ incomplete.
///
/// Given the integer `n`,
/// return _the number of __complete rows__ of the staircase you will build_.
///
/// __Constraints:__
///
/// `1 <= n <= 2^31 - 1`
///
/// https://leetcode.com/problems/arranging-coins/
struct Solution;
impl Solution {
    /// Approach 2: Math
    /// https://leetcode.com/problems/arranging-coins/solution/
    pub fn arrange_coins(n: i32) -> i32 {
        println!("arrange_coins({})", n);
        ((2.0 * n as f64 + 0.25).sqrt() - 0.5) as i32
    }
    /// Approach 1: Binary Search
    /// https://leetcode.com/problems/arranging-coins/solution/
    pub fn arrange_coins_leetcode_binary_search(n: i32) -> i32 {
        println!("arrange_coins({})", n);
        let n = n as i64;
        let (mut l, mut r) = (0, n);
        while l <= r {
            let k = l + (r - l) / 2;
            let curr = k * (k + 1) / 2;
            if curr == n {
                return k as i32;
            }
            if n < curr {
                r = k - 1;
            } else {
                l = k + 1;
            }
        }
        r as i32
    }
    /// 15:45-15:47
    pub fn arrange_coins_my_iter(mut n: i32) -> i32 {
        println!("arrange_coins({})", n);
        let mut r = 0;
        while n > r {
            n -= r + 1;
            r += 1;
        }
        r
    }
    /// 15:42-15:45
    pub fn arrange_coins_rec(n: i32) -> i32 {
        println!("arrange_coins({})", n);
        fn rec(n: i32, r: i32) -> i32 {
            if n == r {
                r
            } else if n < r {
                r - 1
            } else {
                rec(n - r, r + 1)
            }
        }
        rec(n, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n1() { assert_eq!(Solution::arrange_coins(1), 1); }
    #[rustfmt::skip] #[test] fn n2() { assert_eq!(Solution::arrange_coins(2), 1); }
    #[rustfmt::skip] #[test] fn n3() { assert_eq!(Solution::arrange_coins(3), 2); }
    #[rustfmt::skip] #[test] fn n4() { assert_eq!(Solution::arrange_coins(4), 2); }
    #[test]
    fn n5() {
        assert_eq!(Solution::arrange_coins(5), 2);
        // Explanation: Because the 3rd row is incomplete, we return 2.
        // c
        // c c
        // c c _
    }
    #[rustfmt::skip] #[test] fn n6() { assert_eq!(Solution::arrange_coins(6), 3); }
    #[rustfmt::skip] #[test] fn n7() { assert_eq!(Solution::arrange_coins(7), 3); }
    #[test]
    fn n8() {
        assert_eq!(Solution::arrange_coins(8), 3);
        // Explanation: Because the 4th row is incomplete, we return 3.
        // c
        // c c
        // c c c
        // c c _ _
    }
    #[rustfmt::skip] #[test] fn  n9() { assert_eq!(Solution::arrange_coins( 9),  3); }
    #[rustfmt::skip] #[test] fn n10() { assert_eq!(Solution::arrange_coins(10),  4); }
    #[rustfmt::skip] #[test] fn n14() { assert_eq!(Solution::arrange_coins(14),  4); }
    #[rustfmt::skip] #[test] fn n15() { assert_eq!(Solution::arrange_coins(15),  5); }
    #[rustfmt::skip] #[test] fn n16() { assert_eq!(Solution::arrange_coins(16),  5); }
    #[rustfmt::skip] #[test] fn n17() { assert_eq!(Solution::arrange_coins(17),  5); }
    #[rustfmt::skip] #[test] fn n21() { assert_eq!(Solution::arrange_coins(21),  6); }
    #[rustfmt::skip] #[test] fn n28() { assert_eq!(Solution::arrange_coins(28),  7); }
    #[rustfmt::skip] #[test] fn n32() { assert_eq!(Solution::arrange_coins(32),  7); }
    #[rustfmt::skip] #[test] fn n36() { assert_eq!(Solution::arrange_coins(36),  8); }
    #[rustfmt::skip] #[test] fn n45() { assert_eq!(Solution::arrange_coins(45),  9); }
    #[rustfmt::skip] #[test] fn n55() { assert_eq!(Solution::arrange_coins(55), 10); }
    #[rustfmt::skip] #[test] fn n66() { assert_eq!(Solution::arrange_coins(66), 11); }
    #[rustfmt::skip] #[test] fn n1000000000() { assert_eq!(Solution::arrange_coins(1000000000), 44720); }
    #[rustfmt::skip] #[test] fn n2147483647() { assert_eq!(Solution::arrange_coins(2147483647), 65535); }
}
