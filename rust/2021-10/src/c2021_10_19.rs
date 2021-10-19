#![allow(dead_code)]
/// 496. Next Greater Element I
/// ===========================
///
/// The __next greater element__ of some element `x` in an array
/// is the __first greater__ element that is __to the right__ of `x` in the same array.
///
/// You are given two __distinct 0-indexed__ integer arrays `nums1` and `nums2`,
/// where `nums1` is a subset of `nums2`.
///
/// For each `0 <= i < nums1.length`, find the index `j` such that `nums1[i] == nums2[j]`
/// and determine __the next greater element__ of `nums2[j]` in `nums2`.
/// If there is no next greater element, then the answer for this query is `-1`.
///
/// Return _an array `ans` of length `nums1.length`
/// such that `ans[i]` is the __next greater element__ as described above_.
///
/// __Constraints:__
///
/// - `1 <= nums1.length <= nums2.length <= 1000`
/// - `0 <= nums1[i], nums2[i] <= 10_000`
/// - All integers in `nums1` and `nums2` are __unique__.
/// - All the integers of `nums1` also appear in `nums2`.
///
/// __Follow up:__ Could you find an `O(nums1.length + nums2.length)` solution?
///
/// https://leetcode.com/problems/next-greater-element-i/
struct Solution;
impl Solution {
    /// Approach 3: Using Stack
    /// https://leetcode.com/problems/next-greater-element-i/solution/
    pub fn next_greater_element(q: Vec<i32>, ns: Vec<i32>) -> Vec<i32> {
        println!("next_greater_element({:?}, {:?}", q, ns);
        use std::collections::HashMap;

        let mut map = HashMap::with_capacity(ns.len());
        let mut stack = vec![];
        for n in ns {
            while let Some(&k) = stack.last() {
                if k < n {
                    map.insert(stack.pop().unwrap(), n);
                } else {
                    break;
                }
            }
            stack.push(n);
        }
        while let Some(k) = stack.pop() {
            map.insert(k, -1);
        }
        q.iter().map(|n| map.get(n).unwrap().to_owned()).collect()
    }
    /// 10:03-10:06
    pub fn next_greater_element_brute_force_2(q: Vec<i32>, ns: Vec<i32>) -> Vec<i32> {
        println!("next_greater_element({:?}, {:?}", q, ns);
        q.into_iter()
            .map(|n1| {
                ns.iter()
                    .enumerate()
                    .find(|&(_, &n2)| n2 == n1)
                    .and_then(|(i, _)| ns[i..].iter().find(|&&n2| n2 > n1))
                    .unwrap_or(&-1)
                    .to_owned()
            })
            .collect()
    }
    /// 09:52-10:03
    pub fn next_greater_element_brute_force_1(q: Vec<i32>, ns: Vec<i32>) -> Vec<i32> {
        println!("next_greater_element({:?}, {:?}", q, ns);
        q.into_iter()
            .map(|n1| {
                ns.iter()
                    .enumerate()
                    .find(|&(_, &n2)| n2 == n1)
                    .map(|(i, _)| (i, n1))
                    .unwrap()
            })
            .map(|(i, n1)| {
                ns[i..]
                    .iter()
                    .find(|&&n2| n2 > n1)
                    .map(|n| n.to_owned())
                    .unwrap_or(-1)
            })
            .map(|n| n.to_owned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_4_1_2_n_1_3_4_2() {
        let n1 = vec![4, 1, 2];
        let n2 = vec![1, 3, 4, 2];
        let e = [-1, 3, -1];
        assert_eq!(Solution::next_greater_element(n1, n2), e);
        // Explanation: The next greater element for each value of nums1 is as follows:
        // - 4: There is no next greater element, so the answer is -1.
        // - 1: The next greater element is 3.
        // - 2: There is no next greater element, so the answer is -1.
    }
    #[test]
    fn n_2_4_n_1_2_3_4() {
        let n1 = vec![2, 4];
        let n2 = vec![1, 2, 3, 4];
        let e = [3, -1];
        assert_eq!(Solution::next_greater_element(n1, n2), e);
        // Explanation: The next greater element for each value of nums1 is as follows:
        // - 2: The next greater element is 3.
        // - 4: There is no next greater element, so the answer is -1.
    }
    #[test]
    fn n_2_4_n_4_3_2_1_5() {
        let n1 = vec![1, 2, 3, 4];
        let n2 = vec![4, 3, 2, 1, 5];
        let e = [5, 5, 5, 5];
        assert_eq!(Solution::next_greater_element(n1, n2), e);
    }
    #[test]
    fn n_2_4_n_4_2_1_3_5() {
        let n1 = vec![1, 2, 3, 4];
        let n2 = vec![4, 2, 1, 3, 5];
        let e = [3, 3, 5, 5];
        assert_eq!(Solution::next_greater_element(n1, n2), e);
    }

    #[test]
    fn n_1to1000_n_1to1000() {
        let n1: Vec<i32> = (1..=1000).collect();
        let n2 = n1.clone();
        let e: Vec<i32> = (2..=1001).map(|i| if i > 1000 { -1 } else { i }).collect();
        assert_eq!(Solution::next_greater_element(n1, n2), e);
    }
    #[test]
    fn n_1000to1_n_1000to1() {
        let n1: Vec<i32> = (1..=1000).rev().collect();
        let n2 = n1.clone();
        let e = vec![-1; 1000];
        assert_eq!(Solution::next_greater_element(n1, n2), e);
    }
}
