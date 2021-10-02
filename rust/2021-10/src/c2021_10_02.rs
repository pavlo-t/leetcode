#![allow(dead_code)]
/// 174. Dungeon Game
/// =================
///
/// The demons had captured the princess and imprisoned her in the __bottom-right corner__ of a `dungeon`.
/// The `dungeon` consists of `m x n` rooms laid out in a 2D grid.
/// Our valiant knight was initially positioned in the __top-left room__
/// and must fight his way through `dungeon` to rescue the princess.
///
/// The knight has an initial health point represented by a positive integer.
/// If at any point his health point drops to `0` or below, he dies immediately.
///
/// Some of the rooms are guarded by demons (represented by negative integers),
/// so the knight loses health upon entering these rooms;
/// other rooms are either empty (represented as 0)
/// or contain magic orbs that increase the knight's health (represented by positive integers).
///
/// To reach the princess as quickly as possible,
/// the knight decides to move only __rightward__ or __downward__ in each step.
///
/// Return _the knight's minimum initial health so that he can rescue the princess_.
///
/// __Note__ that any room can contain threats or power-ups,
/// even the first room the knight enters and the bottom-right room where the princess is imprisoned.
///
/// __Constraints:__
///
/// - `1 <= dungeon.length, dungeon[i].length <= 200
/// - `-1000 <= dungeon[i][j] <= 1000
///
/// https://leetcode.com/problems/dungeon-game/
struct Solution;
impl Solution {
    pub fn calculate_minimum_hp(mut d: Vec<Vec<i32>>) -> i32 {
        println!("calculate_minimum_hp(");
        d.iter().for_each(|r| println!("  {:?}", r));
        let (m, n) = (d.len(), d[0].len());
        let (lr, lc) = (m - 1, n - 1);
        d[lr][lc] = (1 - d[lr][lc]).max(1);
        for r in (0..lr).rev() {
            d[r][lc] = (d[r + 1][lc] - d[r][lc]).max(1);
        }
        for c in (0..lc).rev() {
            d[lr][c] = (d[lr][c + 1] - d[lr][c]).max(1);
        }
        for c in (0..lc).rev() {
            for r in (0..lr).rev() {
                d[r][c] = (d[r][c + 1].min(d[r + 1][c]) - d[r][c]).max(1);
            }
        }
        d.iter().for_each(|r| println!("  {:?}", r));
        d[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn d_m2m3p3_m5m10p1_p10p30m5() {
        #[rustfmt::skip]
        let d = vv![
            [-2,  -3,  3],
            [-5, -10,  1],
            [10,  30, -5]];
        assert_eq!(Solution::calculate_minimum_hp(d), 7);
        // Explanation: The initial health of the knight must be at least 7
        // if he follows the optimal path: RIGHT-> RIGHT -> DOWN -> DOWN.
    }
    #[test]
    fn d_m2m3p3_m5m10p1_p10p30p5() {
        #[rustfmt::skip]
        let d = vv![
            [-2,  -3, 3],
            [-5, -10, 1],
            [10,  30, 5]];
        assert_eq!(Solution::calculate_minimum_hp(d), 6);
    }
    #[test]
    fn d_0() {
        let d = vv![[0]];
        assert_eq!(Solution::calculate_minimum_hp(d), 1);
    }
    #[test]
    fn d_3() {
        let d = vv![[3]];
        assert_eq!(Solution::calculate_minimum_hp(d), 1);
    }

    mod performance {
        use super::*;

        #[test]
        fn d_200x200x1() {
            let d = vec![vec![1; 200]; 200];
            assert_eq!(Solution::calculate_minimum_hp(d), 1);
        }
        #[test]
        fn d_200x200xm1() {
            let d = vec![vec![-1; 200]; 200];
            assert_eq!(Solution::calculate_minimum_hp(d), 400);
        }
    }
}
