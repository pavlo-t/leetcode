#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut remainders = vec![0; 60];

        for &t in time.iter() {
            let r = (t % 60) as usize;
            if r == 0 {
                result += remainders[r]
            } else {
                result += remainders[60 - r]
            }
            remainders[r] += 1;
        }

        result
    }
}

struct SolutionBruteForce;

impl SolutionBruteForce {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in 0..(time.len() - 1) {
            for j in i + 1..time.len() {
                if (time[i] + time[j]) % 60 == 0 {
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let time = vec![30, 20, 150, 100, 40];
        assert_eq!(Solution::num_pairs_divisible_by60(time), 3);
        //Explanation: Three pairs have a total duration divisible by 60:
        // (time[0] = 30, time[2] = 150): total duration 180
        // (time[1] = 20, time[3] = 100): total duration 120
        // (time[1] = 20, time[4] = 40): total duration 60
    }

    #[test]
    fn example2() {
        let time = vec![60, 60, 60];
        assert_eq!(Solution::num_pairs_divisible_by60(time), 3);
        //Explanation: All three pairs have a total duration of 120, which is divisible by 60.
    }

    #[test]
    fn test_t10() {
        let time = vec![10];
        assert_eq!(Solution::num_pairs_divisible_by60(time), 0);
    }

    #[test]
    fn test_t60000_10s_is_0() {
        let time = vec![10; 60000];
        assert_eq!(Solution::num_pairs_divisible_by60(time), 0);
    }
    //

    #[test]
    fn test_t60000_60s_is_1_799_970_000() {
        let time = vec![60; 60000];
        assert_eq!(Solution::num_pairs_divisible_by60(time), 1_799_970_000);
    }

    #[test]
    fn test_t1to60000_is_29_999_000() {
        let time = (1..=60000).collect();
        assert_eq!(Solution::num_pairs_divisible_by60(time), 29_999_000);
    }
}
