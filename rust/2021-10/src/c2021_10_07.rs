#![allow(dead_code)]
/// 79. Word Search
/// ===============
///
/// Given an `m x n` grid of characters `board` and a string `word`,
/// return _`true` if `word` exists in the grid_.
///
/// The word can be constructed from letters of sequentially adjacent cells,
/// where adjacent cells are horizontally or vertically neighboring.
/// The same letter cell may not be used more than once.
///
/// __Constraints:__
///
/// - `1 <= board.length, board[i].length <= 6`
/// - `1 <= word.length <= 15`
/// - `board` and `word` consists of only lowercase and uppercase English letters.
///
/// __Follow up:__ Could you use search pruning to make your solution faster with a larger board?
///
/// https://leetcode.com/problems/word-search/
struct Solution;
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        println!("exist({:?}, {})", board, word);
        #[rustfmt::skip]
        fn bts(r: usize, c: usize, i: usize,
            used: &mut Vec<Vec<bool>>,
            tried: &mut Vec<Vec<Vec<bool>>>,
            b: &[Vec<char>], w: &[char],
        ) -> bool {
            if i >= w.len() {
                true
            } else if b[r][c] != w[i] || used[r][c] || tried[r][c][i] {
                false
            } else {
                used[r][c] = true;
                for &(r, c) in [(r.wrapping_sub(1), c), (r + 1, c),
                                (r, c.wrapping_sub(1)), (r, c + 1)]
                               .iter()
                               .filter(|&&(r, c)| r < b.len() && c < b[0].len()) {
                    if bts(r, c, i + 1, used, tried, b, w) {
                        return true;
                    }
                }
                used[r][c] = false;
                tried[r][c][i] = true;
                false
            }
        }
        let m = board.len();
        let n = board[0].len();
        let w = word.chars().collect::<Vec<_>>();
        let mut used = vec![vec![false; n]; m];
        let mut tried = vec![vec![vec![false; w.len()]; n]; m];
        for r in 0..m {
            for c in 0..n {
                if bts(r, c, 0, &mut used, &mut tried, &board, &w) {
                    return true;
                }
            }
        }
        false
    }
    pub fn exist_backtrack(board: Vec<Vec<char>>, word: String) -> bool {
        println!("exist({:?}, {})", board, word);
        #[rustfmt::skip]
        fn bts(r: usize, c: usize, i: usize, used: &mut Vec<Vec<bool>>, b: &[Vec<char>], w: &[char]) -> bool {
            if i >= w.len() {
                true
            } else if b[r][c] != w[i] || used[r][c] {
                false
            } else {
                used[r][c] = true;
                for &(r, c) in [(r.wrapping_sub(1), c), (r + 1, c),
                                (r, c.wrapping_sub(1)), (r, c + 1)]
                               .iter()
                               .filter(|&&(r, c)| r < b.len() && c < b[0].len()) {
                    if bts(r, c, i + 1, used, b, w) {
                        return true;
                    }
                }
                used[r][c] = false;
                false
            }
        }
        let m = board.len();
        let n = board[0].len();
        let w = word.chars().collect::<Vec<_>>();
        let mut used = vec![vec![false; n]; m];
        for r in 0..m {
            for c in 0..n {
                if bts(r, c, 0, &mut used, &board, &w) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn b_abce_sfcs_adee_w_abcced_uppercase_produces_true() {
        let b = vv![
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E']
        ];
        let w = "ABCCED".to_string();
        assert!(Solution::exist(b, w));
    }
    #[test]
    fn b_abce_sfcw_adee_w_see_uppercase_produces_true() {
        let b = vv![
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E']
        ];
        let w = "SEE".to_string();
        assert!(Solution::exist(b, w));
    }
    #[test]
    fn b_abce_sfcw_adee_w_abcb_uppercase() {
        let b = vv![
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E']
        ];
        let w = "ABCB".to_string();
        assert!(!Solution::exist(b, w));
    }
    #[test]
    fn b_abce_sfcw_adee_w_see_case_matters1_cannot_find() {
        let b = vv![
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E']
        ];
        let w = "SEe".to_string();
        assert!(!Solution::exist(b, w));
    }
    #[test]
    fn b_abce_sfcw_adee_w_see_case_matters2_can_find_produces_true() {
        let b = vv![
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'e', 'E']
        ];
        let w = "SEe".to_string();
        assert!(Solution::exist(b, w));
    }

    #[test]
    fn b_6x6xa_w_14xa1b() {
        let b = vec![vec!['a'; 6]; 6];
        let w = "aaaaaaaaaaaaaab".to_string();
        assert!(!Solution::exist(b, w));
    }
}
