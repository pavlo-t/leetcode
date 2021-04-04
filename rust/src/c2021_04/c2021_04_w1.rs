#![allow(dead_code)]
/// Largest Unique Number
/// =====================
///
/// Given an array of integers `A`, return the largest integer that only occurs once.
///
/// If no integer occurs once, return `-1`.
///
/// __Note:__
///
/// - `1 <= A.length <= 2000`
/// - `0 <= A[i] <= 1000`
///
/// https://leetcode.com/explore/challenge/card/april-leetcoding-challenge-2021/593/week-1-april-1st-april-7th/3692/
struct Solution;
impl Solution {
    pub fn largest_unique_number(a: Vec<i32>) -> i32 {
        let mut occurences = vec![0; 1001];
        for i in a {
            occurences[i as usize] += 1;
        }
        for (i, &o) in occurences.iter().enumerate().rev() {
            if o == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let a = vec![5, 7, 3, 9, 4, 9, 8, 3, 1];
        assert_eq!(Solution::largest_unique_number(a), 8);
        // Explanation:
        // The maximum integer in the array is 9 but it is repeated.
        // The number 8 occurs only once, so it's the answer.
    }
    #[test]
    fn example2() {
        let a = vec![9, 9, 8, 8];
        assert_eq!(Solution::largest_unique_number(a), -1);
        // Explanation:
        // There is no number that occurs only once.
    }
    //
}
