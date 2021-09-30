#![allow(dead_code)]
/// Find the Shortest Superstring
/// =============================
///
/// Given an array of strings `words`,
/// return _the smallest string that contains each string in `words` as a substring_.
/// If there are multiple valid strings of the smallest length, return __any of them__.
///
/// You may assume that no string in `words` is a substring of another string in words.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 12`
/// - `1 <= words[i].length <= 20`
/// - `words[i]` consists of lowercase English letters.
/// - All the strings of `words` are __unique__.
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/601/week-4-may-22nd-may-28th/3753/
struct Solution;
impl Solution {
    /// https://www.geeksforgeeks.org/shortest-superstring-problem/
    pub fn shortest_superstring(words: Vec<String>) -> String {
        fn calc(a: &str, b: &str) -> usize {
            for i in 1..a.len() {
                if b.starts_with(&a[i..]) {
                    return b.len() + i - a.len();
                }
            }
            b.len()
        }

        let n = words.len();
        let mut graph = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                graph[i][j] = calc(&words[i], &words[j]);
                graph[j][i] = calc(&words[j], &words[i]);
            }
        }

        let mut dp = vec![vec![usize::MAX; n]; 1 << n];
        let mut path = vec![vec![0; n]; 1 << n];

        let mut last = -1;
        let mut min = usize::MAX;

        for i in 1..(1 << n) {
            for j in 0..n {
                if (i & (1 << j)) > 0 {
                    let prev = i - (1 << j);

                    if prev == 0 {
                        dp[i][j] = words[j].len();
                    } else {
                        for k in 0..n {
                            if dp[prev][k] < usize::MAX && dp[prev][k] + graph[k][j] < dp[i][j] {
                                dp[i][j] = dp[prev][k] + graph[k][j];
                                path[i][j] = k;
                            }
                        }
                    }
                }
                if i == (1 << n) - 1 && dp[i][j] < min {
                    min = dp[i][j];
                    last = j as i32;
                }
            }
        }

        let mut r = String::new();
        let mut cur = (1 << n) - 1;
        let mut stack = vec![];

        while cur > 0 {
            stack.push(last);
            let temp = cur;
            cur -= 1 << last;
            last = path[temp as usize][last as usize] as i32;
        }

        let mut i = stack.pop().unwrap() as usize;
        r.push_str(&words[i]);

        while let Some(j) = stack.pop() {
            let j = j as usize;
            r.push_str(&words[j][words[j].len() - graph[i][j]..]);
            i = j;
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($s:tt),*) => { vec![$($s.to_string()),*] }; }

    #[test]
    fn example1() {
        let words = vs!["alex", "loves", "leetcode"];
        // assert_eq!(Solution::shortest_superstring(words), "alexlovesleetcode");
        assert_eq!(Solution::shortest_superstring(words), "leetcodelovesalex");
        // Explanation: All permutations of "alex","loves","leetcode" would also be accepted.
    }
    #[test]
    fn example2() {
        let words = vs!["catg", "ctaagt", "gcta", "ttca", "atgcatc"];
        assert_eq!(Solution::shortest_superstring(words), "gctaagttcatgcatc");
    }

    #[test]
    fn words_12x20_produces_240_chars() {
        let words = vs![
            "aaaaaaaaaaaaaaaaaaaa",
            "bbbbbbbbbbbbbbbbbbbb",
            "cccccccccccccccccccc",
            "dddddddddddddddddddd",
            "eeeeeeeeeeeeeeeeeeee",
            "ffffffffffffffffffff",
            "gggggggggggggggggggg",
            "hhhhhhhhhhhhhhhhhhhh",
            "iiiiiiiiiiiiiiiiiiii",
            "jjjjjjjjjjjjjjjjjjjj",
            "kkkkkkkkkkkkkkkkkkkk",
            "llllllllllllllllllll"
        ];
        let e = words.iter().rev().fold(String::new(), |mut acc, w| {
            acc.push_str(w);
            acc
        });
        assert_eq!(Solution::shortest_superstring(words), e);
    }
    #[test]
    fn words_12x12_produces_23_chars() {
        let words = vs![
            "abcdefghijkl",
            "bcdefghijklm",
            "cdefghijklmn",
            "defghijklmno",
            "efghijklmnop",
            "fghijklmnopq",
            "ghijklmnopqr",
            "hijklmnopqrs",
            "ijklmnopqrst",
            "jklmnopqrstu",
            "klmnopqrstuv",
            "lmnopqrstuvw"
        ];
        let e = "abcdefghijklmnopqrstuvw";
        assert_eq!(Solution::shortest_superstring(words), e);
    }
    #[test]
    fn words_12x20_produces_31_chars() {
        let words = vs![
            "abcdefghijklmnopqrst",
            "bcdefghijklmnopqrstu",
            "cdefghijklmnopqrstuv",
            "defghijklmnopqrstuvw",
            "efghijklmnopqrstuvwx",
            "fghijklmnopqrstuvwxy",
            "ghijklmnopqrstuvwxyz",
            "hijklmnopqrstuvwxyza",
            "ijklmnopqrstuvwxyzab",
            "jklmnopqrstuvwxyzabc",
            "klmnopqrstuvwxyzabcd",
            "lmnopqrstuvwxyzabcde"
        ];
        let e = "abcdefghijklmnopqrstuvwxyzabcde";
        assert_eq!(Solution::shortest_superstring(words), e);
    }
}
