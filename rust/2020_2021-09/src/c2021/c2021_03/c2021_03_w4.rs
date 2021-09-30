#![allow(dead_code)]
/// Find Smallest Common Element in All Rows
/// ========================================
///
/// Given a matrix `mat` where every row is sorted in __strictly increasing__ order,
/// return the __smallest common element__ in all rows.
///
/// If there is no common element, return `-1`.
///
/// __Constraints:__
///
/// - `1 <= mat.length, mat[i].length <= 500`
/// - `1 <= mat[i][j] <= 10^4`
/// - `mat[i]` is sorted in strictly increasing order.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3680/
struct Solution;
impl Solution {
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let mut idxs = vec![0; mat.len()];
        let mut candidate = mat[0][0];
        let mut r = 0;
        loop {
            if r >= mat.len() {
                return mat[0][idxs[0]];
            }
            if idxs[r] >= mat[r].len() {
                return -1;
            }
            match mat[r][idxs[r]] {
                v if v == candidate => r += 1,
                v if v < candidate => idxs[r] += 1,
                v => {
                    candidate = v;
                    r = if r == 0 { 1 } else { 0 }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mat = vec![
            vec![1, 2, 3, 4, 5],
            vec![2, 4, 5, 8, 10],
            vec![3, 5, 7, 9, 11],
            vec![1, 3, 5, 7, 9],
        ];
        assert_eq!(Solution::smallest_common_element(mat), 5);
    }
    #[test]
    fn common_first_col() {
        let mat = vec![vec![3], vec![3], vec![3]];
        assert_eq!(Solution::smallest_common_element(mat), 3);
    }
    #[test]
    fn no_common_element() {
        let mat = vec![vec![1], vec![2], vec![3]];
        assert_eq!(Solution::smallest_common_element(mat), -1);
    }

    mod performance {
        use super::*;

        #[test]
        fn mat_500x500_no_common_element() {
            let mat = (0..500).map(|i| vec![i; 500]).collect();
            assert_eq!(Solution::smallest_common_element(mat), -1);
        }
        #[test]
        fn mat_500x500_common_in_last_col() {
            let mat = (0..500)
                .map(|i| {
                    let mut v = vec![i; 499];
                    v.push(1000);
                    v
                })
                .collect();
            assert_eq!(Solution::smallest_common_element(mat), 1000);
        }
    }
}
