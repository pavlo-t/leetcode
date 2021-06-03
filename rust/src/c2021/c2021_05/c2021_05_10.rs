#![allow(dead_code)]
/// Count Primes
/// ============
///
/// Count the number of prime numbers less than a non-negative number, `n`.
///
/// __Constraints:__
///
/// - `0 <= n <= 5_000_000`
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3738/
struct Solution;
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            0
        } else {
            let mut primes = vec![true; n as usize];
            primes[0] = false;
            primes[1] = false;

            (2..(n as f64).sqrt().ceil() as usize).for_each(|i| if primes[i] {
                (i * i..n as usize).step_by(i).for_each(|j| primes[j] = false)
            });

            primes.into_iter().fold(0, |acc, b| if b { acc + 1 } else { acc })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_n_10_produces_4() {
        assert_eq!(Solution::count_primes(10), 4);
        // Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
    }
    #[test]
    fn example2_n_0_produces_0() {
        assert_eq!(Solution::count_primes(0), 0);
    }
    #[test]
    fn example3_n_1_produces_0() {
        assert_eq!(Solution::count_primes(1), 0);
    }
    #[test]
    fn n_2_produces_0() {
        assert_eq!(Solution::count_primes(2), 0);
    }
    #[test]
    fn n_3_produces_1() {
        assert_eq!(Solution::count_primes(3), 1);
    }
    #[test]
    fn n_5_produces_2() {
        assert_eq!(Solution::count_primes(5), 2);
    }
    #[test]
    fn n_6_produces_3() {
        assert_eq!(Solution::count_primes(6), 3);
    }

    mod performance {
        use super::*;

        #[test]
        fn n_5_000_000_produces_348513() {
            assert_eq!(Solution::count_primes(5_000_000), 348_513);
        }
    }
}
