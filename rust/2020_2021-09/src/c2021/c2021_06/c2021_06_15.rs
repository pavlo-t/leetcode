#![allow(dead_code)]
/// Matchsticks to Square
/// =====================
///
/// You are given an integer array `matchsticks` where `matchsticks[i]` is the length of the `i`th matchstick.
/// You want to use __all the matchsticks__ to make one square.
/// You __should not break__ any stick, but you can link them up,
/// and each matchstick must be used __exactly one time__.
///
/// Return `true` if you can make this square and `false` otherwise.
///
/// __Constraints:__
///
/// - `1 <= matchsticks.length <= 15`
/// - `0 <= matchsticks[i] <= 10^9`
///
/// https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/605/week-3-june-15th-june-21st/3780/
struct Solution;
impl Solution {
    /// Approach 2: Dynamic Programming
    ///
    /// https://leetcode.com/problems/matchsticks-to-square/solution/
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        if matchsticks.len() < 4 {
            false
        } else {
            let perimeter: i64 = matchsticks.iter().map(|&i| i as i64).sum();
            if perimeter % 4 != 0 {
                false
            } else {
                fn rec(
                    available: usize,
                    mut sides_done: i32,
                    seen: &mut Vec<bool>,
                    side: i64,
                    ms: &[i64],
                ) -> bool {
                    if seen[available] {
                        false
                    } else {
                        seen[available] = true;

                        let total: i64 = (0..ms.len())
                            .filter(|&i| available & (1 << i) == 0)
                            .map(|i| ms[i])
                            .sum();

                        if total % side == 0 {
                            sides_done += 1;
                        }

                        if sides_done == 3 {
                            true
                        } else {
                            let rem = side - total % side;
                            (0..ms.len())
                                .filter(|&i| available & (1 << i) > 0)
                                .map(|i| (i, ms[i]))
                                .filter(|&(_, m)| m <= rem)
                                .any(|(i, _)| rec(available ^ (1 << i), sides_done, seen, side, ms))
                        }
                    }
                }

                let ms = matchsticks
                    .into_iter()
                    .map(|i| i as i64)
                    .collect::<Vec<_>>();

                let available = (1usize << ms.len()) - 1;
                let mut seen = vec![false; available + 1];

                rec(available, 0, &mut seen, perimeter / 4, &ms)
            }
        }
    }

    pub fn makesquare_brute_force_mask_leetcode(matchsticks: Vec<i32>) -> bool {
        if matchsticks.len() < 4 {
            false
        } else {
            let perimeter: i64 = matchsticks.iter().map(|&i| i as i64).sum();
            if perimeter % 4 != 0 {
                false
            } else {
                fn rec(available: usize, mut sides_done: i32, side: i64, ms: &[i64]) -> bool {
                    let total: i64 = (0..ms.len())
                        .filter(|&i| available & (1 << i) == 0)
                        .map(|i| ms[i])
                        .sum();

                    if total % side == 0 {
                        sides_done += 1;
                    }

                    if sides_done == 3 {
                        true
                    } else {
                        let rem = side - total % side;
                        (0..ms.len())
                            .filter(|&i| available & (1 << i) > 0)
                            .map(|i| (i, ms[i]))
                            .filter(|&(_, m)| m <= rem)
                            .any(|(i, _)| rec(available ^ (1 << i), sides_done, side, ms))
                    }
                }

                let ms = matchsticks
                    .into_iter()
                    .map(|i| i as i64)
                    .collect::<Vec<_>>();

                rec((1usize << ms.len()) - 1, 0, perimeter / 4, &ms)
            }
        }
    }

    pub fn makesquare_brute_force_optimized(matchsticks: Vec<i32>) -> bool {
        use std::cmp::Reverse;

        if matchsticks.len() < 4 {
            false
        } else {
            fn rec(ms: &[i64], side: i64, i: usize, rsf: (i64, i64, i64, i64)) -> bool {
                if i == ms.len() {
                    let (a, b, c, d) = rsf;
                    a == b && b == c && c == d
                } else {
                    let (a, b, c, d) = rsf;
                    let m = ms[i];
                    let ni = i + 1;
                    (a + m <= side && rec(ms, side, ni, (a + m, b, c, d)))
                        || (b + m <= side && rec(ms, side, ni, (a, b + m, c, d)))
                        || (c + m <= side && rec(ms, side, ni, (a, b, c + m, d)))
                        || (d + m <= side && rec(ms, side, ni, (a, b, c, d + m)))
                }
            }

            let mut ms = matchsticks
                .into_iter()
                .map(|i| i as i64)
                .collect::<Vec<_>>();
            ms.sort_unstable_by_key(|&i| Reverse(i));
            let sum: i64 = ms.iter().sum();

            sum % 4 == 0 && rec(&ms, sum / 4, 0, (0, 0, 0, 0))
        }
    }

    pub fn makesquare_brute_force_tuple(matchsticks: Vec<i32>) -> bool {
        if matchsticks.len() < 4 {
            false
        } else {
            fn rec(ms: &[i64], i: usize, rsf: (i64, i64, i64, i64)) -> bool {
                if i == ms.len() {
                    let (a, b, c, d) = rsf;
                    a == b && b == c && c == d
                } else {
                    let (a, b, c, d) = rsf;
                    let m = ms[i];
                    rec(ms, i + 1, (a + m, b, c, d))
                        || rec(ms, i + 1, (a, b + m, c, d))
                        || rec(ms, i + 1, (a, b, c + m, d))
                        || rec(ms, i + 1, (a, b, c, d + m))
                }
            }

            let ms = matchsticks
                .into_iter()
                .map(|i| i as i64)
                .collect::<Vec<_>>();

            rec(&ms, 0, (0, 0, 0, 0))
        }
    }

    pub fn makesquare_brute_force_array(matchsticks: Vec<i32>) -> bool {
        if matchsticks.len() < 4 {
            false
        } else {
            fn rec(ms: &[i64], rsf: [i64; 4]) -> bool {
                if ms.is_empty() {
                    rsf.windows(2).all(|i| i[0] == i[1])
                } else {
                    let [a, b, c, d] = rsf;
                    let m = ms[0];
                    rec(&ms[1..], [a + m, b, c, d])
                        || rec(&ms[1..], [a, b + m, c, d])
                        || rec(&ms[1..], [a, b, c + m, d])
                        || rec(&ms[1..], [a, b, c, d + m])
                }
            }

            let ms = matchsticks
                .into_iter()
                .map(|i| i as i64)
                .collect::<Vec<_>>();

            rec(&ms, [0; 4])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ms_1_1_2_2_2_produces_true() {
        let matchsticks = vec![1, 1, 2, 2, 2];
        assert!(Solution::makesquare(matchsticks));
        // Explanation:
        // You can form a square with length 2, one side of the square came two sticks with length 1.
    }
    #[test]
    fn ms_3_3_3_3_4_produces_false() {
        let matchsticks = vec![3, 3, 3, 3, 4];
        assert!(!Solution::makesquare(matchsticks));
        // Explanation: You cannot find a way to form a square with all the matchsticks.
    }

    #[test]
    fn test_134_produces_false() {
        let matchsticks = vec![
            211559, 9514615, 7412176, 5656677, 3816020, 452925, 7979371, 5025276, 8882605, 944541,
            9889007, 2344356, 7252152, 749758, 2311818,
        ];
        assert!(!Solution::makesquare(matchsticks));
    }

    #[test]
    fn ms_1_produces_false() {
        let matchsticks = vec![1];
        assert!(!Solution::makesquare(matchsticks));
    }
    #[test]
    fn ms_1_1_1_1_produces_true() {
        let matchsticks = vec![1, 1, 1, 1];
        assert!(Solution::makesquare(matchsticks));
    }
    #[test]
    fn ms_15x10pow9_produces_false() {
        // let mut str = "[1000000000".to_string();
        // for _ in 0..14 {
        //     str.push_str(",1000000000")
        // }
        // str.push(']');
        // println!("{}", str);

        let matchsticks = vec![1_000_000_000; 15];
        assert!(!Solution::makesquare(matchsticks));
    }
}
