#![allow(dead_code)]
/// 343. Integer Break
/// ==================
///
/// Given an integer `n`, break it into the sum of `k` __positive integers__, where `k >= 2`,
/// and maximize the product of those integers.
///
/// Return _the maximum product you can get_.
///
/// __Constraints:__
///
/// - `2 <= n <= 58`
///
/// https://leetcode.com/problems/integer-break/
struct Solution;
impl Solution {
    pub fn integer_break_rec(n: i32) -> i32 {
        println!("integer_break({})", n);
        fn rec(n: i32) -> i32 {
            match n {
                0..=2 => 1,
                3 => 2,
                4 => 4,
                5 => 6,
                6 => 9,
                n => (2..n / 2).map(|d| d * rec(n - d)).max().unwrap(),
            }
        }
        rec(n)
    }
    pub fn integer_break_rec_with_memo(n: i32) -> i32 {
        println!("integer_break({})", n);
        fn rec(n: usize, memo: &mut Vec<usize>) -> usize {
            if memo[n] != usize::MAX {
                memo[n]
            } else {
                memo[n] = match n {
                    0..=2 => 1,
                    3 => 2,
                    4 => 4,
                    5 => 6,
                    6 => 9,
                    n => (2..n / 2).map(|d| d * rec(n - d, memo)).max().unwrap(),
                };
                memo[n]
            }
        }
        let n = n as usize;
        rec(n, &mut vec![usize::MAX; n + 1]) as i32
    }
    pub fn integer_break_dp(n: i32) -> i32 {
        println!("integer_break({})", n);
        let l = n as usize;
        let mut dp = vec![1, 1, 1, 2, 4, 6, 9];
        for i in 7..=l {
            let mut curr = 0;
            for d in 2..i / 2 {
                curr = curr.max(d as i32 * dp[i - d]);
            }
            dp.push(curr);
        }
        dp[l]
    }
    pub fn integer_break_rec_n_minus_3(n: i32) -> i32 {
        println!("integer_break({})", n);
        fn rec(n: i32) -> i32 {
            match n {
                0..=2 => 1,
                3 => 2,
                4 => 4,
                5 => 6,
                6 => 9,
                n => 3 * rec(n - 3),
            }
        }
        rec(n)
    }
    pub fn integer_break(n: i32) -> i32 {
        println!("integer_break({})", n);
        match n {
            2..=3 => n - 1,
            4 => 4,
            5 => 6,
            6 => 9,
            n => {
                let (mut a, mut b, mut c) = (4, 6, 9);
                for _ in 6..n {
                    std::mem::swap(&mut a, &mut b);
                    std::mem::swap(&mut b, &mut c);
                    c *= 3;
                }
                c
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn  n2() { assert_eq!(Solution::integer_break( 2),   1); } // 1*1
    #[rustfmt::skip] #[test] fn  n3() { assert_eq!(Solution::integer_break( 3),   2); } // 1*2
    #[rustfmt::skip] #[test] fn  n4() { assert_eq!(Solution::integer_break( 4),   4); } // 2*2
    #[rustfmt::skip] #[test] fn  n5() { assert_eq!(Solution::integer_break( 5),   6); } // 3*2
    #[rustfmt::skip] #[test] fn  n6() { assert_eq!(Solution::integer_break( 6),   9); } // 3*3
    #[rustfmt::skip] #[test] fn  n7() { assert_eq!(Solution::integer_break( 7),  12); } // 3*4
    #[rustfmt::skip] #[test] fn  n8() { assert_eq!(Solution::integer_break( 8),  18); } // 3*3*2
    #[rustfmt::skip] #[test] fn  n9() { assert_eq!(Solution::integer_break( 9),  27); } // 3*3*3
    #[rustfmt::skip] #[test] fn n10() { assert_eq!(Solution::integer_break(10),  36); } // 3*3*4
    #[rustfmt::skip] #[test] fn n11() { assert_eq!(Solution::integer_break(11),  54); } // 3*3*3*2
    #[rustfmt::skip] #[test] fn n12() { assert_eq!(Solution::integer_break(12),  81); } // 3*3*3*3
    #[rustfmt::skip] #[test] fn n13() { assert_eq!(Solution::integer_break(13), 108); } // 3*3*3*4
    #[rustfmt::skip] #[test] fn n14() { assert_eq!(Solution::integer_break(14), 162); } // 3*3*3*3*2
    #[rustfmt::skip] #[test] fn n15() { assert_eq!(Solution::integer_break(15), 243); } // 3*3*3*3*3

    //#[ignore]
    #[rustfmt::skip] #[test] fn n58() { assert_eq!(Solution::integer_break(58), 1_549_681_956); }
}
