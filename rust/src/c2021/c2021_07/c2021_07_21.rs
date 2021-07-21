#![allow(dead_code)]
/// Push Dominoes
/// =============
///
/// There are `n` dominoes in a line, and we place each domino vertically upright.
/// In the beginning, we simultaneously push some of the dominoes either to the left or to the right.
///
/// After each second, each domino that is falling to the left pushes the adjacent domino on the left.
/// Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.
///
/// When a vertical domino has dominoes falling on it from both sides, it stays still due to the balance of the forces.
///
/// For the purposes of this question,
/// we will consider that a falling domino expends no additional force to a falling or already fallen domino.
///
/// You are given a string `dominoes` representing the initial state where:
///
/// - `dominoes[i] = 'L'`, if the `i`th domino has been pushed to the left,
/// - `dominoes[i] = 'R'`, if the `i`th domino has been pushed to the right, and
/// - `dominoes[i] = '.'`, if the `i`th domino has not been pushed.
///
/// Return _a string representing the final state_.
///
/// __Constraints:__
///
/// - `1 <= dominoes.length <= 100_000`
/// - `dominoes[i]` is either `'L'`, `'R'`, or `'.'`.
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/610/week-3-july-15th-july-21st/3821/
struct Solution;
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut bytes = dominoes.into_bytes();
        let mut pointers = vec![];
        for (i, &b) in bytes.iter().enumerate() {
            match b {
                b'L' => pointers.push((i, b'L')),
                b'R' => pointers.push((i, b'R')),
                _ => (),
            }
        }
        if pointers.len() > 0 {
            fn set_bytes(bs: &mut [u8], f: usize, t: usize, b: u8) {
                for i in f..t {
                    bs[i] = b;
                }
            }
            let mut i = 0;
            let mut j = 0;
            while i < bytes.len() && j < pointers.len() {
                if pointers[j].1 == b'L' {
                    let to_i = pointers[j].0;
                    set_bytes(&mut bytes, i, to_i, b'L');
                    i = to_i + 1;
                    j += 1;
                } else {
                    i = pointers[j].0 + 1;
                    let nj = j + 1;
                    let (to_i, ni) = if nj == pointers.len() {
                        (bytes.len(), bytes.len())
                    } else if pointers[nj].1 == b'R' {
                        (pointers[nj].0, pointers[nj].0 + 1)
                    } else {
                        let d = pointers[nj].0 + 1 - pointers[j].0;
                        let to_i = pointers[j].0 + d / 2;
                        (to_i, if d % 2 == 0 { to_i } else { to_i + 1 })
                    };
                    set_bytes(&mut bytes, i, to_i, b'R');
                    i = ni;
                    j = nj;
                }
            }
        }
        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_rril_produces_rril() {
        let dominoes = "RR.L".to_string();
        assert_eq!(Solution::push_dominoes(dominoes), "RR.L");
        // Explanation: The first domino expends no additional force on the second domino.
    }
    #[test]
    fn d_iliriiilriilii_produces_llirrillrrllii() {
        let dominoes = ".L.R...LR..L..".to_string();
        assert_eq!(Solution::push_dominoes(dominoes), "LL.RR.LLRRLL..");
        // 01234567890123
        // |\|/|||\/||\||
        // \\|//|\\//\\||
    }
    #[test]
    fn d_liii_produces_liii() {
        let dominoes = "L...".to_string();
        assert_eq!(Solution::push_dominoes(dominoes), "L...");
    }
    #[test]
    fn d_iiir_produces_iiir() {
        let dominoes = "...R".to_string();
        assert_eq!(Solution::push_dominoes(dominoes), "...R");
    }
    #[test]
    fn d_riiiil_produces_rrrlll() {
        let dominoes = "R....L".to_string();
        assert_eq!(Solution::push_dominoes(dominoes), "RRRLLL");
    }
    #[test]
    fn d_riiiiil_produces_rrrilll() {
        let dominoes = "R.....L".to_string();
        assert_eq!(Solution::push_dominoes(dominoes), "RRR.LLL");
    }

    mod performance {
        use super::*;

        #[test]
        fn d_r99999i_produces_100000r() {
            let mut dominoes = "R".to_string();
            for _ in 0..99999 {
                dominoes.push('.');
            }
            assert_eq!(Solution::push_dominoes(dominoes), "R".repeat(100000));
        }
        #[test]
        fn d_99999il_produces_100000l() {
            let mut dominoes = ".".repeat(99999);
            dominoes.push('L');
            assert_eq!(Solution::push_dominoes(dominoes), "L".repeat(100000));
        }
        #[test]
        fn d_100000i_produces_100000i() {
            let dominoes = ".".repeat(100000);
            assert_eq!(Solution::push_dominoes(dominoes), ".".repeat(100000));
        }
        #[test]
        fn d_100000r_produces_100000r() {
            let dominoes = "R".repeat(100000);
            assert_eq!(Solution::push_dominoes(dominoes), "R".repeat(100000));
        }
        #[test]
        fn d_100000l_produces_100000l() {
            let dominoes = "L".repeat(100000);
            assert_eq!(Solution::push_dominoes(dominoes), "L".repeat(100000));
        }
    }
}
