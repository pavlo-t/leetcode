#![allow(dead_code)]
/// 374. Guess Number Higher or Lower
/// =================================
///
/// We are playing the Guess Game. The game is as follows:
///
/// I pick a number from `1` to `n`. You have to guess which number I picked.
///
/// Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.
///
/// You call a pre-defined API `int guess(int num)`, which returns 3 possible results:
///
/// - `-1`: The number I picked is lower than your guess (i.e. `pick < num`).
/// - `1`: The number I picked is higher than your guess (i.e. `pick > num`).
/// - `0`: The number I picked is equal to your guess (i.e. `pick == num`).
///
/// Return _the number that I picked_.
///
/// __Constraints:__
///
/// - `1 <= n <= 2^31 - 1`
/// - `1 <= pick <= n`
///
/// https://leetcode.com/problems/guess-number-higher-or-lower/
struct Solution;
impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        println!("guessNumber({})", n);
        let (mut l, mut r) = (1, n);
        while l < r {
            let m = l + (r - l) / 2;
            match guess(m) {
                -1 => r = m - 1,
                1 => l = m + 1,
                _ => return m,
            }
        }
        l
    }
}

/// Forward declaration of guess API.
/// @param  num   your guess
/// @return       -1 if num is lower than the guess number
///                1 if num is higher than the guess number
///               otherwise return 0
unsafe fn guess(num: i32) -> i32 {
    if num > 6 {
        -1
    } else if num < 6 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_2pow31m1_p_6() {
        unsafe {
            assert_eq!(Solution::guessNumber(2_147_483_647), 6);
        }
    }
}
