#![allow(dead_code)]
struct Solution {}
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let base = minutes_to_test / minutes_to_die + 1;
        (buckets as f64).log(base as f64).ceil() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1000_15_60() {
        assert_eq!(Solution::poor_pigs(1000, 15, 60), 5);
    }

    #[test]
    fn test_4_15_15() {
        assert_eq!(Solution::poor_pigs(4, 15, 15), 2);
    }

    #[test]
    fn test_1_1_1() {
        assert_eq!(Solution::poor_pigs(1, 1, 1), 0);
    }
}
