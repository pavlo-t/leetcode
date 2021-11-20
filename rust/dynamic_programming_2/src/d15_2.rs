#![allow(dead_code)]
/// 254. Factor Combinations
/// ========================
///
/// Numbers can be regarded as the product of their factors.
///
/// For example, `8 = 2 x 2 x 2 = 2 x 4`.
///
/// Given an integer `n`, return _all possible combinations of its factors_.
/// You may return the answer in __any order__.
///
/// __Note__ that the factors should be in the range `[2, n - 1]`.
///
/// __Constraints:__
///
/// - `1 <= n <= 10_000_000`
///
/// https://leetcode.com/problems/factor-combinations/
struct Solution;
impl Solution {
    pub fn get_factors_rec(n: i32) -> Vec<Vec<i32>> {
        println!("get_factors({})", n);
        use std::collections::HashSet;

        fn rec(n: i32) -> HashSet<Vec<i32>> {
            if n < 2 {
                HashSet::new()
            } else {
                let mut results = HashSet::new();
                results.insert(vec![n]);
                for d in 2..=n / 2 {
                    if n % d == 0 {
                        rec(n / d).into_iter().for_each(|mut v| {
                            v.push(d);
                            v.sort_unstable();
                            results.insert(v);
                        });
                    }
                }
                results
            }
        }
        let hs = rec(n);
        hs.into_iter().filter(|v| v.len() > 1).collect::<Vec<_>>()
    }

    pub fn get_factors_my_rec_with_memo_slow_but_passed(n: i32) -> Vec<Vec<i32>> {
        println!("get_factors({})", n);
        use std::collections::{HashMap, HashSet};

        fn rec(n: i32, memo: &mut HashMap<i32, HashSet<Vec<i32>>>) -> &HashSet<Vec<i32>> {
            if memo.contains_key(&n) {
                memo.get(&n).unwrap()
            } else {
                let result = if n < 2 {
                    HashSet::new()
                } else {
                    let mut results = HashSet::new();
                    results.insert(vec![n]);
                    for d in 2..=n / 2 {
                        if n % d == 0 {
                            let result = rec(n / d, memo);
                            result.into_iter().for_each(|v| {
                                let mut v = v.clone();
                                v.push(d);
                                v.sort_unstable();
                                results.insert(v);
                            });
                        }
                    }
                    results
                };
                memo.insert(n, result);
                memo.get(&n).unwrap()
            }
        }
        let mut memo: HashMap<i32, HashSet<Vec<i32>>> = HashMap::new();
        let hs = rec(n, &mut memo);
        hs.iter()
            .filter(|v| v.len() > 1)
            .map(|v| v.to_owned())
            .collect::<Vec<_>>()
    }

    /// From other submissions
    /// https://leetcode.com/submissions/detail/589764127/
    pub fn get_factors(n: i32) -> Vec<Vec<i32>> {
        println!("get_factors({})", n);
        fn factors(n: i32, result: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>) {
            let l = if cur.is_empty() {
                2
            } else {
                cur.push(n);
                result.push(cur.clone());
                cur.pop();
                *cur.last().unwrap()
            };

            for i in l..=n {
                if i * i > n {
                    break;
                }

                if n % i == 0 {
                    cur.push(i);
                    factors(n / i, result, cur);
                    cur.pop();
                }
            }
        }

        let mut result = vec![];
        factors(n, &mut result, &mut vec![]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn n1() {
        let e: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::get_factors(1), e);
    }
    #[test]
    fn n12() {
        let e = vv![[2, 2, 3], [2, 6], [3, 4]];
        let mut r = Solution::get_factors(12);
        r.sort_unstable();
        assert_eq!(r, e);
    }
    #[test]
    fn n37() {
        let e: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::get_factors(37), e);
    }
    #[test]
    fn n32() {
        #[rustfmt::skip]
        let e = vv![[2, 2, 2, 2, 2], [2, 2, 2, 4], [2, 2, 8], [2, 4, 4], [2, 16], [4, 8]];
        let mut r = Solution::get_factors(32);
        r.sort_unstable();
        assert_eq!(r, e);
    }

    #[test]
    fn n1_000_000() {
        let r = Solution::get_factors(1_000_000);
        assert_eq!(r.len(), 1042);
    }
    #[test]
    fn n10_000_000() {
        let r = Solution::get_factors(10_000_000);
        assert_eq!(r.len(), 2997);
    }
}
