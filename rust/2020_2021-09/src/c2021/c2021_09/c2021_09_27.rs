#![allow(dead_code)]
/// Unique Email Addresses
/// ======================
///
/// Every __valid email__ consists of a __local name__ and a __domain name__, separated by the `'@'` sign.
/// Besides lowercase letters, the email may contain one or more `'.'` or `'+'`.
///
/// - For example, in `"alice@leetcode.com"`, `"alice"` is the __local name__,
///   and `"leetcode.com"` is the __domain name__.
///
/// If you add periods `'.'` between some characters in the __local name__ part of an email address,
/// mail sent there will be forwarded to the same address without dots in the local name.
/// Note that this rule __does not apply__ to __domain names__.
///
/// - For example, `"alice.z@leetcode.com"` and `"alicez@leetcode.com"` forward to the same email address.
///
/// If you add a plus `'+'` in the __local name__, everything after the first plus sign __will be ignored__.
/// This allows certain emails to be filtered.
/// Note that this rule __does not apply__ to __domain names__.
///
/// - For example, `"m.y+name@email.com"` will be forwarded to `"my@email.com"`.
///
/// It is possible to use both of these rules at the same time.
///
/// Given an array of strings `emails` where we send one email to each `email[i]`,
/// return _the number of different addresses that actually receive mails_.
///
/// __Constraints:__
///
/// - `1 <= emails.length <= 100`
/// - `1 <= emails[i].length <= 100`
/// - `email[i]` consist of lowercase English letters, `'+'`, `'.'` and `'@'`.
/// - Each `emails[i]` contains exactly one `'@'` character.
/// - All local and domain names are non-empty.
/// - Local names do not start with a `'+'` character.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/639/week-4-september-22nd-september-28th/3989/
struct Solution;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        println!("num_unique_emails({:?}", emails);
        use std::collections::HashSet;

        fn clean(mut e: String) -> String {
            unsafe {
                let v = e.as_mut_vec();
                let mut l = 0;
                let mut r = 0;
                while v[r] != b'@' {
                    #[rustfmt::skip]
                    match v[r] {
                        b'+' => while v[r] != b'@' { r += 1},
                        b'.' => r += 1,
                        b => {
                            v[l] = b;
                            l += 1;
                            r += 1;
                        }
                    };
                }
                while r < v.len() {
                    v[l] = v[r];
                    l += 1;
                    r += 1;
                }
                v.truncate(l);
            }
            e
        }

        emails
            .into_iter()
            .map(|e| clean(e))
            .collect::<HashSet<_>>()
            .len() as i32
    }

    pub fn num_unique_emails_my1(emails: Vec<String>) -> i32 {
        println!("num_unique_emails({:?}", emails);
        use std::collections::HashSet;

        fn clean(mut e: String) -> String {
            unsafe {
                let v = e.as_mut_vec();
                let mut l = 0;
                let mut r = 0;
                let mut seen_at = false;
                while r < v.len() {
                    if seen_at {
                        v[l] = v[r];
                        l += 1;
                        r += 1;
                    } else {
                        #[rustfmt::skip]
                        match v[r] {
                            b'+' => while v[r] != b'@' { r += 1},
                            b'.' => r += 1,
                            b => {
                                if b == b'@' { seen_at = true; }
                                v[l] = b;
                                l += 1;
                                r += 1;
                            }
                        };
                    }
                }
                v.truncate(l);
            }
            e
        }
        emails
            .into_iter()
            .map(|e| clean(e))
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]}}

    #[test]
    fn example1() {
        let e = vs![
            "test.email+alex@leetcode.com",
            "test.e.mail+bob.cathy@leetcode.com",
            "testemail+david@lee.tcode.com"
        ];
        assert_eq!(Solution::num_unique_emails(e), 2);
        // Explanation: "testemail@leetcode.com" and "testemail@lee.tcode.com" actually receive mails.
    }
    #[test]
    fn example2() {
        let e = vs!["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"];
        assert_eq!(Solution::num_unique_emails(e), 3);
    }

    #[test]
    fn dot_plus() {
        let e = vs![".+@e", ".+1@e"];
        assert_eq!(Solution::num_unique_emails(e), 1);
    }
}
