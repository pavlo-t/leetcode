// @formatter:off
#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;

        let mut r = Vec::with_capacity(nums.len() - k as usize + 1);
        let mut w = VecDeque::with_capacity(k as usize);

        let pop_back_while_lower = |i: usize, w: &mut VecDeque<i32>| {
            while let Some(&j) = w.back() {
                if nums[j as usize] > nums[i] { break; }
                w.pop_back();
            }
        };

        for i in 0..k {
            pop_back_while_lower(i as usize, &mut w);
            w.push_back(i);
        }
        r.push(nums[*w.front().unwrap() as usize]);

        for i in k..nums.len() as i32 {
            if let Some(&j) = w.front() {
                if j <= i - k { w.pop_front(); }
            }
            pop_back_while_lower(i as usize, &mut w);
            w.push_back(i);
            r.push(nums[*w.front().unwrap() as usize]);
        }

        r
    }
}

struct SolutionBruteForce;
impl SolutionBruteForce {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            nums
        } else {
            let mut result = Vec::new();
            for l in 0..=(nums.len() - k as usize) {
                result.push((l..(l + k as usize)).map(|i| nums[i]).max().unwrap());
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let expected = vec![3, 3, 5, 5, 6, 7];
        assert_eq!(Solution::max_sliding_window(nums, 3), expected);
        // Explanation:
        // Window position                Max
        // ---------------               -----
        // [1  3  -1] -3  5  3  6  7       3
        //  1 [3  -1  -3] 5  3  6  7       3
        //  1  3 [-1  -3  5] 3  6  7       5
        //  1  3  -1 [-3  5  3] 6  7       5
        //  1  3  -1  -3 [5  3  6] 7       6
        //  1  3  -1  -3  5 [3  6  7]      7
    }
    #[test]
    fn example2() { assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]); }
    #[test]
    fn example3() { assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]); }
    #[test]
    fn example4() { assert_eq!(Solution::max_sliding_window(vec![9, 11], 2), vec![11]); }
    #[test]
    fn example5() { assert_eq!(Solution::max_sliding_window(vec![4, -2], 2), vec![4]); }
    #[test]
    fn test_1to100000_k2() {
        let nums = (1..=100000).collect();
        let expected: Vec<_> = (2..=100000).collect();
        assert_eq!(Solution::max_sliding_window(nums, 2), expected);
    }
    #[test]
    fn test_1to100000_k50000() {
        let nums = (1..=100000).collect();
        let expected: Vec<_> = (50000..=100000).collect();
        assert_eq!(Solution::max_sliding_window(nums, 50000), expected);
    }
    #[test]
    fn test_100000to1_k50000() {
        let nums = (1..=100000).rev().collect();
        let expected: Vec<_> = (50000..=100000).rev().collect();
        assert_eq!(Solution::max_sliding_window(nums, 50000), expected);
    }
}