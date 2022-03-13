#![allow(dead_code)]
/// 247. Strobogrammatic Number II
/// ==============================
///
/// Given an integer `n`, return all the __strobogrammatic numbers__ that are of length `n`.
/// You may return the answer in __any order__.
///
/// A __strobogrammatic number__ is a number that looks the same when rotated `180` degrees (looked at upside down).
///
/// __Constraints:__
///
/// - `1 <= n <= 14`
///
/// https://leetcode.com/problems/strobogrammatic-number-ii/
struct Solution;
impl Solution {
    pub fn find_strobogrammatic(n: i32) -> Vec<String> {
        fn add_layer(first: char, inner: &str, last: char) -> String {
            let mut s = String::with_capacity(inner.len() + 2);
            s.push(first);
            s.push_str(inner);
            s.push(last);
            s
        }
        fn rec(n: i32, outer: bool) -> Vec<String> {
            match n {
                0 => vec!["".to_string()],
                1 => vec!["0".to_string(), "1".to_string(), "8".to_string()],
                _ => {
                    let inners = rec(n - 2, false);
                    let mut result = vec![];
                    if !outer {
                        for inner in inners.iter() {
                            result.push(add_layer('0', inner, '0'));
                        }
                    }
                    for (f, l) in [('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')] {
                        for inner in inners.iter() {
                            result.push(add_layer(f, inner, l));
                        }
                    }
                    result
                }
            }
        }
        rec(n, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }

    #[rustfmt::skip] #[test]
    fn n_1() { assert_eq!(Solution::find_strobogrammatic(1), vs!["0", "1", "8"]); }
    #[rustfmt::skip] #[test]
    fn n_2() { assert_eq!(Solution::find_strobogrammatic(2), vs!["11", "69", "88", "96"]); }
    #[rustfmt::skip] #[test]
    fn n_3() {
        assert_eq!(Solution::find_strobogrammatic(3), vs![
            "101", "111", "181",
            "609", "619", "689",
            "808", "818", "888",
            "906", "916", "986"]);
    }
    #[rustfmt::skip] #[test]
    fn n_4() {
        assert_eq!(Solution::find_strobogrammatic(4), vs![
            "1001","1111","1691","1881","1961",
            "6009","6119","6699","6889","6969",
            "8008","8118","8698","8888","8968",
            "9006","9116","9696","9886","9966"]);
    }
    #[rustfmt::skip] #[test]
    fn n_5() {
        assert_eq!(Solution::find_strobogrammatic(5), vs![
            "10001","10101","10801","11011","11111","11811","16091","16191","16891","18081","18181","18881","19061","19161","19861",
            "60009","60109","60809","61019","61119","61819","66099","66199","66899","68089","68189","68889","69069","69169","69869",
            "80008","80108","80808","81018","81118","81818","86098","86198","86898","88088","88188","88888","89068","89168","89868",
            "90006","90106","90806","91016","91116","91816","96096","96196","96896","98086","98186","98886","99066","99166","99866"]);
    }
    #[rustfmt::skip] #[test]
    fn n_14() {
        assert_eq!(Solution::find_strobogrammatic(14).len(), 62500);
    }
}
