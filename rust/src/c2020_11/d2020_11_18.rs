#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut is = intervals;
        is.sort_unstable();
        is.iter().fold(vec![], |mut acc, i| {
            match acc.last_mut() {
                Some(l) if l[1] >= i[0] => {
                    l[1] = l[1].max(i[1]);
                    acc
                }
                _ => {
                    acc.push(vec![i[0], i[1]]);
                    acc
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s1e3_s2e6_s8e10_s15e18() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(Solution::merge(intervals), expected);
        // Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
    }

    #[test]
    fn test_s1e4_s1e5() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let expected = vec![vec![1, 5]];
        assert_eq!(Solution::merge(intervals), expected);
        // Explanation: Intervals [1,4] and [4,5] are considered overlapping.
    }

    #[test]
    fn test_s1e4_s0e4() {
        let intervals = vec![vec![1, 4], vec![0, 4]];
        let expected = vec![vec![0, 4]];
        assert_eq!(Solution::merge(intervals), expected);
    }

    #[test]
    fn test_10000_s1e2_to_s10000e10001() {
        let mut intervals = Vec::new();
        for i in 1..=10000 {
            intervals.push(vec![i, i + 1]);
        }
        let expected = vec![vec![1, 10001]];
        assert_eq!(Solution::merge(intervals), expected);
    }

}
