#![allow(dead_code)]

/// ### The K Weakest Rows in a Matrix
///
/// Given a `m * n` matrix `mat` of _ones_ (representing soldiers) and _zeros_ (representing civilians),
/// return the indexes of the `k` weakest rows in the matrix ordered from the weakest to the strongest.
///
/// A row `i` is weaker than row `j`, if the number of soldiers in row `i` is less than the number
/// of soldiers in row `j`, or they have the same number of soldiers but `i` is less than `j`.
/// Soldiers are __always__ stand in the frontier of a row, that is,
/// always _ones_ may appear first and then _zeros_.
///
/// __Constraints:__
///
/// - `2 <= mat.length, mat[i].length <= 100`
/// - `1 <= k <= m`
/// - `matrix[i][j]` is either `0` __or__ `1`.
///
/// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/586/week-3-february-15th-february-21st/3641/
struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut strength = mat.into_iter().enumerate()
            .map(|(i, r)| (r.iter().sum(), i))
            .collect::<Vec<(i32, usize)>>();
        strength.sort();
        strength.into_iter()
            .map(|(_, i)| i as i32)
            .take(k as usize)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mat = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1]];
        assert_eq!(Solution::k_weakest_rows(mat, 3), [2, 0, 3]);
        //Explanation:
        // The number of soldiers for each row is:
        // row 0 -> 2
        // row 1 -> 4
        // row 2 -> 1
        // row 3 -> 2
        // row 4 -> 5
        // Rows ordered from the weakest to the strongest are [2,0,3,1,4]
    }

    #[test]
    fn example2() {
        let mat = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0]];
        assert_eq!(Solution::k_weakest_rows(mat, 2), [0, 2]);
        //Explanation:
        // The number of soldiers for each row is:
        // row 0 -> 1
        // row 1 -> 4
        // row 2 -> 1
        // row 3 -> 1
        // Rows ordered from the weakest to the strongest are [0,2,3,1]
    }
}
