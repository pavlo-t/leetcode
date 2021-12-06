#![allow(dead_code)]
/// 1217. Minimum Cost to Move Chips to The Same Position
/// =====================================================
///
/// We have `n` chips, where the position of the `i`th chip is `position[i]`.
///
/// We need to move all the chips to __the same position__.
/// In one step, we can change the position of the `i`th chip from `position[i]` to:
///
/// - `position[i] + 2` or `position[i] - 2` with `cost = 0`.
/// - `position[i] + 1` or `position[i] - 1` with `cost = 1`.
///
/// Return _the minimum cost_ needed to move all the chips to the same position.
///
/// __Constraints:__
///
/// - `1 <= position.length <= 100`
/// - `1 <= position[i] <= 1_000_000_000`
///
/// https://leetcode.com/problems/minimum-cost-to-move-chips-to-the-same-position/
struct Solution;
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        println!("min_cost_to_move_chips({:?})", position);
        position
            .into_iter()
            .map(|p| (p % 2) as usize)
            .fold([0, 0], |mut results, p| {
                results[p] += 1;
                results
            })
            .into_iter()
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_1_2_3() {
        let p = vec![1, 2, 3];
        assert_eq!(Solution::min_cost_to_move_chips(p), 1);
        // Explanation:
        // First step: Move the chip at position 3 to position 1 with cost = 0.
        // Second step: Move the chip at position 2 to position 1 with cost = 1.
        // Total cost is 1.
    }
    #[test]
    fn p_2_2_2_3_3() {
        let p = vec![2, 2, 2, 3, 3];
        assert_eq!(Solution::min_cost_to_move_chips(p), 2);
        // Explanation:
        // We can move the two chips at position  3 to position 2.
        // Each move has cost = 1.
        // The total cost = 2.
    }
    #[test]
    fn p_1_1_000_000_000() {
        let p = vec![1, 1000000000];
        assert_eq!(Solution::min_cost_to_move_chips(p), 1);
    }
}
