#![allow(dead_code)]
//! \#315. Count of Smaller Numbers After Self
//! ==========================================
//!
//! <https://leetcode.com/problems/count-of-smaller-numbers-after-self>
//!
//! You are given an integer array `nums` and you have to return a new `counts` array.
//! The `counts` array has the property where `counts[i]` is the number of smaller elements to the right of `nums[i]`.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_07::c2022_07_23::*;
//! assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), [2, 1, 1, 0]);
//! ```
//!
//! __Explanation:__
//!
//! - To the right of `5` there are `2` smaller elements (`2` and `1`).
//! - To the right of `2` there is only `1` smaller element (`1`).
//! - To the right of `6` there is `1` smaller element (`1`).
//! - To the right of `1` there is `0` smaller element.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_07::c2022_07_23::*;
//! assert_eq!(Solution::count_smaller(vec![-1]), [0]);
//! ```
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_07::c2022_07_23::*;
//! assert_eq!(Solution::count_smaller(vec![-1, -1]), [0, 0]);
//! ```
//!
//! ##### Constraints
//!
//! - `1 <= nums.length <= 100_000`
//! - `-10_000 <= nums[i] <= 10_000`

pub struct Solution;
impl Solution {
    /// DP vec[vec[count; NUM_RANGE]]
    pub fn count_smaller_v1(nums: Vec<i32>) -> Vec<i32> {
        const MIN_NUM: i32 = -10_000;
        const MAX_NUM: i32 = 10_000;
        const MAX_NUM_IDX: usize = (MAX_NUM - MIN_NUM) as usize;

        let n = nums.len();
        let mut dp = vec![vec![0; MAX_NUM_IDX + 1]; n];
        let mut result = vec![0; n];

        for i in (0..n - 1).rev() {
            for num in 0..=MAX_NUM_IDX {
                let num_i32 = (num as i32) + MIN_NUM;
                dp[i][num] = dp[i + 1][num] + (nums[i + 1] < num_i32) as i32;
            }
            let num_idx = (nums[i] - MIN_NUM) as usize;
            result[i] = dp[i][num_idx];
        }

        result
    }

    /// DP vec[count; NUM_RANGE]
    pub fn count_smaller_v2(nums: Vec<i32>) -> Vec<i32> {
        let min_max_fn = |(max, min): (i32, i32), &num| (max.max(num), min.min(num));
        let (max_num, min_num) = nums.iter().fold((i32::MIN, i32::MAX), min_max_fn);
        let max_num_idx = (max_num - min_num) as usize;
        let as_idx = |num: i32| (num - min_num) as usize;

        let n = nums.len();
        let mut dp = vec![0; max_num_idx + 1];
        let mut result = vec![0; n];

        for i in (0..n - 1).rev() {
            for num in (as_idx(nums[i + 1] + 1)..=max_num_idx).rev() {
                dp[num] += 1;
            }
            result[i] = dp[as_idx(nums[i])];
        }

        result
    }

    /// DP HashMap<NUM, count>
    pub fn count_smaller_v3(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut dp: HashMap<i32, i32> = HashMap::new();
        for &num in &nums {
            dp.insert(num, 0);
        }

        let n = nums.len();
        let mut result = vec![0; n];

        for i in (0..n - 1).rev() {
            for (&num, count) in dp.iter_mut() {
                *count += (nums[i + 1] < num) as i32;
            }
            result[i] = *dp.get(&nums[i]).unwrap();
        }

        result
    }

    /// Reuse results for nums
    pub fn count_smaller_v4(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let n = nums.len();

        let mut next_same_num: HashMap<i32, usize> = HashMap::new();
        let mut result = vec![0; n];

        for (i, &num) in nums.iter().enumerate().rev() {
            let mut count = 0;
            if let Some(&next_idx) = next_same_num.get(&num) {
                for j in i + 1..next_idx {
                    count += (num > nums[j]) as i32;
                }
                result[i] = count + result[next_idx];
            } else {
                for j in i + 1..n {
                    count += (num > nums[j]) as i32;
                }
                result[i] = count;
            }
            next_same_num.insert(num, i);
        }

        result
    }

    /// Using a [segment tree (recursive)](https://www.geeksforgeeks.org/segment-tree-set-1-sum-of-given-range)
    pub fn count_smaller_v5(nums: Vec<i32>) -> Vec<i32> {
        type Range = (usize, usize);

        pub struct SegmentTree {
            tree: Vec<i32>,
            n: usize,
        }

        impl SegmentTree {
            pub fn new(n: usize) -> Self {
                let height = (n as f64).log2().ceil() as u32;
                let max_size = 2 * 2usize.pow(height);
                let tree = vec![0; max_size];

                Self { tree, n }
            }

            pub fn range_sum(&self, left: usize, right: usize) -> i32 {
                self.range_sum_rec(0, (0, self.n - 1), (left, right))
            }

            fn range_sum_rec(&self, node: usize, (l, r): Range, query: Range) -> i32 {
                let (left, right) = query;
                if r < left || right < l {
                    0
                } else if left <= l && r <= right {
                    self.tree[node]
                } else {
                    let m = l + (r - l) / 2;
                    let l_sum = self.range_sum_rec(node * 2 + 1, (l, m), query);
                    let r_sum = self.range_sum_rec(node * 2 + 2, (m + 1, r), query);
                    l_sum + r_sum
                }
            }

            pub fn add(&mut self, idx: usize, diff: i32) {
                self.add_rec(0, (0, self.n - 1), idx, diff);
            }

            fn add_rec(&mut self, node: usize, (l, r): Range, idx: usize, diff: i32) {
                if l <= idx && r >= idx {
                    self.tree[node] += diff;
                    if l < r {
                        let m = l + (r - l) / 2;
                        self.add_rec(node * 2 + 1, (l, m), idx, diff);
                        self.add_rec(node * 2 + 2, (m + 1, r), idx, diff);
                    }
                }
            }
        }

        let min_max = |(min, max): (i32, i32), &val| (min.min(val), max.max(val));
        let n = nums.len();
        let (min_val, max_val) = nums.iter().fold((i32::MAX, i32::MIN), min_max);
        let to_idx = |val: i32| (val - min_val) as usize;

        let mut tree = SegmentTree::new((max_val - min_val) as usize + 1);
        let mut result = vec![0; n];

        for (i, &val) in nums.iter().enumerate().rev() {
            if val > min_val {
                result[i] = tree.range_sum(0, to_idx(val) - 1)
            }
            tree.add(to_idx(val), 1);
        }

        result
    }

    /// Using a [segment tree (iterative)](https://www.geeksforgeeks.org/iterative-segment-tree-range-minimum-query)
    pub fn count_smaller_v6(nums: Vec<i32>) -> Vec<i32> {
        pub struct SegmentTree {
            tree: Vec<i32>,
            n: usize,
        }

        impl SegmentTree {
            pub fn new(n: usize) -> Self {
                let tree = vec![0; n * 2];
                Self { tree, n }
            }

            pub fn range_sum(&self, mut left: usize, mut right: usize) -> i32 {
                let mut result = 0;

                left += self.n;
                right += self.n + 1;

                while left < right {
                    if left % 2 == 1 {
                        result += self.tree[left];
                        left += 1;
                    }
                    if right % 2 == 1 {
                        right -= 1;
                        result += self.tree[right];
                    }
                    left /= 2;
                    right /= 2;
                }

                result
            }

            pub fn add(&mut self, mut i: usize, diff: i32) {
                i += self.n;
                self.tree[i] += diff;
                while i > 1 {
                    i /= 2;
                    self.tree[i] = self.tree[i * 2] + self.tree[i * 2 + 1];
                }
            }
        }

        let min_max = |(min, max): (i32, i32), &val| (min.min(val), max.max(val));
        let n = nums.len();
        let (min_val, max_val) = nums.iter().fold((i32::MAX, i32::MIN), min_max);
        let to_idx = |val: i32| (val - min_val) as usize;

        let mut tree = SegmentTree::new((max_val - min_val) as usize + 1);
        let mut result = vec![0; n];

        for (i, &val) in nums.iter().enumerate().rev() {
            if val > min_val {
                result[i] = tree.range_sum(0, to_idx(val) - 1)
            }
            tree.add(to_idx(val), 1);
        }

        result
    }

    /// Using a [binary indexed tree](https://www.geeksforgeeks.org/binary-indexed-tree-or-fenwick-tree-2)
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        pub struct BinaryIndexedTree {
            tree: Vec<i32>,
        }
        impl BinaryIndexedTree {
            pub fn new(n: usize) -> Self {
                let tree = vec![0; n + 1];
                Self { tree }
            }

            pub fn add(&mut self, mut i: usize, val: i32) {
                i += 1;
                while i < self.tree.len() {
                    self.tree[i] += val;
                    i += 1 << i.trailing_zeros();
                }
            }

            pub fn prefix_sum(&self, mut i: usize) -> i32 {
                let mut result = 0;
                i += 1;
                while i > 0 {
                    result += self.tree[i];
                    i -= 1 << i.trailing_zeros();
                }
                result
            }
        }

        let min_max = |(min, max): (i32, i32), &val| (min.min(val), max.max(val));
        let n = nums.len();
        let (min_val, max_val) = nums.iter().fold((i32::MAX, i32::MIN), min_max);
        let to_idx = |val: i32| (val - min_val) as usize;

        let mut tree = BinaryIndexedTree::new((max_val - min_val) as usize + 1);
        let mut result = vec![0; n];

        for (i, &val) in nums.iter().enumerate().rev() {
            if val > min_val {
                result[i] = tree.prefix_sum(to_idx(val) - 1);
            }
            tree.add(to_idx(val), 1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5p2p6p1() {
        let n = vec![5, 2, 6, 1];
        let e = vec![2, 1, 1, 0];
        assert_eq!(Solution::count_smaller(n), e);
        // Explanation:
        // To the right of 5 there are 2 smaller elements (2 and 1).
        // To the right of 2 there is only 1 smaller element (1).
        // To the right of 6 there is 1 smaller element (1).
        // To the right of 1 there is 0 smaller element.
    }
    #[test]
    fn m1() {
        let n = vec![-1];
        let e = vec![0];
        assert_eq!(Solution::count_smaller(n), e);
    }
    #[test]
    fn m1m1() {
        let n = vec![-1, -1];
        let e = vec![0, 0];
        assert_eq!(Solution::count_smaller(n), e);
    }
    #[test]
    fn p1p5p2p6p1() {
        let n = vec![1, 5, 2, 6, 1];
        let e = vec![0, 2, 1, 1, 0];
        assert_eq!(Solution::count_smaller(n), e);
    }

    //#[ignore]
    #[test]
    fn m10000_to_p10000_cycle_take_100000() {
        let n = (-10000..=10000).cycle().take(100_000).collect();
        assert_eq!(Solution::count_smaller(n).len(), 100_000);
    }
    //#[ignore]
    #[test]
    fn p10000_to_m10000_cycle_take_100000() {
        let n = (-10000..=10000).rev().cycle().take(100_000).collect();
        assert_eq!(Solution::count_smaller(n).len(), 100_000);
    }

    //#[ignore]
    #[test]
    fn test52() {
        let (nums, expected) = read_data("src/c2022_07_23_test_52.txt");
        assert_eq!(Solution::count_smaller(nums), expected);
    }
    //#[ignore]
    #[test]
    fn test58() {
        let (nums, expected) = read_data("src/c2022_07_23_test_58.txt");
        assert_eq!(Solution::count_smaller(nums), expected);
    }

    fn read_data(file: &str) -> (Vec<i32>, Vec<i32>) {
        let contents = std::fs::read_to_string(file).expect("NO FILE");
        let mut is_neg = false;
        let mut num = 0;
        let mut chars = contents.chars();

        let mut nums = vec![];
        while let Some(c) = chars.next() {
            match c {
                '[' | ' ' | '\n' => (),
                ',' | ']' => {
                    nums.push(num);
                    num = 0;
                    if c == ']' {
                        break;
                    }
                }
                '-' => is_neg = true,
                '0'..='9' => {
                    num *= 10;
                    num += (c as u8 - b'0') as i32;
                    if is_neg {
                        num = -num;
                        is_neg = false;
                    }
                }
                c => panic!("Unexpected char: {c}"),
            }
        }

        let mut expected_result = vec![];
        while let Some(c) = chars.next() {
            match c {
                '[' | ' ' | '\n' => (),
                ',' | ']' => {
                    expected_result.push(num);
                    num = 0;
                }
                '0'..='9' => {
                    num *= 10;
                    num += (c as u8 - b'0') as i32;
                }
                c => panic!("Unexpected char: {c}"),
            }
        }

        (nums, expected_result)
    }
}
