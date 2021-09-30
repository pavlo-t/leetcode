#![allow(dead_code)]
/// Verify Preorder Serialization of a Binary Tree
/// ==============================================
///
/// One way to serialize a binary tree is to use __preorder traversal__.
/// When we encounter a non-null node, we record the node's value.
/// If it is a null node, we record using a sentinel value such as `'#'`.
///
/// ```text
///     9
///    / \
///   3   2
///  / \   \
/// 4   1   6
/// ```
///
/// For example, the above binary tree can be serialized to the string `"9,3,4,#,#,1,#,#,2,#,6,#,#"`,
/// where `'#'` represents a null node.
///
/// Given a string of comma-separated values `preorder`,
/// return `true` if it is a correct preorder traversal serialization of a binary tree.
///
/// It is __guaranteed__ that each comma-separated value in the string must be either an integer
/// or a character `'#'` representing null pointer.
///
/// You may assume that the input format is always valid.
///
/// - For example, it could never contain two consecutive commas, such as `"1,,3"`.
///
/// __Note:__ You are not allowed to reconstruct the tree.
///
/// __Constraints:__
///
/// - `1 <= preorder.length <= 10_000`
/// - `preoder` consist of integers in the range `[0, 100]` and `'#'` separated by commas `','`.
///
/// https://leetcode.com/explore/challenge/card/august-leetcoding-challenge-2021/616/week-4-august-22nd-august-28th/3920/
struct Solution;
impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        fn next_node(s: &str) -> Option<(Option<i32>, &str)> {
            match s.find(',') {
                Some(i) => Some((s[..i].parse().ok(), &s[i + 1..])),
                None if s.len() > 0 => Some((s.parse().ok(), "")),
                _ => None,
            }
        }

        let mut slots = 0;
        let mut s: &str = &preorder;
        while let Some((v, ns)) = next_node(s) {
            if v.is_none() {
                slots -= 1;
                if slots < 0 {
                    return ns.len() == 0;
                }
            } else {
                slots += 1;
            }
            s = ns;
        }
        false
    }
    // https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/solution/389071
    //
    // "9,3,4,#,#,1,#,#,2,#,6,#,#"
    // 9 -> 9
    // 3 -> 9,3
    // 4 -> 9,3,4
    // # -> 9,3
    // # -> 9
    // 1 -> 9,1
    // # -> 9
    // # -> []
    // 2 -> 2
    // # -> []
    // 6 -> 6
    // # -> []
    // # -> return true;
    pub fn is_valid_serialization_better(preorder: String) -> bool {
        fn next_node(s: &str) -> Option<(Option<i32>, &str)> {
            match s.find(',') {
                Some(i) => Some((s[..i].parse().ok(), &s[i + 1..])),
                None if s.len() > 0 => Some((s.parse().ok(), "")),
                _ => None,
            }
        }

        let mut stack = vec![];
        let mut s: &str = &preorder;
        while let Some((v, ns)) = next_node(s) {
            //println!("s: {}, stack: {:?}", s, stack);
            if v.is_none() {
                if stack.pop().is_none() {
                    return ns.len() == 0;
                }
            } else {
                stack.push(true);
            }
            s = ns;
        }
        false
    }
    pub fn is_valid_serialization_my(preorder: String) -> bool {
        fn next_node(s: &str) -> (Option<i32>, Option<&str>) {
            match s.find(',') {
                Some(i) => (s[..i].parse().ok(), Some(&s[i + 1..])),
                None => (s.parse().ok(), None),
            }
        }
        fn drop_leaf(s: &mut Vec<Option<i32>>) -> bool {
            if s.len() > 1 && s.last().map(|n| n.is_none()).unwrap_or(false) {
                s.pop();
                s.pop();
                true
            } else {
                false
            }
        }
        fn mutate_stack(v: Option<i32>, s: &mut Vec<Option<i32>>) -> bool {
            match (v, s.last()) {
                (None, Some(None)) if s.len() == 1 => return false,
                (None, Some(None)) => {
                    while drop_leaf(s) {}
                    s.push(None);
                }
                _ => s.push(v),
            }
            true
        }

        let mut stack = vec![];
        let mut s: &str = &preorder;
        while let (v, Some(ns)) = next_node(s) {
            if !mutate_stack(v, &mut stack) {
                return false;
            }
            s = ns;
        }
        mutate_stack(next_node(s).0, &mut stack) && stack == vec![None]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_9_3_4_n_n_1_n_n_2_n_6_n_n_is_valid() {
        let preorder = "9,3,4,#,#,1,#,#,2,#,6,#,#".to_string();
        assert!(Solution::is_valid_serialization(preorder));
        // My thoughts:
        // 9 (9)
        // 3 (9,3)
        // 4 (9,3,4)
        // # (9,3,4,#)
        // # (9,3,4,#,#) -> (9,3,#)
        // 1 (9,3,#,1)
        // # (9,3,#,1,#)
        // # (9,3,#,1,#,#) -> (9,3,#,#) -> (9,#)
        // 2 (9,#,2)
        // # (9,#,2,#)
        // 6 (9,#,2,#,6)
        // # (9,#,2,#,6,#)
        // # (9,#,2,#,6,#,#) -> (9,#,2,#,#) -> (9,#,#) -> ()
    }
    #[test]
    fn p_1_n_is_not_valid() {
        assert!(!Solution::is_valid_serialization("1,#".to_string()));
    }
    #[test]
    fn p_9_n_n_1_is_not_valid() {
        assert!(!Solution::is_valid_serialization("9,#,#,1".to_string()));
    }

    #[test]
    fn p_n_is_valid() {
        assert!(Solution::is_valid_serialization("#".to_string()));
    }
    #[test]
    fn p_100_n_n_is_valid() {
        assert!(Solution::is_valid_serialization("100,#,#".to_string()));
    }
}
