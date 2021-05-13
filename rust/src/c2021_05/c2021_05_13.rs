#![allow(dead_code)]
/// Ambiguous Coordinates
/// =====================
///
/// We had some 2-dimensional coordinates, like `"(1, 3)"` or `"(2, 0.5)"`.
/// Then, we removed all commas, decimal points, and spaces, and ended up with the string `s`.
/// Return a list of strings representing all possibilities for what our original coordinates could have been.
///
/// Our original representation never had extraneous zeroes,
/// so we never started with numbers like "00", "0.0", "0.00", "1.0", "001", "00.01",
/// or any other number that can be represented with less digits.
/// Also, a decimal point within a number never occurs without at least one digit occurring before it,
/// so we never started with numbers like ".1".
///
/// The final answer list can be returned in any order.
/// Also note that all coordinates in the final answer have exactly one space between them
/// (occurring after the comma.)
///
/// __Note:__
///
/// - `4 <= s.length <= 12`.
/// - `s[0]` = "(", `s[s.length - 1]` = ")", and the other elements in `s` are digits.
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3741/
struct Solution;
impl Solution {
    pub fn ambiguous_coordinates_3(s: String) -> Vec<String> {
        fn is_valid((l, r): &(&str, &str)) -> bool {
            (l.len() == 1 || !l.starts_with('0')) && (r.len() == 0 || !r.ends_with('0'))
        }
        fn set_dot((l, r): (&str, &str)) -> String {
            if r.len() == 0 {
                l.to_string()
            } else {
                format!("{}.{}", l, r)
            }
        }

        let s = &s[1..s.len() - 1];
        (1..s.len())
            .map(|i| (&s[..i], &s[i..]))
            .flat_map(|(x, y)| {
                (1..=x.len()).flat_map(move |xi| {
                    (1..=y.len())
                        .map(move |yi| ((&x[..xi], &x[xi..]), (&y[..yi], &y[yi..])))
                        .filter(|(x, y)| is_valid(x) && is_valid(y))
                        .map(|(x, y)| format!("({}, {})", set_dot(x), set_dot(y)))
                })
            })
            .collect()
    }

    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        fn is_valid((s, i): &(&str, usize)) -> bool {
            s.len() == 1 || (i == &1 || !s.starts_with('0')) && (i == &0 || !s.ends_with('0'))
        }
        fn set_dot((s, i): (&str, usize)) -> String {
            if i == 0 {
                s.to_string()
            } else {
                format!("{}.{}", &s[..i], &s[i..])
            }
        }

        let s = &s[1..s.len() - 1];
        (1..s.len())
            .map(|i| (&s[..i], &s[i..]))
            .flat_map(|(x, y)| {
                (0..x.len()).flat_map(move |xi| {
                    (0..y.len())
                        .map(move |yi| ((x, xi), (y, yi)))
                        .filter(|(x, y)| is_valid(x) && is_valid(y))
                        .map(|(x, y)| format!("({}, {})", set_dot(x), set_dot(y)))
                })
            })
            .collect()
    }

    pub fn ambiguous_coordinates_my_1(s: String) -> Vec<String> {
        fn set_dot(s: &str, i: usize) -> String {
            if i == 0 {
                s.to_string()
            } else {
                format!("{}.{}", &s[..i], &s[i..])
            }
        }

        fn is_valid(s: &str) -> bool {
            if s.contains('.') {
                match s.parse::<f64>() {
                    Err(_) => false,
                    Ok(f) => f != 0.0 && f.to_string() == s,
                }
            } else {
                match s.parse::<i32>() {
                    Err(_) => false,
                    Ok(i) => i.to_string() == s,
                }
            }
        }

        let s = &s[1..s.len() - 1];
        (1..s.len())
            .map(|i| (&s[..i], &s[i..]))
            .flat_map(|(x, y)| {
                (0..x.len()).flat_map(move |xi| {
                    (0..y.len()).map(move |yi| (set_dot(x, xi), set_dot(y, yi)))
                })
            })
            .filter(|(x, y)| is_valid(x) && is_valid(y))
            .map(|(x, y)| format!("({}, {})", x, y))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($x:tt),*) => { vec![$($x.to_string()),*] }; }

    #[test]
    fn example1_s123() {
        let s = "(123)".to_string();
        let e = vs!["(1, 23)", "(1, 2.3)", "(12, 3)", "(1.2, 3)"];
        assert_eq!(Solution::ambiguous_coordinates(s), e);
    }
    #[test]
    fn example2_s00011() {
        let s = "(00011)".to_string();
        let e = vs!["(0, 0.011)", "(0.001, 1)"];
        assert_eq!(Solution::ambiguous_coordinates(s), e);
        // Explanation:
        // 0.0, 00, 0001 or 00.01 are not allowed.
    }
    #[test]
    fn example3_s0123() {
        let s = "(0123)".to_string();
        let e = vs![
            "(0, 123)",
            "(0, 1.23)",
            "(0, 12.3)",
            "(0.1, 23)",
            "(0.1, 2.3)",
            "(0.12, 3)"
        ];
        assert_eq!(Solution::ambiguous_coordinates(s), e);
    }
    #[test]
    fn example4_s100() {
        let s = "(100)".to_string();
        let e = vs!["(10, 0)"];
        assert_eq!(Solution::ambiguous_coordinates(s), e);
        // Explanation:
        // 1.0 is not allowed.
    }

    mod performance {
        use super::*;
        #[test]
        fn s1111111111() {
            let s = "(1111111111)".to_string();
            // let e = vs!["(10, 0)"];
            let r = Solution::ambiguous_coordinates(s);
            assert_eq!(r.len(), 165);
        }
        #[test]
        fn s1234567890() {
            let s = "(1234567890)".to_string();
            // let e = vs!["(10, 0)"];
            let r = Solution::ambiguous_coordinates(s);
            assert_eq!(r.len(), 45);
        }
    }
}
