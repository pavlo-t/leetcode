#![allow(dead_code)]

/// # Russian Doll Envelopes
///
/// You are given a 2D array of integers `envelopes` where `envelopes[i] = [wi, hi]`
/// represents the width and the height of an envelope.
///
/// One envelope can fit into another if and only if both the width and height of one envelope
/// is greater than the width and height of the other envelope.
///
/// Return _the maximum number of envelopes can you Russian doll (i.e., put one inside the other)_.
///
/// __Note:__ You cannot rotate an envelope.
///
/// __Constraints:__
///
/// - `1 <= envelopes.length <= 5000`
/// - `envelopes[i].length == 2`
/// - `1 <= wi, hi <= 10_000`
///
/// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/592/week-5-march-29th-march-31st/3690/
struct Solution;
//noinspection DuplicatedCode
impl Solution {
    fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        envelopes.sort_unstable_by_key(|v| (v[0], Reverse(v[1])));
        envelopes
            .into_iter()
            .fold(Vec::new(), |mut acc, v| {
                let h = v[1];
                match acc.binary_search(&h) {
                    Err(i) if i == acc.len() => acc.push(h),
                    Err(i) => acc[i] = h,
                    _ => (),
                }
                acc
            })
            .len() as i32
    }

    fn max_envelopes_iterative(mut envelopes: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        envelopes.sort_unstable_by_key(|v| (v[0], Reverse(v[1])));
        let mut hs = vec![];
        for v in envelopes {
            let h = v[1];
            if let Err(i) = hs.binary_search(&h) {
                if i == hs.len() {
                    hs.push(h);
                } else {
                    hs[i] = h;
                }
            }
        }
        hs.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
        assert_eq!(Solution::max_envelopes(envelopes), 3);
        // Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
    }
    #[test]
    fn example2() {
        let envelopes = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::max_envelopes(envelopes), 1);
    }

    #[test]
    fn test33() {
        let envelopes = vec![
            vec![2, 100],
            vec![3, 200],
            vec![4, 300],
            vec![5, 500],
            vec![5, 400],
            vec![5, 250],
            vec![6, 370],
            vec![6, 360],
            vec![7, 380],
        ];
        assert_eq!(Solution::max_envelopes(envelopes), 5);
    }

    #[test]
    fn test_need_to_skip_some_containers() {
        let envelopes = vec![
            vec![2, 100],
            vec![3, 200],
            vec![4, 300],
            vec![5, 500],
            vec![5, 320],
            vec![5, 250],
            vec![6, 100],
            vec![7, 380],
        ];
        assert_eq!(Solution::max_envelopes(envelopes), 5);
    }

    mod performance {
        use super::*;

        #[test]
        fn test_all_1s() {
            let envelopes = (0..5000).map(|_| vec![1, 1]).collect();
            assert_eq!(Solution::max_envelopes(envelopes), 1);
        }
        #[test]
        fn test_all_fit() {
            let envelopes = (1..=5000).map(|i| vec![i, i]).collect();
            assert_eq!(Solution::max_envelopes(envelopes), 5000);
        }
    }
}
