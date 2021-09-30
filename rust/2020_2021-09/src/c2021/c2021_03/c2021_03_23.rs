#![allow(dead_code)]
/// # 3Sum With Multiplicity
///
/// Given an integer array `arr`, and an integer `target`, return the number of tuples `i, j, k`
/// such that `i < j < k` and `arr[i] + arr[j] + arr[k] == target`.
///
/// As the answer can be very large, return it __modulo__ `10^9 + 7`.
///
/// __Constraints:__
///
/// - `3 <= arr.length <= 3000`
/// - `0 <= arr[i] <= 100`
/// - `0 <= target <= 300`
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3682/
struct Solution;
//noinspection DuplicatedCode
impl Solution {
    pub fn three_sum_multi_very_cool(arr: Vec<i32>, target: i32) -> i32 {
        const M: i32 = 1_000_000_007;
        let mut dp = vec![0; 300];
        let mut result = 0;
        for i in 0..arr.len() {
            if target - arr[i] >= 0 {
                result += dp[target as usize - arr[i] as usize];
            }
            result %= M;
            for j in 0..i {
                dp[(arr[i] + arr[j]) as usize] += 1;
            }
        }
        result % M
    }

    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        use std::collections::HashMap;
        const M: i64 = 1_000_000_007;

        let counts = arr
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<i32, i64>, &i| {
                *acc.entry(i).or_default() += 1;
                acc
            });
        let mut keys = counts.keys().map(|&k| k).collect::<Vec<_>>();
        keys.sort_unstable();

        let mut result = 0;

        for i in 0..keys.len() {
            if keys[i] * 3 == target {
                let c = counts[&keys[i]];
                result += c * (c - 1) * (c - 2) / 6;
                break;
            }
            for j in i + 1..keys.len() {
                if keys[i] * 2 + keys[j] == target {
                    let c = counts[&keys[i]];
                    result += c * (c - 1) * counts[&keys[j]] / 2;
                }
                if keys[i] + keys[j] * 2 == target {
                    let c = counts[&keys[j]];
                    result += counts[&keys[i]] * c * (c - 1) / 2;
                }
                if keys[i] + keys[j] >= target {
                    break;
                }
                for k in j + 1..keys.len() {
                    let s = keys[i] + keys[j] + keys[k];
                    if s > target {
                        break;
                    } else if s == target {
                        result += counts[&keys[i]] * counts[&keys[j]] * counts[&keys[k]];
                    }
                }
            }
        }

        (result % M) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_arr1122334455t8_produces_20() {
        let arr = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        let target = 8;
        assert_eq!(Solution::three_sum_multi(arr, target), 20);
        // Explanation:
        // Enumerating by the values (arr[i], arr[j], arr[k]):
        // (1, 2, 5) occurs 8 times;
        // (1, 3, 4) occurs 8 times;
        // (2, 2, 4) occurs 2 times;
        // (2, 3, 3) occurs 2 times.
    }
    #[test]
    fn example2_arr112222t5_produces_12() {
        let arr = vec![1, 1, 2, 2, 2, 2];
        let target = 5;
        assert_eq!(Solution::three_sum_multi(arr, target), 12);
        // Explanation:
        // arr[i] = 1, arr[j] = arr[k] = 2 occurs 12 times:
        // We choose one 1 from [1,1] in 2 ways,
        // and two 2s from [2,2,2,2] in 6 ways.
    }

    #[test]
    fn test63_arr020t2_produces_1() {
        let arr = vec![0, 2, 0];
        let target = 2;
        assert_eq!(Solution::three_sum_multi(arr, target), 1);
    }
    #[test]
    fn test69_arr020t2_produces_a_lot() {
        let arr = vec![0; 3000];
        let target = 0;
        assert_eq!(Solution::three_sum_multi(arr, target), 495_500_972);
    }

    #[test]
    fn arr123t6_produces_1() {
        let arr = vec![1, 2, 3];
        let target = 6;
        assert_eq!(Solution::three_sum_multi(arr, target), 1);
    }
    #[test]
    fn arr222t6_produces_1() {
        let arr = vec![2, 2, 2];
        let target = 6;
        assert_eq!(Solution::three_sum_multi(arr, target), 1);
    }
    #[test]
    fn arr112t4_produces_1() {
        let arr = vec![1, 1, 2];
        let target = 4;
        assert_eq!(Solution::three_sum_multi(arr, target), 1);
    }
    #[test]
    fn arr122t5_produces_1() {
        let arr = vec![1, 2, 2];
        let target = 5;
        assert_eq!(Solution::three_sum_multi(arr, target), 1);
    }
    #[test]
    fn arr221t5_produces_1() {
        let arr = vec![2, 2, 1];
        let target = 5;
        assert_eq!(Solution::three_sum_multi(arr, target), 1);
    }
    #[test]
    fn arr2222t6_produces_4() {
        let arr = vec![2, 2, 2, 2];
        let target = 6;
        assert_eq!(Solution::three_sum_multi(arr, target), 4);
    }
}
