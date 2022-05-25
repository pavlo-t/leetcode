#![allow(dead_code)]
/// \#354. Russian Doll Envelopes
/// =============================
///
/// You are given a 2D array of integers `envelopes` where `envelopes[i] = [wi, hi]` represents
/// the width and the height of an envelope.
///
/// One envelope can fit into another if and only if both the width and height of one envelope
/// are greater than the other envelope's width and height.
///
/// Return _the maximum number of envelopes you can Russian doll (i.e., put one inside the other)_.
///
/// __Note:__ You cannot rotate an envelope.
///
/// __Constraints:__
///
/// - `1 <= envelopes.length <= 100_000`
/// - `envelopes[i].length == 2`
/// - `1 <= wi, hi <= 100_000`
///
/// https://leetcode.com/problems/russian-doll-envelopes/
struct Solution;
impl Solution {
    /// 12:51‥13:27
    pub fn max_envelopes_rec(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable();
        fn rec(i: usize, pw: i32, ph: i32, es: &[Vec<i32>]) -> i32 {
            if i == es.len() {
                0
            } else {
                let skip = rec(i + 1, pw, ph, es);
                let (w, h) = (es[i][0], es[i][1]);
                if w > pw && h > ph {
                    let take = 1 + rec(i + 1, w, h, es);
                    take.max(skip)
                } else {
                    skip
                }
            }
        }
        rec(0, 0, 0, &envelopes)
    }

    /// from old solution `rust/2020_2021-09/src/c2021/c2021_03/c2021_03_30.rs`
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        fn push<V>(v: V, mut vec: Vec<V>) -> Vec<V> {
            vec.push(v);
            vec
        }
        fn set<V>(i: usize, v: V, mut vec: Vec<V>) -> Vec<V> {
            vec[i] = v;
            vec
        }
        envelopes.sort_unstable_by(|a, b| (a[0], b[1]).cmp(&(b[0], a[1])));
        envelopes
            .into_iter()
            .map(|e| e[1])
            .fold(Vec::new(), |acc, h| match acc.binary_search(&h) {
                Err(i) if i == acc.len() => push(h, acc),
                Err(i) => set(i, h, acc),
                _ => acc,
            })
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn e5x4_6x4_6x7_2x3() {
        let e = vv![[5, 4], [6, 4], [6, 7], [2, 3]];
        assert_eq!(Solution::max_envelopes(e), 3);
        // w 5->5,4 6->6,4
        // Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
    }
    #[test]
    fn e1x1_1x1_1x1() {
        let e = vv![[1, 1], [1, 1], [1, 1]];
        assert_eq!(Solution::max_envelopes(e), 1);
    }

    #[rustfmt::skip] #[test] fn e1x1_1x1() { assert_eq!(Solution::max_envelopes(vv![[1,1],[1,1]]), 1); }
    #[rustfmt::skip] #[test] fn e1x1_1x2() { assert_eq!(Solution::max_envelopes(vv![[1,1],[1,2]]), 1); }
    #[rustfmt::skip] #[test] fn e1x1_2x1() { assert_eq!(Solution::max_envelopes(vv![[1,1],[2,1]]), 1); }
    #[rustfmt::skip] #[test] fn e1x1_2x2() { assert_eq!(Solution::max_envelopes(vv![[1,1],[2,2]]), 2); }

    #[test]
    fn e1x1_2x2_3x3_4x1_5x5() {
        let e = vv![[1, 1], [2, 2], [3, 3], [4, 1], [5, 5]];
        assert_eq!(Solution::max_envelopes(e), 4);
    }
    #[test]
    fn e1x1_2x5_3x3_4x4_5x5() {
        let e = vv![[1, 1], [2, 5], [3, 3], [4, 4], [5, 5]];
        assert_eq!(Solution::max_envelopes(e), 4);
    }

    /// Got stack overflow with default stack
    #[test]
    fn e1to100000x1to100000() {
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(31))
            .spawn(|| {
                let e = (1..=100000).map(|i| vec![i, i]).collect();
                assert_eq!(Solution::max_envelopes(e), 100000);
            })
            .unwrap();
        child.join().unwrap();
    }
    #[test]
    fn e1to100000x1() {
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(31))
            .spawn(|| {
                let e = (1..=100000).map(|i| vec![i, 1]).collect();
                assert_eq!(Solution::max_envelopes(e), 1);
            })
            .unwrap();
        child.join().unwrap();
    }
    #[test]
    fn e1x1to100000() {
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(31))
            .spawn(|| {
                let e = (1..=100000).map(|i| vec![1, i]).collect();
                assert_eq!(Solution::max_envelopes(e), 1);
            })
            .unwrap();
        child.join().unwrap();
    }
}
