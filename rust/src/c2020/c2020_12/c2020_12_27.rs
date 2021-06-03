#![allow(dead_code)]

/// ### Jump Game IV
///
/// https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3582/
struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::collections::VecDeque;

        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &n) in arr.iter().enumerate() {
            map.entry(n).or_insert(Vec::new()).push(i);
        }

        let mut seen: Vec<bool> = vec![false; arr.len()];
        seen[0] = true;

        let mut queue = VecDeque::new();
        queue.push_back(Some(0));
        queue.push_back(None);

        let mut steps = 0;

        while let Some(i_opt) = queue.pop_front() {
            if let Some(i) = i_opt {
                if i == arr.len() - 1 {
                    return steps;
                }
                if let Some(js) = map.insert(arr[i], Vec::new()) {
                    for j in js {
                        if !seen[j] {
                            queue.push_back(Some(j));
                            seen[j] = true;
                        }
                    }
                }
                if i > 0 && !seen[i - 1] {
                    queue.push_back(Some(i - 1));
                    seen[i - 1] = true;
                }
                if !seen[i + 1] {
                    queue.push_back(Some(i + 1));
                    seen[i + 1] = true;
                }
            } else {
                queue.push_back(None);
                steps += 1;
            }
        }

        panic!("Illegal state");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_a100_m23_m23_404_100_23_23_23_3_404_is_3() {
        let arr = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
        assert_eq!(Solution::min_jumps(arr), 3);
        // Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9. Note that index 9 is the last index of the array.
    }

    #[test]
    fn example2_a7_is_0() {
        assert_eq!(Solution::min_jumps(vec![7]), 0);
        // Explanation: Start index is the last index. You don't need to jump.
    }

    #[test]
    fn example3_a7_6_9_6_9_6_9_7_is_1() {
        let arr = vec![7, 6, 9, 6, 9, 6, 9, 7];
        assert_eq!(Solution::min_jumps(arr), 1)
        // Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
    }

    #[test]
    fn example4_a6_1_9_is_2() {
        let arr = vec![6, 1, 9];
        assert_eq!(Solution::min_jumps(arr), 2);
    }

    #[test]
    fn example5_a11_22_7_7_7_7_7_7_7_22_13_is_3() {
        let arr = vec![11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13];
        assert_eq!(Solution::min_jumps(arr), 3);
    }

    #[test]
    fn a1to50000_is_49999() {
        let arr = (1..=50000).collect();
        assert_eq!(Solution::min_jumps(arr), 49999);
    }

    #[test]
    fn a50000_1s_is_1() {
        let arr = vec![1; 50000];
        assert_eq!(Solution::min_jumps(arr), 1);
    }
}
