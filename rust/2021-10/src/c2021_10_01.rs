#![allow(dead_code)]
///
struct Solution;
impl Solution {
    fn test(i: i32) -> i32 {
        i + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_plus_1() {
        assert_eq!(Solution::test(1), 2);
    }
    #[test]
    fn test_2_plus_1() {
        assert_eq!(Solution::test(2), 2);
    }
}
