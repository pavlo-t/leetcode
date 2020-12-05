#![allow(dead_code)]

/// https://leetcode.com/problems/the-kth-factor-of-n/solution/
struct Solution;

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut k = k;
        let sqrt_n = (n as f64).sqrt();
        let mut divisors = Vec::new();

        for x in 1..=(sqrt_n as i32) {
            if n % x == 0 {
                k -= 1;
                divisors.push(x);
                if k == 0 {
                    return x;
                }
            }
        }

        // If n is a perfect square we have to skip the duplicate in the divisor list
        if sqrt_n * sqrt_n == n as f64 {
            k += 1;
        }

        let n_div: i32 = divisors.len() as i32;
        if k <= n_div {
            n / divisors[(n_div - k) as usize]
        } else {
            -1
        }
    }
}

// @formatter:off
struct SolutionBruteForce;
impl SolutionBruteForce {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut k = k;
        let mut i = 1;

        while i <= n {
            if n % i == 0 {
                k -= 1;
                if k == 0 {
                    return i;
                }
            }
            i += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_n12_k3_is_3() {
        assert_eq!(Solution::kth_factor(12, 3), 3);
        //Explanation: Factors list is [1, 2, 3, 4, 6, 12], the 3rd factor is 3.
    }
    #[test]
    fn example2_n7_k2_is_7() {
        assert_eq!(Solution::kth_factor(7, 2), 7);
        //Explanation: Factors list is [1, 7], the 2nd factor is 7.
    }
    #[test]
    fn example3_n4_k_4_is_m1() {
        assert_eq!(Solution::kth_factor(4, 4), -1);
        //Explanation: Factors list is [1, 2, 4], there is only 3 factors. We should return -1.
    }
    #[test]
    fn example4_n1_k1_is_1() {
        assert_eq!(Solution::kth_factor(1, 1), 1);
        //Explanation: Factors list is [1], the 1st factor is 1.
    }
    #[test]
    fn example5_n1000_k3_is_4() {
        assert_eq!(Solution::kth_factor(1000, 3), 4);
        //Explanation: Factors list is [1, 2, 4, 5, 8, 10, 20, 25, 40, 50, 100, 125, 200, 250, 500, 1000].
    }
    #[test]
    fn n1000_k1000_is_m1() { assert_eq!(Solution::kth_factor(1000, 1000), -1); }
    #[test]
    fn n2_000_000_000_k1000_is_m1() { assert_eq!(Solution::kth_factor(2_000_000_000, 1000), -1); }
}