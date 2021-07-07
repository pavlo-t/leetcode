#![allow(dead_code)]
/// Kth Smallest Element in a Sorted Matrix
/// =======================================
///
/// Given an `n x n` `matrix` where each of the rows and columns are sorted in ascending order,
/// return _the `k`th smallest element in the matrix_.
///
/// Note that it is the `k`th smallest element __in the sorted order__, not the `k`th __distinct__ element.
///
/// __Constraints:__
///
/// - `n == matrix.length`
/// - `n == matrix[i].length`
/// - `1 <= n <= 300`
/// - `-1_000_000_000 <= matrix[i][j] <= 1_000_000_000`
/// - All the rows and columns of `matrix` are __guaranteed__ to be sorted in __non-decreasing order__.
/// - `1 <= k <= n^2`
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/608/week-1-july-1st-july-7th/3805/
struct Solution;
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut v = matrix.into_iter().flat_map(|v| v).collect::<Vec<_>>();
        v.sort_unstable();
        v[(k - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn m1_5_9r10_11_13r12_13_15_k8_produces_13() {
        let matrix = vv![
            [1, 5, 9],
            [10, 11, 13],
            [12, 13, 15]];
        assert_eq!(Solution::kth_smallest(matrix, 8), 13);
        //Explanation: The elements in the matrix are [1,5,9,10,11,12,13,13,15], and the 8th smallest number is 13
    }
    #[test]
    fn mm5_k1_produces_m5() {
        let matrix = vv![[-5]];
        assert_eq!(Solution::kth_smallest(matrix, 1), -5);
    }

    mod performance {
        use super::*;

        #[test]
        fn m300x300_1s_k90000_produces_1() {
            let matrix = vec![vec![1; 300]; 300];
            assert_eq!(Solution::kth_smallest(matrix, 90000), 1);
        }
    }
}
