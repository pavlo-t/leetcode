#![allow(dead_code)]

struct Solution;
struct Solution1;

impl Solution {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        a.iter().fold((&-1, 0, false, 0), |(a, size, seen_the_peak, rsf), b| {
            if a == &-1 || a == b || (size == 0 && a > b) {
                (b, 0, false, rsf)
            } else if !seen_the_peak && a < b {
                (b, size + 1, false, rsf)
            } else if a > b {
                (b, size + 1, true, rsf.max(size + 2))
            } else { // seen_the_peak && a < b
                (b, 1, false, rsf)
            }
        }).3
    }
}

impl Solution1 {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut start = 0;

        while start < a.len() {
            let mut end = start;
            if end + 1 < a.len() && a[end] < a[end + 1] {
                while end + 1 < a.len() && a[end] < a[end + 1] { end += 1; }
                if end + 1 < a.len() && a[end] > a[end + 1] {
                    while end + 1 < a.len() && a[end] > a[end + 1] { end += 1; }
                    ans = ans.max((end - start + 1) as i32)
                }
            }
            start = end.max(start + 1);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(Solution::longest_mountain(vec![]), 0)
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_mountain(vec![1]), 0)
    }

    #[test]
    fn test_1_1() {
        assert_eq!(Solution::longest_mountain(vec![1, 1]), 0)
    }

    #[test]
    fn test_1_2() {
        assert_eq!(Solution::longest_mountain(vec![1, 2]), 0)
    }

    #[test]
    fn test_2_1() {
        assert_eq!(Solution::longest_mountain(vec![2, 1]), 0)
    }

    #[test]
    fn test_2_2_1() {
        assert_eq!(Solution::longest_mountain(vec![2, 2, 1]), 0)
    }

    #[test]
    fn test_1_2_2() {
        assert_eq!(Solution::longest_mountain(vec![1, 2, 2]), 0)
    }

    #[test]
    fn test_1_2_1() {
        assert_eq!(Solution::longest_mountain(vec![1, 2, 1]), 3)
    }

    #[test]
    fn test_1_2_2_1() {
        assert_eq!(Solution::longest_mountain(vec![1, 2, 2, 1]), 0)
    }

    #[test]
    fn test_1_2_1_1() {
        assert_eq!(Solution::longest_mountain(vec![1, 2, 1, 1]), 3)
    }

    #[test]
    fn test_1_1_2_1() {
        assert_eq!(Solution::longest_mountain(vec![1, 1, 2, 1]), 3)
    }

    #[test]
    fn test_2_2_2() {
        assert_eq!(Solution::longest_mountain(vec![2, 2, 2]), 0)
    }

    #[test]
    fn test_2_1_4_7_3_2_5() {
        assert_eq!(Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5)
    }

    #[test]
    fn test_10000_ones() {
        assert_eq!(Solution::longest_mountain(vec![1; 10_000]), 0)
    }

    #[test]
    fn test_10000_long_mountain() {
        let mut a = Vec::new();
        for i in 1..10_000 { a.push(i); }
        a.push(1);

        assert_eq!(Solution::longest_mountain(a), 10_000)
    }

    #[test]
    fn test_10000_mountains() {
        let mut a = Vec::new();
        for i in 1..10_000 { a.push(i % 2); }
        a.push(1);

        assert_eq!(Solution::longest_mountain(a), 3)
    }
}