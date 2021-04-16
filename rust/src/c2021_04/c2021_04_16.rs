#![allow(dead_code)]
/// Remove All Adjacent Duplicates in String II
/// ===========================================
///
/// Given a string `s`, a _k duplicate removal_ consists of choosing `k` adjacent and equal letters
/// from `s` and removing them causing the left and the right side of the deleted substring to
/// concatenate together.
///
/// We repeatedly make `k` duplicate removals on `s` until we no longer can.
///
/// Return the final string after all such duplicate removals have been made.
///
/// It is guaranteed that the answer is unique.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 100_000`
/// - `2 <= k <= 10_000`
/// - `s` only contains lower case English letters.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/595/week-3-april-15th-april-21st/3710/
struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        s.chars()
            .fold(Vec::new(), |mut stack, x| {
                match stack.pop() {
                    Some((y, c)) if y == x && c + 1 < k => stack.push((x, c + 1)),
                    Some((y, _)) if y == x => (),
                    other => {
                        if let Some(pc) = other {
                            stack.push(pc);
                        }
                        stack.push((x, 1))
                    }
                }
                stack
            })
            .into_iter()
            .fold(String::new(), |mut r, (c, cnt)| {
                (0..cnt).for_each(|_| r.push(c));
                r
            })
    }

    pub fn remove_duplicates_iterative(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i32)> = Vec::new();
        for c in s.chars() {
            match stack.pop() {
                Some((pc, cnt)) if pc == c && cnt < k - 1 => stack.push((c, cnt + 1)),
                Some((pc, _)) if pc == c => (),
                e => {
                    if let Some((pc, cnt)) = e {
                        stack.push((pc, cnt));
                    }
                    stack.push((c, 1))
                }
            }
        }

        let mut result = String::new();
        for (c, cnt) in stack {
            result.push_str(&c.to_string().repeat(cnt as usize));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_s_abcd_k2_produces_abcd() {
        assert_eq!(Solution::remove_duplicates("abcd".to_string(), 2), "abcd");
        // Explanation: There's nothing to delete.
    }
    #[test]
    fn example2_s_deeedbbcccbdaa_k3_produces_aa() {
        let s = "deeedbbcccbdaa".to_string();
        assert_eq!(Solution::remove_duplicates(s, 3), "aa");
        // Explanation:
        // First delete "eee" and "ccc", get "ddbbbdaa"
        // Then delete "bbb", get "dddaa"
        // Finally delete "ddd", get "aa"
    }
    #[test]
    fn example3_s_pbbcggttciiippooaais_k2_produces_ps() {
        let s = "pbbcggttciiippooaais".to_string();
        assert_eq!(Solution::remove_duplicates(s, 2), "ps");
    }

    mod performance {
        use super::*;

        #[test]
        fn test_s_100_000_a_k2_produces_empty_string() {
            let s = "a".repeat(100_000);
            assert_eq!(Solution::remove_duplicates(s, 2), "");
        }
        #[test]
        fn test_s_100_000_chars_k2_produces_a() {
            let mut s = "abcdeedcba".repeat(10_000);
            s.push('a');
            assert_eq!(Solution::remove_duplicates(s, 2), "a");
        }
    }
}
