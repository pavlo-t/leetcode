#![allow(dead_code)]

/// ### Check Array Formation Through Concatenation
///
/// https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3589/
struct Solution;

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        use std::collections::HashMap;
        let map: HashMap<_, _> = pieces.into_iter().map(|p| (p[0], p)).collect();
        let mut i = 0;

        while let Some(p) = map.get(&arr[i]) {
            for &pv in p {
                if arr[i] != pv {
                    return false;
                }
                i += 1;
            }
            if i == arr.len() {
                return true;
            }
        }

        false
    }

    pub fn can_form_array_map_head_tail(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        use std::collections::HashMap;

        let map: HashMap<_, _> = pieces.iter().map(|p| (p[0], &p[1..])).collect();
        let mut i = 0;

        while let Some(&p) = map.get(&arr[i]) {
            for &pv in p {
                i += 1;
                if arr[i] != pv {
                    return false;
                }
            }
            i += 1;
            if i == arr.len() {
                return true;
            }
        }

        false
    }

    pub fn can_form_array_find_piece(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut i = 0;

        while let Some(p) = pieces.iter().find(|p| p[0] == arr[i]) {
            for &pv in p {
                if arr[i] != pv {
                    return false;
                }
                i += 1;
            }
            if i == arr.len() {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_a85_p85_is_true() {
        let arr = vec![85];
        let pieces = vec![vec![85]];
        assert!(Solution::can_form_array(arr, pieces))
    }

    #[test]
    fn example2_a15_88_pa88_b15_is_true() {
        let arr = vec![15, 88];
        let pieces = vec![vec![88], vec![15]];
        assert!(Solution::can_form_array(arr, pieces))
        // Explanation: Concatenate [15] then [88]
    }

    #[test]
    fn example3_a49_18_16_pa16_18_49_is_false() {
        let arr = vec![49, 18, 16];
        let pieces = vec![vec![16, 18, 49]];
        assert!(!Solution::can_form_array(arr, pieces))
        // Explanation: Even though the numbers match, we cannot reorder pieces[0].
    }

    #[test]
    fn example4_a91_4_64_78_pa78_b4_64_c91_is_true() {
        let arr = vec![91, 4, 64, 78];
        let pieces = vec![vec![78], vec![4, 64], vec![91]];
        assert!(Solution::can_form_array(arr, pieces))
        // Explanation: Concatenate [91] then [4,64] then [78]
    }

    #[test]
    fn example5_a1_3_5_7_pa2_4_5_8_is_false() {
        let arr = vec![1, 3, 5, 7];
        let pieces = vec![vec![2, 4, 6, 8]];
        assert!(!Solution::can_form_array(arr, pieces))
    }

    #[test]
    fn a1to10000_p10000to1map_vec_is_true() {
        let arr = (1..=10000).collect();
        let pieces = (1..=10000).rev().map(|i| vec![i]).collect();
        assert!(Solution::can_form_array(arr, pieces))
    }

    #[test]
    fn a1to10000_p10000to10by10map_vec_is_true() {
        let arr = (1..=10000).collect();
        let pieces = (10..=10000).rev().step_by(10)
            .map(|i| (i - 9..=i).collect()).collect();
        assert!(Solution::can_form_array(arr, pieces))
    }
}
