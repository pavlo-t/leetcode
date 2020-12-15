#![allow(dead_code)]

struct SolutionBacktrackingDp;

impl SolutionBacktrackingDp {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = vec![];
        Self::dfs(&mut result, &s, 0, &mut Vec::new(), &mut vec![vec![false; s.len()]; s.len()]);
        result
    }

    fn dfs(result: &mut Vec<Vec<String>>,
           s: &str,
           start: usize,
           curr: &mut Vec<String>,
           dp: &mut Vec<Vec<bool>>) {
        if start >= s.len() {
            result.push(curr.clone())
        }
        for end in start..s.len() {
            if s.as_bytes()[start] == s.as_bytes()[end] && (end - start <= 2 || dp[start + 1][end - 1]) {
                dp[start][end] = true;
                curr.push(String::from(&s[start..=end]));
                Self::dfs(result, s, end + 1, curr, dp);
                curr.pop();
            }
        }
    }
}


struct SolutionBacktracking;

impl SolutionBacktracking {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();

        Self::backtrack(s, &mut Vec::new(), &mut result);

        result
    }

    fn backtrack(s: String, curr: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        if s.is_empty() {
            result.push(curr.clone())
        }

        for i in 1..=s.len() {
            let l = String::from(&s[0..i]);
            let r = String::from(&s[i..s.len()]);
            if Self::is_palindrome(&l) {
                curr.push(l);
                Self::backtrack(r, curr, result);
                curr.pop();
            }
        }
    }

    fn is_palindrome(s: &str) -> bool {
        if s.len() <= 1 {
            true
        } else {
            let bytes = s.as_bytes();
            let mut l = 0;
            let mut r = s.len() - 1;

            while l < r {
                if bytes[l] != bytes[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }

            true
        }
    }
}


struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let capacity = if s.len() < 2 { 1 } else { 1 << (s.len() - 1) };
        let mut result: Vec<Vec<String>> = Vec::with_capacity(capacity);
        result.push(Vec::with_capacity(s.len()));

        for c in s.chars() {
            let mut new_partitions = Vec::with_capacity(result.len());
            for partition in result.iter_mut() {
                let mut new_partition = partition.clone();
                if let Some(last) = new_partition.last_mut() {
                    last.push(c);
                    new_partitions.push(new_partition);
                }

                partition.push(c.to_string());
            }
            result.extend(new_partitions);
        }

        result
            .into_iter()
            .filter(|ss| ss.iter().all(|s| Self::is_palindrome(s)))
            .collect()
    }

    fn is_palindrome(s: &str) -> bool {
        if s.len() <= 1 { true } else {
            let mut chars = s.chars();
            let mut chars_rev = s.chars().rev();
            let mut count = s.len() / 2;

            while count > 0 {
                if chars.next() != chars_rev.next() {
                    return false;
                }
                count -= 1;
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_aab() {
        let expected = vec![vec!["a", "a", "b"], vec!["aa", "b"]];
        assert_eq!(Solution::partition("aab".to_string()), expected);
    }

    #[test]
    fn example2_a() {
        assert_eq!(Solution::partition("a".to_string()), vec![vec!["a"]]);
    }

    #[test]
    fn test_aaa() {
        let expected = vec![vec!["a", "a", "a"], vec!["aa", "a"], vec!["a", "aa"], vec!["aaa"]];
        // let expected = vec![vec!["a", "a", "a"], vec!["a", "aa"], vec!["aa", "a"], vec!["aaa"]];
        assert_eq!(Solution::partition("aaa".to_string()), expected);
    }

    #[test]
    fn test_16_a() {
        assert_eq!(Solution::partition("aaaaaaaaaaaaaaaa".to_string()).len(), 32768);
    }
}
