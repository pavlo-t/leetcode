#![allow(dead_code)]
/// Intersection of Two Arrays II
/// =============================
///
/// Given two integer arrays `nums1` and `nums2`, return _an array of their intersection_.
/// Each element in the result must appear as many times as it shows in both arrays
/// and you may return the result in __any order__.
///
/// __Constraints:__
///
/// - `1 <= nums1.length, nums2.length <= 1000`
/// - `0 <= nums1[i], nums2[i] <= 1000`
///
/// __Follow up:__
///
/// - What if the given array is already sorted? How would you optimize your algorithm?
/// - What if `nums1`'s size is small compared to `nums2`'s size? Which algorithm is better?
/// - What if elements of `nums2` are stored on disk, and the memory is limited
///   such that you cannot load all elements into the memory at once?
///
/// https://leetcode.com/explore/challenge/card/september-leetcoding-challenge-2021/638/week-3-september-15th-september-21st/3978/
struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut m1 = nums1.into_iter().fold(HashMap::new(), |mut rsf, i| {
            *rsf.entry(i).or_insert(0) += 1;
            rsf
        });
        let mut result = vec![];
        for i in nums2 {
            if let Some(c) = m1.get_mut(&i) {
                result.push(i);
                if c > &mut 1 {
                    *c -= 1;
                } else {
                    m1.remove(&i);
                }
            }
        }
        result
    }
    pub fn intersect_1(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        fn into_counts(ns: Vec<i32>) -> HashMap<i32, i32> {
            ns.into_iter().fold(HashMap::new(), |mut rsf, i| {
                *rsf.entry(i).or_insert(0) += 1;
                rsf
            })
        }
        let m1 = into_counts(nums1);
        let m2 = into_counts(nums2);
        let mut result = vec![];
        for (&i, &c1) in m1.iter() {
            if let Some(&c2) = m2.get(&i) {
                for _ in 0..c1.min(c2) {
                    result.push(i);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1_2_2_1_n_2_2_produces_2_2() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let e = vec![2, 2];
        let mut r = Solution::intersect(nums1, nums2);
        r.sort_unstable();
        assert_eq!(r, e);
    }
    #[test]
    fn n_4_9_5_n_9_4_9_8_4_produces_4_9() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let e = vec![4, 9];
        let mut r = Solution::intersect(nums1, nums2);
        r.sort_unstable();
        assert_eq!(r, e);
        // Explanation: [9,4] is also accepted.
    }
}
