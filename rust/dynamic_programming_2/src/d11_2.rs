#![allow(dead_code)]
/// 5. Longest Palindromic Substring
/// ================================
///
/// Given a string `s`, return _the longest palindromic substring in `s`_.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 1000`
/// - `s` consist of only digits and English letters.
///
/// https://leetcode.com/problems/longest-palindromic-substring/
struct Solution;
impl Solution {
    /// 23:55-00:09
    pub fn longest_palindrome_rec(s: String) -> String {
        println!("longest_palindrome({})", s);
        fn is_palindrome(len: usize, l: usize, s: &[u8]) -> bool {
            len < 2 || (s[l] == s[l + len - 1] && is_palindrome(len - 2, l + 1, s))
        }
        fn rec(len: usize, l: usize, s: &[u8]) -> &[u8] {
            if len == 0 {
                &s[0..0]
            } else if l + len > s.len() {
                rec(len - 1, 0, s)
            } else if is_palindrome(len, l, s) {
                &s[l..l + len]
            } else {
                rec(len, l + 1, s)
            }
        }
        let bs = s.as_bytes();
        let n = bs.len();
        String::from_utf8(rec(n, 0, bs).iter().map(|&b| b).collect()).unwrap()
    }
    /// 00:09-00:17
    pub fn longest_palindrome_rec_with_memo(s: String) -> String {
        println!("longest_palindrome({})", s);
        #[rustfmt::skip]
        fn is_palindrome(len: usize, l: usize, s: &[u8], memo: &mut Vec<Vec<u8>>) -> bool {
            if memo[len][l] != b'0' {
                memo[len][l] == b't'
            } else {
                memo[len][l] = if len < 2 || (s[l] == s[l + len - 1] && is_palindrome(len - 2, l + 1, s, memo)) {
                    b't'
                } else {
                    b'f'
                };
                memo[len][l] == b't'
            }
        }
        fn rec<'a>(len: usize, l: usize, s: &'a [u8], memo: &mut Vec<Vec<u8>>) -> &'a [u8] {
            if len == 0 {
                &s[0..0]
            } else if l + len > s.len() {
                rec(len - 1, 0, s, memo)
            } else if is_palindrome(len, l, s, memo) {
                &s[l..l + len]
            } else {
                rec(len, l + 1, s, memo)
            }
        }
        let bs = s.as_bytes();
        let n = bs.len();
        let mut memo = vec![vec![b'0'; n + 1]; n + 1];
        String::from_utf8(rec(n, 0, bs, &mut memo).iter().map(|&b| b).collect()).unwrap()
    }
    /// 00:17-00:25
    pub fn longest_palindrome_dp_vec_vec(s: String) -> String {
        println!("longest_palindrome({})", s);
        let bs = s.as_bytes();
        let n = bs.len();
        let mut dp = vec![vec![true; n + 1]; n + 1];
        let mut best = (1, 0);
        for len in 2..=n {
            for l in 0..=n - len {
                if bs[l] == bs[l + len - 1] && dp[len - 2][l + 1] {
                    if best.0 < len {
                        best = (len, l);
                    }
                } else {
                    dp[len][l] = false;
                }
            }
        }
        let (len, l) = best;
        s[l..l + len].into()
    }
    /// 00:25-00:50
    pub fn longest_palindrome_dp_vec(s: String) -> String {
        println!("longest_palindrome({})", s);
        let bs = s.as_bytes();
        let n = bs.len();
        let mut antepenultimate = vec![true; n];
        let mut penultimate = vec![true; n];
        let mut best = (1, 0);
        for len in 2..=n {
            for l in 0..=n - len {
                if bs[l] == bs[l + len - 1] && antepenultimate[l + 1] {
                    if best.0 < len {
                        best = (len, l);
                    }
                    antepenultimate[l] = true;
                } else {
                    antepenultimate[l] = false;
                }
            }
            std::mem::swap(&mut antepenultimate, &mut penultimate);
        }
        let (len, l) = best;
        s[l..l + len].into()
    }
    /// Approach 5: Manacher's Algorithm
    /// https://leetcode.com/problems/longest-palindromic-substring/solution/
    /// https://en.wikipedia.org/wiki/Longest_palindromic_substring#Manacher's_algorithm
    pub fn longest_palindrome(s: String) -> String {
        println!("longest_palindrome({})", s);
        let n = s.len();
        // string with a bogus character (eg. '|') inserted between each character (including outer boundaries)
        // note: length(S') = length(PalindromeRadii) = 2 × length(S) + 1
        const BC: char = '|';
        let mut ss = String::with_capacity(n * 2 + 1);
        ss.push(BC);
        s.chars().for_each(|c| {
            ss.push(c);
            ss.push(BC);
        });
        let bs = ss.as_bytes();

        // The radius of the longest palindrome centered on each place in S'
        let mut palindrome_radii = vec![0; n * 2 + 1];

        let mut center = 0;
        let mut radius = 0;
        while center < bs.len() {
            // At the start of the loop, Radius is already set to a lower-bound for the longest radius.
            // In the first iteration, Radius is 0, but it can be higher.

            // Determine the longest palindrome starting at Center-Radius and going to Center+Radius
            while center.wrapping_sub(radius + 1) < bs.len()
                && center + (radius + 1) < bs.len()
                && bs[center - (radius + 1)] == bs[center + (radius + 1)]
            {
                radius += 1;
            }

            // Save the radius of the longest palindrome in the array
            palindrome_radii[center as usize] = radius;

            // Below, Center is incremented.
            // If any precomputed values can be reused, they are.
            // Also, Radius may be set to a value greater than 0

            let old_center = center;
            let old_radius = radius;
            center += 1;
            // Radius' default value will be 0, if we reach the end of the following loop.
            radius = 0;
            while center <= old_center + old_radius {
                // Because Center lies inside the old palindrome and every character inside
                // a palindrome has a "mirrored" character reflected across its center, we
                // can use the data that was precomputed for the Center's mirrored point.
                let mirrored_center = old_center - (center - old_center);
                let max_mirrored_radius = old_center + old_radius - center;
                if palindrome_radii[mirrored_center] < max_mirrored_radius {
                    palindrome_radii[center] = palindrome_radii[mirrored_center];
                    center += 1;
                } else if palindrome_radii[mirrored_center] > max_mirrored_radius {
                    palindrome_radii[center] = max_mirrored_radius;
                    center += 1;
                } else {
                    // PalindromeRadii[MirroredCenter] = MaxMirroredRadius
                    radius = max_mirrored_radius;
                    break;
                }
            }
        }
        let (r, c) = palindrome_radii
            .iter()
            .enumerate()
            .map(|(i, &r)| (r, i))
            .max()
            .unwrap();
        let mut result = String::with_capacity(r);
        ss[c - r..c + r]
            .chars()
            .filter(|&c| c != BC)
            .for_each(|c| result.push(c));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: &str, es: Vec<&str>) {
        let r = Solution::longest_palindrome(s.into());
        assert!(es.iter().any(|e| e == &r), "\"{}\" != any of {:?}", r, es);
    }

    #[rustfmt::skip] #[test] fn a () { check("a" , vec!["a"]); }
    #[rustfmt::skip] #[test] fn aa() { check("aa", vec!["aa"]); }
    #[rustfmt::skip] #[test] fn babad() { check("babad", vec!["bab", "aba"]); }
    #[rustfmt::skip] #[test] fn cbbd() { check("cbbd", vec!["bb"]); }
    #[rustfmt::skip] #[test] fn ac() { check("ac", vec!["a", "c"]); }

    #[test]
    fn test_121_() {
        check("xaabacxcabaaxcabaax", vec!["xaabacxcabaax"]);
    }

    #[test]
    fn a_repeat_1000() {
        let s = "a".repeat(1000);
        check(&s, vec![&s]);
    }
    /// If getting stack overflow: add RUST_MIN_STACK=134217728 (2 ** 27) to env:
    /// RUST_MIN_STACK=134217728 cargo test --lib d11_2
    #[test]
    fn a_to_y_repeat_40() {
        let s = "abcdefghijklmnopqrstuvwxy".repeat(40);
        let es = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y",
        ];
        check(&s, es);
    }
}
