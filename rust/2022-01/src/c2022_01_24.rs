#![allow(dead_code)]
/// 520. Detect Capital
/// ===================
///
/// We define the usage of capitals in a word to be right when one of the following cases holds:
///
/// - All letters in this word are capitals, like `"USA"`.
/// - All letters in this word are not capitals, like `"leetcode"`.
/// - Only the first letter in this word is capital, like `"Google"`.
///
/// Given a string `word`, return `true` if the usage of capitals in it is right.
///
/// __Constraints:__
///
/// - `1 <= word.length <= 100`
/// - `word` consists of lowercase and uppercase English letters.
///
/// https://leetcode.com/problems/detect-capital/
struct Solution;
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        word.chars().all(|c| c.is_uppercase()) || word.chars().skip(1).all(|c| c.is_lowercase())
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[rustfmt::skip] #[test] fn a()  { assert_eq!(Solution::detect_capital_use("a".into()), true); }
    #[rustfmt::skip] #[test] fn A()  { assert_eq!(Solution::detect_capital_use("A".into()), true); }

    #[rustfmt::skip] #[test] fn ab()  { assert_eq!(Solution::detect_capital_use("ab".into()), true); }
    #[rustfmt::skip] #[test] fn AB()  { assert_eq!(Solution::detect_capital_use("AB".into()), true); }
    #[rustfmt::skip] #[test] fn Ab()  { assert_eq!(Solution::detect_capital_use("Ab".into()), true); }
    #[rustfmt::skip] #[test] fn aB()  { assert_eq!(Solution::detect_capital_use("aB".into()), false); }

    #[rustfmt::skip] #[test] fn USA()  { assert_eq!(Solution::detect_capital_use("USA".into()), true); }
    #[rustfmt::skip] #[test] fn FlaG() { assert_eq!(Solution::detect_capital_use("FlaG".into()), false); }
}
