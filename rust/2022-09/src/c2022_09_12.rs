#![allow(dead_code)]
//! \#948. Bag of Tokens
//! ====================
//!
//! <https://leetcode.com/problems/bag-of-tokens>
//!
//! You have an initial __power__ of `power`, an initial __score__ of `0`, and a bag of `tokens`
//! where `tokens[i]` is the value of the `i`th token (0-indexed).
//!
//! Your goal is to maximize your total __score__ by potentially playing each token in one of two ways:
//!
//! - If your current __power__ is at least `tokens[i]`, you may play the `i`th token face up,
//!   losing `tokens[i]` __power__ and gaining `1` __score__.
//! - If your current __score__ is at least `1`, you may play the `i`th token face down,
//!   gaining `tokens[i]` __power__ and losing `1` __score__.
//!
//! Each token may be played __at most__ once and __in any order__.
//! You do __not__ have to play all the tokens.
//!
//! Return _the largest possible __score__ you can achieve after playing any number of tokens_.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_09::c2022_09_12::*;
//! assert_eq!(Solution::bag_of_tokens_score(vec![100], 50), 0);
//! ```
//!
//! __Explanation:__ Playing the only token in the bag is impossible
//! because you either have too little power or too little score.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_09::c2022_09_12::*;
//! assert_eq!(Solution::bag_of_tokens_score(vec![100, 200], 150), 1);
//! ```
//!
//! __Explanation:__ Play the `0`th token (`100`) face up, your power becomes `50` and score becomes `1`.
//! There is no need to play the `1`st token since you cannot play it face up to add to your score.
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_09::c2022_09_12::*;
//! assert_eq!(Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200), 2);
//! ```
//!
//! __Explanation:__ Play the tokens in this order to get a score of `2`:
//!
//! 1. Play the `0`th token (`100`) face up, your power becomes `100` and score becomes `1`.
//! 2. Play the `3`rd token (`400`) face down, your power becomes `500` and score becomes `0`.
//! 3. Play the `1`st token (`200`) face up, your power becomes `300` and score becomes `1`.
//! 4. Play the `2`nd token (`300`) face up, your power becomes `0` and score becomes `2`.
//!
//! ##### Constraints
//!
//! - `0 <= tokens.length <= 1000`
//! - `0 <= tokens[i], power < 10_000`

pub struct Solution;
impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        if tokens.is_empty() {
            0
        } else {
            tokens.sort_unstable();

            let (mut l, mut r) = (0, tokens.len() - 1);
            let mut score = 0;
            while l <= r {
                if power >= tokens[l] {
                    power -= tokens[l];
                    score += 1;
                    l += 1;
                } else if l != r && score > 0 && power + tokens[r] >= tokens[l] {
                    power += tokens[r];
                    score -= 1;
                    r -= 1;
                } else {
                    break;
                }
            }

            score
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_empty_p_50() {
        assert_eq!(Solution::bag_of_tokens_score(vec![], 50), 0);
    }

    #[test]
    fn t_100_p_50() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100], 50), 0);
    }

    #[test]
    fn t_100_p_100() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100], 100), 1);
    }

    #[test]
    fn t_100_200_p_150() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100, 200], 150), 1);
    }

    #[test]
    fn t_100_200_300_400_p_200() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200), 2);
    }
}
