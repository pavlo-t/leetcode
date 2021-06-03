#![allow(dead_code)]

/// ### Diagonal Traverse
///
/// https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3580/
struct Solution;

impl Solution {
    pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }

        enum Direction { U, D }
        use Direction::*;

        let m = matrix.len();
        let n = matrix[0].len();
        let lr = m - 1;
        let lc = n - 1;

        (0..(m * n)).fold(
            (vec![0; m * n], U, 0, 0),
            |(mut rsf, d, r, c), i| {
                rsf[i] = matrix[r][c];
                match (d, r, c) {
                    (U, r, c) if c == lc => (rsf, D, r + 1, c),
                    (U, r, c) if r == 0 => (rsf, D, r, c + 1),
                    (U, r, c) => (rsf, U, r - 1, c + 1),
                    (D, r, c) if r == lr => (rsf, U, r, c + 1),
                    (D, r, c) if c == 0 => (rsf, U, r + 1, c),
                    (D, r, c) => (rsf, D, r + 1, c - 1),
                }
            }).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_m123_456_789_is_124753689() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]];
        let expected = vec![1, 2, 4, 7, 5, 3, 6, 8, 9];

        assert_eq!(Solution::find_diagonal_order(matrix), expected);
    }

    #[test]
    fn m_empty_is_empty() {
        assert_eq!(Solution::find_diagonal_order(vec![]), []);
    }

    #[test]
    fn m1_is_1() {
        assert_eq!(Solution::find_diagonal_order(vec![vec![1]]), [1]);
    }

    #[test]
    fn m12_is_12() {
        assert_eq!(Solution::find_diagonal_order(vec![vec![1, 2]]), [1, 2]);
    }

    #[test]
    fn m1_2_is_12() {
        assert_eq!(Solution::find_diagonal_order(vec![vec![1], vec![2]]), [1, 2]);
    }

    #[test]
    fn m10000_1s_is_12() {
        let matrix = vec![vec![1; 100]; 100];
        assert_eq!(Solution::find_diagonal_order(matrix), vec![1; 10000]);
    }
}
