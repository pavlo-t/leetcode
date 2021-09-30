#![allow(dead_code)]
/// Generate Parentheses
/// ====================
///
/// Given `n` pairs of parentheses,
/// write a function to _generate all combinations of well-formed parentheses_.
///
/// __Constraints:__
///
/// - `1 <= n <= 8`
///
/// https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/605/week-3-june-15th-june-21st/3781/
struct Solution;
impl Solution {
    /// https://www.geeksforgeeks.org/print-all-combinations-of-balanced-parentheses/
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn bts(n: i32, o: i32, c: i32, s: &mut String, r: &mut Vec<String>) {
            if c == n {
                r.push(s.clone());
            } else {
                if o < n {
                    s.push('(');
                    bts(n, o + 1, c, s, r);
                    s.pop();
                }
                if o > c {
                    s.push(')');
                    bts(n, o, c + 1, s, r);
                    s.pop();
                }
            }
        }

        let mut result = vec![];
        bts(n, 0, 0, &mut String::new(), &mut result);
        result
    }

    pub fn generate_parenthesis_my3_hash_set_wrong(n: i32) -> Vec<String> {
        use std::collections::HashSet;

        if n == 0 {
            vec![]
        } else if n == 1 {
            vec!["()".to_string()]
        } else {
            let mut result = HashSet::new();
            let next = Self::generate_parenthesis(n - 1);

            for n in next.iter() {
                let mut s = String::from("(");
                s.push_str(n);
                s.push(')');
                result.insert(s);
            }
            match n {
                4 => {
                    result.insert(String::from("(())(())"));
                }
                5 => {
                    result.insert(String::from("((()))(())"));
                    result.insert(String::from("(()())(())"));
                    result.insert(String::from("(())((()))"));
                    result.insert(String::from("(())(()())"));
                    result.insert(String::from("(())()(())"));
                }
                6 => {
                    result.insert(String::from("(((())))(())"));
                    result.insert(String::from("((()))((()))"));
                    result.insert(String::from("(())(((())))"));
                }
                _ => (),
            };
            for n in next.iter() {
                let mut s = String::new();
                s.push_str(n);
                s.push_str("()");
                result.insert(s);
            }
            for n in next.iter() {
                let mut s = String::from("()");
                s.push_str(n);
                result.insert(s);
            }

            result.into_iter().collect()
        }
    }
    pub fn generate_parenthesis_my2_hash_set_wrong(n: i32) -> Vec<String> {
        use std::collections::HashSet;

        if n == 1 {
            vec!["()".to_string()]
        } else {
            let mut result = HashSet::new();
            let next = Self::generate_parenthesis(n - 1);

            for n in next.iter() {
                let mut s = String::from("(");
                s.push_str(n);
                s.push(')');
                result.insert(s);
            }
            for n in next.iter() {
                let mut s = String::new();
                s.push_str(n);
                s.push_str("()");
                result.insert(s);
            }
            for n in next.iter() {
                let mut s = String::from("()");
                s.push_str(n);
                result.insert(s);
            }

            result.into_iter().collect()
        }
    }
    pub fn generate_parenthesis_v1_wrong(n: i32) -> Vec<String> {
        if n == 1 {
            vec!["()".to_string()]
        } else {
            let next = Self::generate_parenthesis(n - 1);
            let mut result = vec![];
            for n in next.iter().skip(1) {
                let mut s = String::from("()");
                s.push_str(n);
                result.push(s);
            }
            for n in next.iter() {
                let mut s = String::new();
                s.push_str(n);
                s.push_str("()");
                result.push(s);
            }
            for n in next.iter() {
                let mut s = String::from("(");
                s.push_str(n);
                s.push(')');
                result.push(s);
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}

    #[test]
    fn n1_produces_1_comb() {
        assert_eq!(Solution::generate_parenthesis(1), ["()"]);
    }
    #[test]
    fn n2_produces_2_combs() {
        let e = ["(())", "()()"];
        assert_eq!(Solution::generate_parenthesis(2), e);
        // let e = ["(())", "()()"]
        //     .iter()
        //     .map(|s| s.to_string())
        //     .collect::<HashSet<_>>();
        // let r = Solution::generate_parenthesis(2);
        // assert_eq!(r.len(), e.len());
        // for s in &r {
        //     assert!(e.contains(s));
        // }
    }
    #[test]
    fn n3_produces_5_combs() {
        let e = ["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(Solution::generate_parenthesis(3), e);
        // let e = ["((()))", "(()())", "(())()", "()(())", "()()()"]
        //     .iter()
        //     .map(|s| s.to_string())
        //     .collect::<HashSet<_>>();
        // let r = Solution::generate_parenthesis(3);
        // assert_eq!(r.len(), e.len());
        // for s in &r {
        //     assert!(e.contains(s));
        // }
    }
    #[test]
    fn n4_produces_14_combs() {
        // let mut my = vs![
        //     "()()()()", "()(())()", "()(()())", "()((()))", "()(())()", "()()()()", "(())()()",
        //     "(()())()", "((()))()", "(()(()))", "(()()())", "((())())", "((()()))", "(((())))"
        // ];
        // let mut lc = vs![
        //     "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
        //     "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
        // ];
        //
        // my.sort_unstable();
        // lc.sort_unstable();
        //
        // assert_eq!(my, lc);

        // let e = [
        //     "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
        //     "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
        // ];
        // assert_eq!(Solution::generate_parenthesis(4), e);
        let e = [
            "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
            "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();
        let r = Solution::generate_parenthesis(4);
        for s in e.iter() {
            assert!(r.contains(s), "didn't contain: {}", s)
        }
        assert_eq!(r.len(), e.len());
    }
    #[test]
    fn n5_produces_42_combs() {
        let e = [
            "((((()))))",
            "(((()())))",
            "(((())()))",
            "(((()))())",
            "(((())))()",
            "((()(())))",
            "((()()()))",
            "((()())())",
            "((()()))()",
            "((())(()))",
            "((())()())",
            "((())())()",
            "((()))(())",
            "((()))()()",
            "(()((())))",
            "(()(()()))",
            "(()(())())",
            "(()(()))()",
            "(()()(()))",
            "(()()()())",
            "(()()())()",
            "(()())(())",
            "(()())()()",
            "(())((()))",
            "(())(()())",
            "(())(())()",
            "(())()(())",
            "(())()()()",
            "()(((())))",
            "()((()()))",
            "()((())())",
            "()((()))()",
            "()(()(()))",
            "()(()()())",
            "()(()())()",
            "()(())(())",
            "()(())()()",
            "()()((()))",
            "()()(()())",
            "()()(())()",
            "()()()(())",
            "()()()()()",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();
        let r = Solution::generate_parenthesis(5);
        for s in e.iter() {
            assert!(r.contains(s), "didn't contain: {}", s)
        }
        assert_eq!(r.len(), e.len());
    }
    #[test]
    fn n8_produces_1430_combs() {
        let r = Solution::generate_parenthesis(8);
        assert_eq!(r.len(), 1430);
        let r = r.into_iter().collect::<HashSet<_>>();
        assert_eq!(r.len(), 1430);
    }
}
