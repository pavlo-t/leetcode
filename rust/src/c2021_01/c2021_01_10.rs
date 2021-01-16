#![allow(dead_code)]

/// ### Create Sorted Array through Instructions
/// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3599/
struct Solution;

const M: i32 = 1_000_000_007;

//noinspection DuplicatedCode
impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        fn update(mut i: usize, st: &mut [i32], m: usize) {
            i += m;
            while i > 0 {
                st[i] += 1;
                i >>= 1;
            }
        }

        fn query(mut l: usize, mut r: usize, st: &[i32], m: usize) -> i32 {
            let mut result = 0;
            l += m;
            r += m;
            while l < r {
                if (l & 1) == 1 {
                    result += st[l];
                    l += 1;
                }
                if (r & 1) == 1 {
                    r -= 1;
                    result += st[r];
                }
                l >>= 1;
                r >>= 1;
            }
            result
        }

        let m = 100_001;
        let mut segment_tree = vec![0; m * 2];
        let mut cost = 0;

        for v in instructions {
            let v = v as usize;
            let l = query(0, v, &segment_tree, m);
            let r = query(v + 1, m, &segment_tree, m);
            cost = (cost + l.min(r)) % M;
            update(v, &mut segment_tree, m);
        }

        cost
    }

    pub fn create_sorted_array_fenwick_tree_aka_binary_indexed_tree(instructions: Vec<i32>) -> i32 {
        fn lsb(i: usize) -> usize {
            let i = i as i32;
            (i & -i) as usize
        }

        fn sum(mut i: usize, bit: &[i32]) -> i32 {
            let mut sum = 0;
            while i > 0 {
                sum += bit[i];
                i -= lsb(i);
            }
            sum
        }

        fn add(mut i: usize, bit: &mut [i32]) {
            while i < bit.len() {
                bit[i] += 1;
                i += lsb(i);
            }
        }

        let mut bit = vec![0; 100_001];
        let mut cost = 0;

        for (i, &v) in instructions.iter().enumerate() {
            let v = v as usize;
            cost = (cost + sum(v - 1, &bit).min((i as i32) - sum(v, &bit))) % M;
            add(v, &mut bit);
        }

        cost
    }

    pub fn create_sorted_array_brute_force(instructions: Vec<i32>) -> i32 {
        let mut result = 0;

        for (i, v) in instructions.iter().enumerate() {
            let (lt, gt) = instructions[..i].iter().fold((0, 0), |(mut lt, mut gt), w| {
                if w < v { lt += 1; } else if w > v { gt += 1; }
                (lt, gt)
            });
            result += lt.min(gt);
            result %= M;
        }

        result
    }
    pub fn create_sorted_array_brute_force_build_array(instructions: Vec<i32>) -> i32 {
        let mut arr = Vec::with_capacity(instructions.len());
        let mut result = 0;

        for i in instructions {
            let (lt, gt) = arr.iter().fold((0, 0), |(mut lt, mut gt), &v| {
                if v < i { lt += 1; } else if v > i { gt += 1; }
                (lt, gt)
            });
            arr.push(i);
            result += lt.min(gt);
            result %= M;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_i1562_is_1() {
        assert_eq!(Solution::create_sorted_array(vec![1, 5, 6, 2]), 1);
        // Explanation: Begin with nums = [].
        // Insert 1 with cost min(0, 0) = 0, now nums = [1].
        // Insert 5 with cost min(1, 0) = 0, now nums = [1,5].
        // Insert 6 with cost min(2, 0) = 0, now nums = [1,5,6].
        // Insert 2 with cost min(1, 2) = 1, now nums = [1,2,5,6].
        // The total cost is 0 + 0 + 0 + 1 = 1.
    }

    #[test]
    fn example2_i123654_is_3() {
        assert_eq!(Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]), 3);
        // Explanation: Begin with nums = [].
        // Insert 1 with cost min(0, 0) = 0, now nums = [1].
        // Insert 2 with cost min(1, 0) = 0, now nums = [1,2].
        // Insert 3 with cost min(2, 0) = 0, now nums = [1,2,3].
        // Insert 6 with cost min(3, 0) = 0, now nums = [1,2,3,6].
        // Insert 5 with cost min(3, 1) = 1, now nums = [1,2,3,5,6].
        // Insert 4 with cost min(3, 2) = 2, now nums = [1,2,3,4,5,6].
        // The total cost is 0 + 0 + 0 + 0 + 1 + 2 = 3.
    }

    #[test]
    fn example3_i133324212_is_4() {
        assert_eq!(Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2]), 4);
        // Explanation: Begin with nums = [].
        // Insert 1 with cost min(0, 0) = 0, now nums = [1].
        // Insert 3 with cost min(1, 0) = 0, now nums = [1,3].
        // Insert 3 with cost min(1, 0) = 0, now nums = [1,3,3].
        // Insert 3 with cost min(1, 0) = 0, now nums = [1,3,3,3].
        // Insert 2 with cost min(1, 3) = 1, now nums = [1,2,3,3,3].
        // Insert 4 with cost min(5, 0) = 0, now nums = [1,2,3,3,3,4].
        // Insert 2 with cost min(1, 4) = 1, now nums = [1,2,2,3,3,3,4].
        // Insert 1 with cost min(0, 6) = 0, now nums = [1,1,2,2,3,3,3,4].
        // Insert 2 with cost min(2, 4) = 2, now nums = [1,1,2,2,2,3,3,3,4].
        // The total cost is 0 + 0 + 0 + 0 + 1 + 0 + 1 + 0 + 2 = 4.
    }

    #[test]
    fn i1_is_0() {
        let instructions = vec![1];
        assert_eq!(Solution::create_sorted_array(instructions), 0);
    }

    #[test]
    fn i1to10000_is_0() {
        let instructions = (1..=10000).collect();
        assert_eq!(Solution::create_sorted_array(instructions), 0);
    }

    #[test]
    fn i1to100000_is_0() {
        let instructions = (1..=100000).collect();
        assert_eq!(Solution::create_sorted_array(instructions), 0);
    }
}
