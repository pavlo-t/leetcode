#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            return -1
        }
        let mut remainder = 0;
        for l in 1..=k {
            remainder = (remainder * 10 + 1) % k;
            if remainder == 0 { return l }
        }
        -1
    }
}
// @formatter:off

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_1_is_1() { assert_eq!(Solution::smallest_repunit_div_by_k(1), 1); }
    #[test] fn test_2_is_m1() { assert_eq!(Solution::smallest_repunit_div_by_k(2), -1); }
    #[test] fn test_3_is_3() { assert_eq!(Solution::smallest_repunit_div_by_k(3), 3); }
    #[test] fn test_4_is_m1() { assert_eq!(Solution::smallest_repunit_div_by_k(4), -1); }
    #[test] fn test_5_is_m1() { assert_eq!(Solution::smallest_repunit_div_by_k(5), -1); }
    #[test] fn test_6_is_m1() { assert_eq!(Solution::smallest_repunit_div_by_k(6), -1); }
    #[test] fn test_7_is_m1() { assert_eq!(Solution::smallest_repunit_div_by_k(7), 6); }

    #[test] fn test_99233_is_99232() { assert_eq!(Solution::smallest_repunit_div_by_k(99233), 99232); }
    #[test] fn test_95000to100000() {
        for i in 95000..=100000 { assert!(Solution::smallest_repunit_div_by_k(i) >= -1); }
    }
}
// @formatter:on
