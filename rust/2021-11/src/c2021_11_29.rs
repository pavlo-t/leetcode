#![allow(dead_code)]
/// 721. Accounts Merge
/// ===================
///
/// Given a list of `accounts` where each element `accounts[i]` is a list of strings,
/// where the first element `accounts[i][0]` is a name,
/// and the rest of the elements are __emails__ representing emails of the account.
///
/// Now, we would like to merge these accounts.
/// Two accounts definitely belong to the same person if there is some common email to both accounts.
/// Note that even if two accounts have the same name,
/// they may belong to different people as people could have the same name.
/// A person can have any number of accounts initially,
/// but all of their accounts definitely have the same name.
///
/// After merging the accounts,
/// return the accounts in the following format: the first element of each account is the name,
/// and the rest of the elements are emails __in sorted order__.
/// The accounts themselves can be returned in __any order__.
///
/// __Constraints:__
///
/// - `1 <= accounts.length <= 1000`
/// - `2 <= accounts[i].length <= 10`
/// - `1 <= accounts[i][j] <= 30`
/// - `accounts[i][0]` consists of English letters.
/// - `accounts[i][j]` `(for j > 0)` is a valid email.
///
/// https://leetcode.com/problems/accounts-merge/
struct Solution;
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        println!("accounts_merge({:?})", accounts);
        use std::collections::{HashMap, HashSet};

        let mut merged: HashMap<String, Vec<HashSet<String>>> = HashMap::new();
        for account in accounts {
            println!("      merge:   {:?})", account);
            let mut a_iter = account.into_iter();
            let name = a_iter.next().unwrap();
            let emails = a_iter.collect::<HashSet<_>>();

            let entries = merged.entry(name).or_default();
            let mut matches: Vec<usize> = vec![];
            for (i, entry) in entries.iter_mut().enumerate() {
                if emails.iter().any(|email| entry.contains(email)) {
                    matches.push(i);
                }
            }
            if matches.is_empty() {
                entries.push(emails);
            } else {
                let mut matches = matches.into_iter();
                let insert_at = matches.next().unwrap();
                for email in emails {
                    entries[insert_at].insert(email);
                }
                for take_from in matches {
                    entries.push(HashSet::new());
                    let emails = entries.swap_remove(take_from);
                    for email in emails {
                        entries[insert_at].insert(email);
                    }
                }
                let mut i = 0;
                while i < entries.len() {
                    if entries[i].is_empty() {
                        entries.swap_remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
            println!("       merged: {:?}", merged);
        }
        println!("       merged: {:?}", merged);

        let mut results = vec![];
        for (name, entries) in merged {
            for emails in entries {
                let mut result = vec![name.clone()];
                let mut emails: Vec<String> = emails.into_iter().collect();
                emails.sort_unstable();
                result.append(&mut emails);
                results.push(result);
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($x:expr),*) => {vec![$($x.to_string()),*]};}
    macro_rules! vvs {($($x:tt),*) => {vec![$(vs!$x),*]};}

    fn check(a: Vec<Vec<String>>, mut e: Vec<Vec<String>>) {
        let mut r = Solution::accounts_merge(a);
        r.sort_unstable();
        e.sort_unstable();
        assert_eq!(r, e);
    }

    #[test]
    fn example1() {
        let a = vvs![
            ["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            ["John", "johnsmith@mail.com", "john00@mail.com"],
            ["Mary", "mary@mail.com"],
            ["John", "johnnybravo@mail.com"]
        ];
        let e = vvs![
            [
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com"
            ],
            ["Mary", "mary@mail.com"],
            ["John", "johnnybravo@mail.com"]
        ];
        //assert_eq!(Solution::accounts_merge(a), e);
        check(a, e);
        // Explanation:
        // The first and second John's are the same person as they have the common email "johnsmith@mail.com".
        // The third John and Mary are different people as none of their email addresses are used by other accounts.
        // We could return these lists in any order, for example the answer
        // [
        //   ['Mary', 'mary@mail.com'],
        //   ['John', 'johnnybravo@mail.com'],
        //   ['John', 'john00@mail.com', 'john_newyork@mail.com', 'johnsmith@mail.com']
        // ] would still be accepted.
    }
    #[test]
    fn example2() {
        let a = vvs![
            ["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
            ["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
            ["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
            ["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
            ["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"]
        ];
        let e = vvs![
            ["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
            ["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
            ["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
            ["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
            ["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"]
        ];
        //assert_eq!(Solution::accounts_merge(a), e);
        check(a, e);
    }

    #[test]
    fn test_17() {
        let a = vvs![
            [
                "Gabe",
                "Gabe5@m.co",
                "Gabe45@m.co",
                "Gabe46@m.co",
                "Gabe47@m.co",
                "Gabe48@m.co",
                "Gabe49@m.co",
                "Gabe50@m.co",
                "Gabe51@m.co",
                "Gabe52@m.co"
            ],
            [
                "Gabe",
                "Gabe3@m.co",
                "Gabe27@m.co",
                "Gabe28@m.co",
                "Gabe29@m.co",
                "Gabe30@m.co",
                "Gabe31@m.co",
                "Gabe32@m.co",
                "Gabe33@m.co",
                "Gabe34@m.co"
            ],
            [
                "Gabe",
                "Gabe6@m.co",
                "Gabe54@m.co",
                "Gabe55@m.co",
                "Gabe56@m.co",
                "Gabe57@m.co",
                "Gabe58@m.co",
                "Gabe59@m.co",
                "Gabe60@m.co",
                "Gabe61@m.co"
            ],
            [
                "Gabe",
                "Gabe0@m.co",
                "Gabe0@m.co",
                "Gabe1@m.co",
                "Gabe2@m.co",
                "Gabe3@m.co",
                "Gabe4@m.co",
                "Gabe5@m.co",
                "Gabe6@m.co",
                "Gabe7@m.co"
            ],
            [
                "Gabe",
                "Gabe1@m.co",
                "Gabe9@m.co",
                "Gabe10@m.co",
                "Gabe11@m.co",
                "Gabe12@m.co",
                "Gabe13@m.co",
                "Gabe14@m.co",
                "Gabe15@m.co",
                "Gabe16@m.co"
            ],
            [
                "Gabe",
                "Gabe2@m.co",
                "Gabe18@m.co",
                "Gabe19@m.co",
                "Gabe20@m.co",
                "Gabe21@m.co",
                "Gabe22@m.co",
                "Gabe23@m.co",
                "Gabe24@m.co",
                "Gabe25@m.co"
            ],
            [
                "Gabe",
                "Gabe4@m.co",
                "Gabe36@m.co",
                "Gabe37@m.co",
                "Gabe38@m.co",
                "Gabe39@m.co",
                "Gabe40@m.co",
                "Gabe41@m.co",
                "Gabe42@m.co",
                "Gabe43@m.co"
            ],
            [
                "Gabe",
                "Gabe0@m.co",
                "Gabe0@m.co",
                "Gabe1@m.co",
                "Gabe2@m.co",
                "Gabe3@m.co",
                "Gabe4@m.co",
                "Gabe5@m.co",
                "Gabe6@m.co",
                "Gabe7@m.co"
            ]
        ];
        let e = vvs![[
            "Gabe",
            "Gabe0@m.co",
            "Gabe10@m.co",
            "Gabe11@m.co",
            "Gabe12@m.co",
            "Gabe13@m.co",
            "Gabe14@m.co",
            "Gabe15@m.co",
            "Gabe16@m.co",
            "Gabe18@m.co",
            "Gabe19@m.co",
            "Gabe1@m.co",
            "Gabe20@m.co",
            "Gabe21@m.co",
            "Gabe22@m.co",
            "Gabe23@m.co",
            "Gabe24@m.co",
            "Gabe25@m.co",
            "Gabe27@m.co",
            "Gabe28@m.co",
            "Gabe29@m.co",
            "Gabe2@m.co",
            "Gabe30@m.co",
            "Gabe31@m.co",
            "Gabe32@m.co",
            "Gabe33@m.co",
            "Gabe34@m.co",
            "Gabe36@m.co",
            "Gabe37@m.co",
            "Gabe38@m.co",
            "Gabe39@m.co",
            "Gabe3@m.co",
            "Gabe40@m.co",
            "Gabe41@m.co",
            "Gabe42@m.co",
            "Gabe43@m.co",
            "Gabe45@m.co",
            "Gabe46@m.co",
            "Gabe47@m.co",
            "Gabe48@m.co",
            "Gabe49@m.co",
            "Gabe4@m.co",
            "Gabe50@m.co",
            "Gabe51@m.co",
            "Gabe52@m.co",
            "Gabe54@m.co",
            "Gabe55@m.co",
            "Gabe56@m.co",
            "Gabe57@m.co",
            "Gabe58@m.co",
            "Gabe59@m.co",
            "Gabe5@m.co",
            "Gabe60@m.co",
            "Gabe61@m.co",
            "Gabe6@m.co",
            "Gabe7@m.co",
            "Gabe9@m.co"
        ]];
        //assert_eq!(Solution::accounts_merge(a), e);
        check(a, e);
    }
    #[test]
    fn test_26() {
        let a = vvs![
            ["David", "David0@m.co", "David1@m.co"],
            ["David", "David3@m.co", "David4@m.co"],
            ["David", "David4@m.co", "David5@m.co"],
            ["David", "David2@m.co", "David3@m.co"],
            ["David", "David1@m.co", "David2@m.co"]
        ];
        let e = vvs![[
            "David",
            "David0@m.co",
            "David1@m.co",
            "David2@m.co",
            "David3@m.co",
            "David4@m.co",
            "David5@m.co"
        ]];
        //assert_eq!(Solution::accounts_merge(a), e);
        check(a, e);
    }
    #[test]
    fn test_38() {
        let a = vvs![
            ["Hanzo", "Hanzo2@m.co", "Hanzo3@m.co"],
            ["Hanzo", "Hanzo4@m.co", "Hanzo5@m.co"],
            ["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co"],
            ["Hanzo", "Hanzo3@m.co", "Hanzo4@m.co"],
            ["Hanzo", "Hanzo7@m.co", "Hanzo8@m.co"],
            ["Hanzo", "Hanzo1@m.co", "Hanzo2@m.co"],
            ["Hanzo", "Hanzo6@m.co", "Hanzo7@m.co"],
            ["Hanzo", "Hanzo5@m.co", "Hanzo6@m.co"]
        ];
        let e = vvs![[
            "Hanzo",
            "Hanzo0@m.co",
            "Hanzo1@m.co",
            "Hanzo2@m.co",
            "Hanzo3@m.co",
            "Hanzo4@m.co",
            "Hanzo5@m.co",
            "Hanzo6@m.co",
            "Hanzo7@m.co",
            "Hanzo8@m.co"
        ]];
        //assert_eq!(Solution::accounts_merge(a), e);
        check(a, e);
    }
    #[test]
    fn test_50() {
        let a = vvs![
            ["David", "Avid0@m.co", "David0@m.co", "David1@m.co"],
            ["David", "Gvid3@m.co", "David3@m.co", "David4@m.co"],
            ["David", "David4@m.co", "David5@m.co"],
            ["David", "David2@m.co", "David3@m.co"],
            ["David", "David1@m.co", "David2@m.co"]
        ];
        let e = vvs![[
            "David",
            "Avid0@m.co",
            "David0@m.co",
            "David1@m.co",
            "David2@m.co",
            "David3@m.co",
            "David4@m.co",
            "David5@m.co",
            "Gvid3@m.co"
        ]];
        //assert_eq!(Solution::accounts_merge(a), e);
        check(a, e);
    }
}
