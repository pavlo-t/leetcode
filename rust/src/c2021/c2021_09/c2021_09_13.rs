#![allow(dead_code)]
/// Maximum Number of Balloons
/// ==========================
///
/// Given a string `text`, you want to use the characters of `text` to
/// form as many instances of the word __"balloon"__ as possible.
///
/// You can use each character in `text` __at most once__.
/// Return the maximum number of instances that can be formed.
///
/// __Constraints:__
///
/// - `1 <= text.length <= 10_000`
/// - `text` consists of lower case English letters only.
///
/// https://leetcode.com/explore/challenge/card/september-leetcoding-challenge-2021/637/week-2-september-8th-september-14th/3973/
struct Solution;
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        println!("max_number_of_balloons({})", text);
        let (mut b, mut a, mut l, mut o, mut n) = (0, 0, 0, 0, 0);
        text.chars().for_each(|c| match c {
            'b' => b += 1,
            'a' => a += 1,
            'l' => l += 1,
            'o' => o += 1,
            'n' => n += 1,
            _ => (),
        });
        l /= 2;
        o /= 2;
        b.min(a).min(l).min(o).min(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nlaebolko_1() {
        let text = "nlaebolko".to_string();
        assert_eq!(Solution::max_number_of_balloons(text), 1);
    }
    #[test]
    fn loonbalxballpoon_2() {
        let text = "loonbalxballpoon".to_string();
        assert_eq!(Solution::max_number_of_balloons(text), 2);
    }
    #[test]
    fn leetcode_0() {
        let text = "leetcode".to_string();
        assert_eq!(Solution::max_number_of_balloons(text), 0);
    }
}
