#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut counts: HashMap<i32, u32> = HashMap::new();
        for &num in nums.iter() {
            let count = counts.get(&num).unwrap_or(&0).to_owned();
            counts.insert(num, count + 1);
        }

        Solution::backtrack(&mut Vec::new(), nums.len(), &mut counts)
    }

    fn backtrack(comb: &mut Vec<i32>, size: usize, counts: &mut HashMap<i32, u32>) -> Vec<Vec<i32>> {
        if comb.len() == size {
            vec![comb.clone()]
        } else {
            let mut results = Vec::new();
            // for (&num, &count) in counts.clone().iter() {
            for (&num, &count) in counts.iter() {
                let mut counts_clone = counts.clone();
                if count > 0 {
                    comb.push(num);
                    counts_clone.insert(num, count - 1);
                    let mut result = Solution::backtrack(comb, size, &mut counts_clone);
                    results.append(&mut result);
                    comb.pop();
                    counts_clone.insert(num, count + 1);
                }
            }
            results
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1_2() {
        let expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        let mut result = Solution::permute_unique(vec![1, 1, 2]);
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_1_2_3() {
        let expected =
            vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]];
        let mut result = Solution::permute_unique(vec![1, 2, 3]);
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_1() {
        let nums = vec![1];
        let expected = vec![vec![1]];
        assert_eq!(Solution::permute_unique(nums), expected);
    }

    #[test]
    fn test_3_3_0_3() {
        let expected = vec![vec![0, 3, 3, 3], vec![3, 0, 3, 3], vec![3, 3, 0, 3], vec![3, 3, 3, 0]];
        let mut result = Solution::permute_unique(vec![3, 3, 0, 3]);
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_1_1_1_1_1_1_1_1() {
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let expected = vec![vec![1, 1, 1, 1, 1, 1, 1, 1]];
        assert_eq!(Solution::permute_unique(nums), expected);
    }

    #[test]
    fn test_1_2_3_4_5_6_7_8() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(Solution::permute_unique(nums).len(), 40320);
    }
}
