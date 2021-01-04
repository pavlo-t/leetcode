#![allow(dead_code)]

/// ### Reach a Number
///
/// https://leetcode.com/explore/featured/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3583/
struct Solution;

impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let target = target.abs();

        let mut sum = 0;
        let mut step = 0;

        while sum < target {
            step += 1;
            sum += step;
        }

        if (sum - target) % 2 == 0 {
            step
        } else if step % 2 == 0 {
            step + 1
        } else {
            step + 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_t3_is_2() {
        assert_eq!(Solution::reach_number(3), 2);
        // Explanation:
        // On the first move we step from 0 to 1.
        // On the second step we step from 1 to 3.
    }

    #[test]
    fn example2_t2_is_3() {
        assert_eq!(Solution::reach_number(2), 3);
        // Explanation:
        // On the first move we step from 0 to 1.
        // On the second move we step  from 1 to -1.
        // On the third move we step from -1 to 2.
    }

    #[test]
    fn t7_is_5() {
        assert_eq!(Solution::reach_number(7), 5);
    }

    #[test]
    fn t12_is_7() {
        assert_eq!(Solution::reach_number(12), 7);
    }

    #[test]
    fn tm12_is_7() {
        assert_eq!(Solution::reach_number(-12), 7);
    }

    #[test]
    fn t1_000_000_000_is_7() {
        assert_eq!(Solution::reach_number(1_000_000_000), 44723);
    }
}