/// # Stamping The Sequence
///
/// You want to form a `target` string of __lowercase letters__.
///
/// At the beginning, your sequence is `target.length` `'?'` marks.
/// You also have a `stamp` of lowercase letters.
///
/// On each turn, you may place the stamp over the sequence,
/// and replace every letter in the sequence with the corresponding letter from the stamp.
/// You can make up to `10 * target.length` turns.
///
/// For example, if the initial sequence is "?????", and your stamp is "abc",
/// then you may make "abc??", "?abc?", "??abc" in the first turn.
/// Note that the stamp must be fully contained in the boundaries of the sequence in order to stamp.
///
/// If the sequence is possible to stamp, then return an array of the index of the left-most letter
/// being stamped at each turn.
/// If the sequence is not possible to stamp, return an empty array.
///
/// For example, if the sequence is "ababc", and the stamp is `"abc"`,
/// then we could return the answer `[0, 2]`,
/// corresponding to the moves "?????" -> "abc??" -> "ababc".
///
/// Also, if the sequence is possible to stamp, it is guaranteed it is possible to stamp within
/// `10 * target.length` moves.
/// Any answers specifying more than this number of moves will not be accepted.
///
/// __Note:__
///
/// - `1 <= stamp.length <= target.length <= 1000`
/// - `stamp` and `target` only contain lowercase letters.
///
/// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/592/week-5-march-29th-march-31st/3691/
struct Solution;
impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        fn stamp_eql(stamp: &[u8], other: &[u8]) -> bool {
            let mut real_matches = 0;
            for i in 0..other.len() {
                if other[i] != b'_' {
                    if other[i] != stamp[i] {
                        return false;
                    }
                    real_matches += 1;
                }
            }
            real_matches > 0
        }

        let mut target_bytes = target.as_bytes().to_vec();
        let stamp_bytes = stamp.as_bytes();
        let stamp_len = stamp_bytes.len();
        let mut result = Vec::new();
        let mut replaced_bytes = 0;
        while replaced_bytes < target_bytes.len() {
            let mut rbs = 0;
            let mut i = 0;
            while i <= target_bytes.len() - stamp_len {
                if stamp_eql(stamp_bytes, &target_bytes[i..(i + stamp_len)]) {
                    result.push(i as i32);
                    for j in 0..stamp_len {
                        if target_bytes[i + j] != b'_' {
                            rbs += 1;
                            target_bytes[i + j] = b'_';
                        }
                    }
                    i += stamp_len;
                } else {
                    i += 1;
                }
            }
            if rbs == 0 {
                return Vec::new();
            }
            replaced_bytes += rbs;
        }
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_s_abc_t_ababc() {
        let stamp = "abc".to_string();
        let target = "ababc".to_string();
        let expected = vec![0, 2];
        assert_eq!(Solution::moves_to_stamp(stamp, target), expected);
        // ([1,0,2] would also be accepted as an answer, as well as some other answers.)
    }
    #[test]
    fn example2_s_abca_t_aabcaca() {
        let stamp = "abca".to_string();
        let target = "aabcaca".to_string();
        let expected = vec![3, 0, 1];
        assert_eq!(Solution::moves_to_stamp(stamp, target), expected);
    }

    #[test]
    fn test15_s_aye_t_eyeye() {
        let stamp = "aye".to_string();
        let target = "eyeye".to_string();
        assert_eq!(Solution::moves_to_stamp(stamp, target), []);
    }

    #[test]
    fn test_s_abc_t_abcbc() {
        let stamp = "abc".to_string();
        let target = "abcbc".to_string();
        let expected = vec![2, 0];
        assert_eq!(Solution::moves_to_stamp(stamp, target), expected);
    }
    #[test]
    fn test_s_abc_t_aaabc() {
        let stamp = "abc".to_string();
        let target = "aaabc".to_string();
        let expected = vec![0, 1, 2];
        assert_eq!(Solution::moves_to_stamp(stamp, target), expected);
    }

    mod performance {
        use super::*;

        #[test]
        fn test_s_ab_t_999a1b() {
            let stamp = "ab".to_string();
            let mut target = "a".repeat(999);
            target.push('b');
            let expected = (0..999).collect::<Vec<_>>();
            assert_eq!(Solution::moves_to_stamp(stamp, target), expected);
        }
    }
}
