#![allow(dead_code)]
//! \#1996. The Number of Weak Characters in the Game
//! =================================================
//!
//! <https://leetcode.com/problems/the-number-of-weak-characters-in-the-game>
//!
//! You are playing a game that contains multiple characters,
//! and each of the characters has __two__ main properties: __attack__ and __defense__.
//! You are given a 2D integer array `properties` where `properties[i] = [attack_i, defense_i]`
//! represents the properties of the `i`th character in the game.
//!
//! A character is said to be __weak__ if any other character has __both__ attack and defense
//! levels __strictly greater__ than this character's attack and defense levels.
//! More formally, a character `i` is said to be __weak__ if there exists another
//! character `j` where `attack_j > attack_i` and `defense_j > defense_i`.
//!
//! Return _the number of __weak__ characters_.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_09::c2022_09_09::*;
//! # use c2022_09::vv;
//! let properties = vv![[5, 5], [6, 3], [3, 6]];
//! assert_eq!(Solution::number_of_weak_characters(properties), 0);
//! ```
//!
//! __Explanation:__ No character has strictly greater attack and defense than the other.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_09::c2022_09_09::*;
//! # use c2022_09::vv;
//! let properties = vv![[2, 2], [3, 3]];
//! assert_eq!(Solution::number_of_weak_characters(properties), 1);
//! ```
//!
//! __Explanation:__ The first character is weak because the second character has a strictly greater attack and defense.
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_09::c2022_09_09::*;
//! # use c2022_09::vv;
//! let properties = vv![[1, 5], [10, 4], [4, 3]];
//! assert_eq!(Solution::number_of_weak_characters(properties), 1);
//! ```
//!
//! __Explanation:__ The third character is weak because the second character has a strictly greater attack and defense.
//!
//! ##### Constraints
//!
//! - `2 <= properties.length <= 100_000`
//! - `properties[i].length == 2`
//! - `1 <= attack_i, defense_i <= 100_000`

pub struct Solution;
impl Solution {
    pub fn number_of_weak_characters_v1(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_unstable();

        let n = properties.len();
        let mut weak = 0;

        let max_atack = properties[n - 1][0];
        let mut curr_atack = max_atack;

        let mut prev_max_defence = 0;
        let mut curr_max_defence = 0;

        for i in (0..n).rev() {
            let (atack, defence) = (properties[i][0], properties[i][1]);
            if atack < curr_atack {
                prev_max_defence = curr_max_defence;
                curr_atack = atack;
            }
            curr_max_defence = curr_max_defence.max(defence);
            if atack < max_atack && defence < prev_max_defence {
                weak += 1;
            }
        }

        weak
    }

    pub fn number_of_weak_characters_v2(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_unstable_by_key(|p| (p[0], -p[1]));

        let mut weak = 0;
        let mut max_defence = 0;

        for i in (0..properties.len()).rev() {
            let defence = properties[i][1];
            if defence < max_defence {
                weak += 1;
            }
            max_defence = max_defence.max(defence);
        }

        weak
    }

    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let max_atack = properties.iter().map(|p| p[0]).max().unwrap() as usize;

        let mut max_defence = vec![0; max_atack + 2];
        for (atack, defence) in properties.iter().map(|p| (p[0] as usize, p[1])) {
            max_defence[atack] = max_defence[atack].max(defence);
        }

        for atack in (0..max_atack).rev() {
            max_defence[atack] = max_defence[atack].max(max_defence[atack + 1]);
        }

        let mut weak = 0;
        for (atack, defence) in properties.iter().map(|p| (p[0] as usize, p[1])) {
            if defence < max_defence[atack + 1] {
                weak += 1;
            }
        }

        weak
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vv;

    #[test]
    fn p_5_5_p_6_3_p_3_6() {
        let properties = vv![[5, 5], [6, 3], [3, 6]];
        assert_eq!(Solution::number_of_weak_characters(properties), 0);
    }

    #[test]
    fn p_2_2_p_3_3() {
        let properties = vv![[2, 2], [3, 3]];
        assert_eq!(Solution::number_of_weak_characters(properties), 1);
    }

    #[test]
    fn p_1_5_p_10_4_p_4_3() {
        let properties = vv![[1, 5], [10, 4], [4, 3]];
        assert_eq!(Solution::number_of_weak_characters(properties), 1);
    }

    #[test]
    fn p_1_1_p_2_1_p_1_2() {
        let properties = vv![[1, 1], [2, 1], [1, 2]];
        assert_eq!(Solution::number_of_weak_characters(properties), 0);
    }
}
