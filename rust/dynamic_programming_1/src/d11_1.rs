#![allow(dead_code)]
/// 264. Ugly Number II
/// ===================
///
/// An __ugly number__ is a positive integer whose prime factors are limited to `2`, `3`, and `5`.
///
/// Given an integer `n`, return _the `n`th __ugly number___.
///
/// __Constraints:__
///
/// - `1 <= n <= 1690`
///
/// https://leetcode.com/problems/ugly-number-ii/
struct Solution;
impl Solution {
    /// https://leetcode.com/submissions/detail/572249337/
    /// https://leetcode.com/problems/ugly-number-ii/discuss/69364/My-16ms-C%2B%2B-DP-solution-with-short-explanation
    /// nums[1] = min(nums[0]*2, nums[0]*3, nums[0]*5)
    /// nums[2] = min(nums[1]*2, nums[0]*3, nums[0]*5)
    /// And so on. Be careful about the cases such as 6, in which we need to forward both pointers of 2 and 3.
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n == 1 {
            1
        } else {
            let n = n as usize;
            let mut nums = vec![1];
            let (mut i, mut j, mut k) = (0, 0, 0);
            while nums.len() < n {
                let (x, y, z) = (nums[i] * 2, nums[j] * 3, nums[k] * 5);
                let num = x.min(y).min(z);
                nums.push(num);
                if x == num {
                    i += 1;
                }
                if y == num {
                    j += 1;
                }
                if z == num {
                    k += 1;
                }
            }
            nums[n - 1]
        }
    }
    /// 22:06-22:08
    pub fn nth_ugly_number_dp_vec(n: i32) -> i32 {
        println!("nth_ugly_number({})", n);
        let n = n as usize;
        let mut dp: Vec<i32> = vec![1; n + 1];

        for i in 2..=n {
            let mut curr = i32::MAX;
            let prev = dp[i - 1];
            for &n in dp[..i].iter() {
                for &m in [2, 3, 5].iter() {
                    let n = n.saturating_mul(m);
                    if n > prev {
                        curr = curr.min(n);
                    }
                }
            }
            dp[i] = curr;
        }

        dp[n]
    }
    /// 21:58-22:06
    pub fn nth_ugly_number_dp_vec_iter(n: i32) -> i32 {
        println!("nth_ugly_number({})", n);
        let n = n as usize;
        let mut dp: Vec<i32> = vec![1; n + 1];

        for i in 2..=n {
            dp[i] = dp[..i]
                .iter()
                .flat_map(|&n| [2, 3, 5].iter().map(move |&m| n.saturating_mul(m)))
                .filter(|&n| n > dp[i - 1])
                .min()
                .unwrap();
        }

        dp[n]
    }
    /// 21:52-21:58
    pub fn nth_ugly_number_rec_with_memo(n: i32) -> i32 {
        println!("nth_ugly_number({})", n);
        fn rec(n: usize, memo: &mut Vec<i32>) -> i32 {
            if n == 1 {
                1
            } else if memo[n] > 0 {
                memo[n]
            } else {
                let prev = rec(n - 1, memo);
                memo[n] = (1..n)
                    .map(|n| rec(n, memo))
                    .flat_map(|n| [2, 3, 5].iter().map(move |&m| n.saturating_mul(m)))
                    .filter(|&n| n > prev)
                    .min()
                    .unwrap();
                memo[n]
            }
        }
        let n = n as usize;
        rec(n, &mut vec![0; n + 1])
    }
    /// 21:02-21:52
    pub fn nth_ugly_number_rec(n: i32) -> i32 {
        println!("nth_ugly_number({})", n);
        fn rec(n: i32) -> i32 {
            if n == 1 {
                1
            } else {
                let prev = rec(n - 1);
                (1..n)
                    .map(rec)
                    .flat_map(|n| [2, 3, 5].iter().map(move |&m| n.saturating_mul(m)))
                    .filter(|&n| n > prev)
                    .min()
                    .unwrap()
            }
        }
        rec(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1 2 3 22 5 23 222 33 25 223 35 2222 233 225 2223 55 333 235
    // 1 2 3  4 5  6   8  9 10  12 15   16  18  20   24 25  27  30

    /// Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
    #[rustfmt::skip] #[test] fn n_01() { assert_eq!(Solution::nth_ugly_number( 1),  1); } // 1   1 1 1
    #[rustfmt::skip] #[test] fn n_02() { assert_eq!(Solution::nth_ugly_number( 2),  2); } // 1*2 2 1 1
    #[rustfmt::skip] #[test] fn n_03() { assert_eq!(Solution::nth_ugly_number( 3),  3); } // 1*3 2 3 1
    #[rustfmt::skip] #[test] fn n_04() { assert_eq!(Solution::nth_ugly_number( 4),  4); } // 2*2 4 3 1
    #[rustfmt::skip] #[test] fn n_05() { assert_eq!(Solution::nth_ugly_number( 5),  5); } // 1*5 4 3 5
    #[rustfmt::skip] #[test] fn n_06() { assert_eq!(Solution::nth_ugly_number( 6),  6); } // 3*2 4 6 5
    #[rustfmt::skip] #[test] fn n_07() { assert_eq!(Solution::nth_ugly_number( 7),  8); } // 4*2 8 6 5
    #[rustfmt::skip] #[test] fn n_08() { assert_eq!(Solution::nth_ugly_number( 8),  9); } // 3*3
    #[rustfmt::skip] #[test] fn n_09() { assert_eq!(Solution::nth_ugly_number( 9), 10); } // 2*5
    /// Explanation: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.
    #[rustfmt::skip] #[test] fn n_10() { assert_eq!(Solution::nth_ugly_number(10), 12); } // 6*2
    #[rustfmt::skip] #[test] fn n_11() { assert_eq!(Solution::nth_ugly_number(11), 15); } // 3*5

    //#[ignore]
    #[rustfmt::skip] #[test] fn n_1690() { assert_eq!(Solution::nth_ugly_number(1690), 2_123_366_400); }
}
