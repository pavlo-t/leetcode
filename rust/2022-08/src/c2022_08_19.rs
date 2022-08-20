#![allow(dead_code)]
//! \#659. Split Array into Consecutive Subsequences
//! ================================================
//!
//! <https://leetcode.com/problems/split-array-into-consecutive-subsequences>
//!
//! You are given an integer array `nums` that is __sorted in non-decreasing order__.
//!
//! Determine if it is possible to split `nums` into __one or more subsequences__
//! such that __both__ of the following conditions are true:
//!
//! - Each subsequence is a __consecutive increasing sequence__
//!   (i.e. each integer is __exactly one__ more than the previous integer).
//! - All subsequences have a length of `3` __or more__.
//!
//! Return _`true` if you can split `nums` according to the above conditions, or `false` otherwise_.
//!
//! A __subsequence__ of an array is a new array that is formed from the original array
//! by deleting some (can be none) of the elements
//! without disturbing the relative positions of the remaining elements.
//! (i.e., `[1,3,5]` is a subsequence of `[1,2,3,4,5]` while `[1,3,2]` is not).
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_19::*;
//! let nums = vec![1, 2, 3, 3, 4, 5];
//! assert_eq!(Solution::is_possible(nums), true);
//! ```
//!
//! __Explanation:__ nums can be split into the following subsequences: `[[1, 2, 3], [3, 4, 5]]`
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_19::*;
//! let nums = vec![1, 2, 3, 3, 4, 4, 5, 5];
//! assert_eq!(Solution::is_possible(nums), true);
//! ```
//!
//! __Explanation:__ nums can be split into the following subsequences: `[[1, 2, 3, 4, 5], [3, 4, 5]]`
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_08::c2022_08_19::*;
//! let nums = vec![1, 2, 3, 4, 4, 5];
//! assert_eq!(Solution::is_possible(nums), false);
//! ```
//!
//! __Explanation:__ It is impossible to split nums into consecutive increasing subsequences of length `3` or more.
//!
//! ##### Constraints
//!
//! - `1 <= nums.length <= 10_000`
//! - `-1000 <= nums[i] <= 1000`
//! - `nums` is sorted in __non-decreasing__ order.

pub struct Solution;
impl Solution {
    /// Build counts and iterate over them
    ///
    /// - Time: `O(n * m)`, where `m` is the number of possible values
    /// - Memory: `O(m)`
    pub fn is_possible_v1(nums: Vec<i32>) -> bool {
        const OFFSET: i32 = 1000;
        const SIZE: usize = 2001;

        let mut counts =
            nums.iter()
                .map(|&num| (num + OFFSET) as usize)
                .fold([0; SIZE], |mut counts, i| {
                    counts[i] += 1;
                    counts
                });

        while let Some(mut i) = counts.iter().position(|&count| count > 0) {
            counts[i] -= 1;
            let mut took = 1;
            i += 1;
            while i < SIZE && counts[i] > 0 && counts[i - 1] + 1 <= counts[i] {
                counts[i] -= 1;
                i += 1;
                took += 1;
            }
            if took < 3 {
                return false;
            }
        }

        true
    }

    /// Using a `VecDeque` to collect the subsequences
    ///
    /// - Time: `O(n)`
    /// - Memory: `O(n)`
    pub fn is_possible_v2(nums: Vec<i32>) -> bool {
        use std::collections::VecDeque;

        let mut subseqs: VecDeque<Vec<i32>> = VecDeque::new();
        let mut prev = i32::MIN;
        let mut prev_count = 0;

        for num in nums.into_iter().chain(std::iter::once(i32::MAX)) {
            if num == prev {
                prev_count += 1;
            } else {
                while subseqs.len() > prev_count {
                    if subseqs.pop_front().filter(|seq| seq.len() < 3).is_some() {
                        return false;
                    }
                }
                subseqs.iter_mut().for_each(|seq| seq.push(prev));
                while subseqs.len() < prev_count {
                    subseqs.push_back(vec![prev]);
                }
                if num != prev + 1 {
                    while let Some(seq) = subseqs.pop_front() {
                        if seq.len() < 3 {
                            return false;
                        }
                    }
                }

                prev = num;
                prev_count = 1;
            }
        }

        true
    }

    /// Using a `VecDeque` to keep track of subsequences' lengths
    ///
    /// - Time: `O(n)`
    /// - Memory: `O(n)`
    pub fn is_possible_v3(nums: Vec<i32>) -> bool {
        use std::collections::VecDeque;

        let mut lengths = VecDeque::new();
        let mut prev = i32::MIN;
        let mut prev_count = 0;

        for num in nums.into_iter().chain(std::iter::once(i32::MAX)) {
            if num == prev {
                prev_count += 1;
            } else {
                while lengths.len() > prev_count {
                    if lengths.pop_front().filter(|&len| len < 3).is_some() {
                        return false;
                    }
                }
                lengths.iter_mut().for_each(|len| *len += 1);
                while lengths.len() < prev_count {
                    lengths.push_back(1);
                }
                if num != prev + 1 {
                    while let Some(len) = lengths.pop_front() {
                        if len < 3 {
                            return false;
                        }
                    }
                }

                prev = num;
                prev_count = 1;
            }
        }

        true
    }

    /// Constant space
    ///
    /// - Time: `O(n)`
    /// - Memory: `O(1)`
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut seqs_of_len = [0u32; 4];

        let mut prev = i32::MIN;
        let mut prev_count = 0;

        for num in nums.into_iter().chain(std::iter::once(i32::MAX)) {
            if num == prev {
                prev_count += 1;
            } else {
                let all_seqs = seqs_of_len.iter().sum::<u32>();

                if let Some(finished_seqs) = all_seqs.checked_sub(prev_count) {
                    if finished_seqs > seqs_of_len[3] {
                        return false;
                    }
                    seqs_of_len[3] -= finished_seqs;
                }

                seqs_of_len[3] += seqs_of_len[2];
                seqs_of_len[2] = seqs_of_len[1];
                seqs_of_len[1] = prev_count.saturating_sub(all_seqs);

                if num != prev + 1 {
                    if seqs_of_len.iter().take(3).any(|&count| count != 0) {
                        return false;
                    }
                    seqs_of_len = [0; 4];
                }

                prev = num;
                prev_count = 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1()       { assert_eq!(Solution::is_possible(vec![1]),       false); }
    #[rustfmt::skip] #[test] fn n_1_2()     { assert_eq!(Solution::is_possible(vec![1,2]),     false); }
    #[rustfmt::skip] #[test] fn n_1_2_2()   { assert_eq!(Solution::is_possible(vec![1,2,2]),   false); }
    #[rustfmt::skip] #[test] fn n_1_2_3()   { assert_eq!(Solution::is_possible(vec![1,2,3]),   true); }
    #[rustfmt::skip] #[test] fn n_1_2_3_4() { assert_eq!(Solution::is_possible(vec![1,2,3,4]), true); }

    #[test]
    fn n_1_2_3_5_6_7() {
        let nums = vec![1, 2, 3, 5, 6, 7];
        assert_eq!(Solution::is_possible(nums), true);
    }

    #[test]
    fn n_1_2_3_3_4_5() {
        let nums = vec![1, 2, 3, 3, 4, 5];
        assert_eq!(Solution::is_possible(nums), true);
    }

    #[test]
    fn n_1_2_3_3_4_4_5_5() {
        let nums = vec![1, 2, 3, 3, 4, 4, 5, 5];
        assert_eq!(Solution::is_possible(nums), true);
    }

    /// ```text
    /// 1 2 3 4
    ///     3 4 5
    ///       4 5 6
    /// ```
    #[test]
    fn n_1_2_3_3_4_4_4_5_5_6() {
        let nums = vec![1, 2, 3, 3, 4, 4, 4, 5, 5, 6];
        assert_eq!(Solution::is_possible(nums), true);
    }

    /// ```text
    /// 1 2 3 4 5
    ///     3 4 5 6
    ///       4 5 6 7
    /// counts: {1:1, 2:1, 3:2, 4:3, 5:3, 6:2, 7:1}
    /// counts: {1:0, 2:0, 3:1, 4:2, 5:2, 6:2, 7:1}
    /// counts: {1:0, 2:0, 3:0, 4:1, 5:1, 6:1, 7:1}
    /// counts: {1:0, 2:0, 3:0, 4:0, 5:0, 6:0, 7:0}
    /// ```
    #[test]
    fn n_1_2_3_3_4_4_4_5_5_5_6_6_7() {
        let nums = vec![1, 2, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7];
        assert_eq!(Solution::is_possible(nums), true);
    }

    /// ```text
    /// 1 2 3 4
    ///       4 5
    /// counts: {1:1, 2:1, 3:1, 4:2, 5:1}
    /// counts: {1:0, 2:0, 3:0, 4:1, 5:1}
    /// counts: {1:0, 2:0, 3:0, 4:0, 5:0}
    /// ```
    #[test]
    fn n_1_2_3_4_4_5() {
        let nums = vec![1, 2, 3, 4, 4, 5];
        assert_eq!(Solution::is_possible(nums), false);
    }

    #[test]
    fn n_1_2_3_5_5_6_7() {
        let nums = vec![1, 2, 3, 5, 5, 6, 7];
        assert_eq!(Solution::is_possible(nums), false);
    }
}
