#![allow(dead_code)]
/// 799. Champagne Tower
/// ====================
///
/// We stack glasses in a pyramid, where the __first__ row has `1` glass,
/// the __second__ row has `2` glasses, and so on until the 100th row.
/// Each glass holds one cup of champagne.
///
/// Then, some champagne is poured into the first glass at the top.
/// When the topmost glass is full, any excess liquid poured will fall equally
/// to the glass immediately to the left and right of it.
/// When those glasses become full, any excess champagne will fall equally
/// to the left and right of those glasses, and so on.
/// (A glass at the bottom row has its excess champagne fall on the floor.)
///
/// For example, after one cup of champagne is poured, the top most glass is full.
/// After two cups of champagne are poured, the two glasses on the second row are half full.
/// After three cups of champagne are poured, those two cups become full - there are 3 full glasses total now.
/// After four cups of champagne are poured, the third row has the middle glass half full,
/// and the two outside glasses are a quarter full.
///
/// Now after pouring some non-negative integer cups of champagne,
/// return how full the `jth` glass in the `ith` row is (both `i` and `j` are 0-indexed.)
///
/// __Constraints:__
///
/// - `0 <= poured <= 1_000_000_000`
/// - `0 <= query_glass <= query_row < 100`
///
/// https://leetcode.com/problems/champagne-tower/
struct Solution;
impl Solution {
    /// from old scala solution
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let row = query_row as usize;
        let mut tower = vec![vec![0.0; row + 2]; row + 2];
        tower[0][0] = poured as f64;
        for r in 0..=row {
            for c in 0..=r {
                let excess = (tower[r][c] - 1.0) / 2.0;
                if excess > 0.0 {
                    tower[r + 1][c] += excess;
                    tower[r + 1][c + 1] += excess;
                }
            }
        }
        tower[row][query_glass as usize].min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_1_r_1_g_1() {
        assert_eq!(Solution::champagne_tower(1, 1, 1), 0.00000);
        // Explanation: We poured 1 cup of champange to the top glass of the tower (which is indexed as (0, 0)).
        // There will be no excess liquid so all the glasses under the top glass will remain empty.
    }
    #[test]
    fn p_2_r_1_g_1() {
        assert_eq!(Solution::champagne_tower(2, 1, 1), 0.50000);
        // Explanation: We poured 2 cups of champange to the top glass of the tower (which is indexed as (0, 0)).
        // There is one cup of excess liquid. The glass indexed as (1, 0) and the glass indexed as (1, 1)
        // will share the excess liquid equally, and each will get half cup of champange.
    }
    #[test]
    fn p_4_r_2_g_0() {
        assert_eq!(Solution::champagne_tower(4, 2, 0), 0.25000);
    }
    #[test]
    fn p_100_000_009_r_33_g_17() {
        assert_eq!(Solution::champagne_tower(100_000_009, 33, 17), 1.00000);
    }
    #[test]
    fn p_1_000_000_000_r_99_g_99() {
        assert_eq!(Solution::champagne_tower(1_000_000_000, 99, 99), 0.00000);
    }
    #[test]
    fn p_1_000_000_000_r_99_g_77() {
        assert_eq!(Solution::champagne_tower(1_000_000_000, 99, 77), 1.00000);
    }
    #[test]
    fn p_1_000_000_000_r_99_g_78() {
        assert_eq!(Solution::champagne_tower(1_000_000_000, 99, 78), 0.00000);
    }
}
