#![allow(dead_code)]
/// 1014. Best Sightseeing Pair
/// ===========================
///
/// You are given an integer array `values` where `values[i]` represents the value of the `i`th sightseeing spot.
/// Two sightseeing spots `i` and `j` have a __distance__ `j - i` between them.
///
/// The score of a pair (`i < j`) of sightseeing spots is `values[i] + values[j] + i - j`:
/// the sum of the values of the sightseeing spots, minus the distance between them.
///
/// Return _the maximum score of a pair of sightseeing spots_.
///
/// __Constraints:__
///
/// - `2 <= values.length <= 50_000`
/// - `1 <= values[i] <= 1000`
///
/// https://leetcode.com/problems/best-sightseeing-pair/
struct Solution;
impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max = values[0];
        for i in 1..values.len() {
            result = result.max(max + values[i] - i as i32);
            max = max.max(values[i] + i as i32);
        }
        result
    }
    pub fn max_score_sightseeing_pair_brute_force(values: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..values.len() - 1 {
            for j in i + 1..values.len() {
                result = result.max(values[i] + values[j] + i as i32 - j as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v_8_1_5_2_6() {
        let v = vec![8, 1, 5, 2, 6];
        assert_eq!(Solution::max_score_sightseeing_pair(v), 11);
        // Explanation: i = 0, j = 2, values[i] + values[j] + i - j = 8 + 5 + 0 - 2 = 11
    }
    #[test]
    fn v_1_2() {
        let v = vec![1, 2];
        assert_eq!(Solution::max_score_sightseeing_pair(v), 2);
    }
    #[test]
    fn v_5_1() {
        let v = vec![5, 1];
        assert_eq!(Solution::max_score_sightseeing_pair(v), 5);
    }

    #[test]
    fn v_1to50000() {
        let v = (1..=50_000).collect();
        assert_eq!(Solution::max_score_sightseeing_pair(v), 99998);
    }
    //#[ignore]
    #[test]
    fn v_50000x1() {
        let v = vec![1; 50000];
        assert_eq!(Solution::max_score_sightseeing_pair(v), 1);
    }
}
