#![allow(dead_code)]
/// 1286. Iterator for Combination
/// ==============================
///
/// Design the `CombinationIterator` class:
///
/// - `CombinationIterator(string characters, int combinationLength)`
///   Initializes the object with a string `characters` of __sorted distinct__ lowercase English letters
///   and a number `combinationLength` as arguments.
/// - `next()` Returns the next combination of length `combinationLength` in __lexicographical order__.
/// - `hasNext()` Returns `true` if and only if there exists a next combination.
///
/// Constraints:
///
/// - `1 <= combinationLength <= characters.length <= 15`
/// - All the characters of `characters` are __unique__.
/// - At most `10_000` calls will be made to `next` and `hasNext`.
/// - It's guaranteed that all calls of the function `next` are valid.
///
/// https://leetcode.com/problems/iterator-for-combination/
#[derive(Debug, Default)]
struct CombinationIterator {
    combinations: Vec<String>,
}
impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let bytes = characters.into_bytes();
        let cl = combination_length as u32;
        let mut combinations = vec![];
        for bitmask in 0i32..1 << bytes.len() {
            if bitmask.count_ones() == cl {
                let mut s = String::with_capacity(cl as usize);
                for i in 0..bytes.len() {
                    if ((1 << bytes.len() - i - 1) & bitmask) != 0 {
                        s.push(bytes[i] as char);
                    }
                }
                combinations.push(s);
            }
        }

        Self { combinations }
    }

    fn next(&mut self) -> String {
        self.combinations.pop().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.combinations.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test]
    fn abc_3() {
        let mut itr = CombinationIterator::new("abc".into(), 3);
        assert!(itr.has_next()); assert_eq!(itr.next(), "abc");
        assert!(!itr.has_next());
    }
    #[rustfmt::skip] #[test]
    fn abc_1() {
        let mut itr = CombinationIterator::new("abc".into(), 1);
        assert!(itr.has_next()); assert_eq!(itr.next(), "a");
        assert!(itr.has_next()); assert_eq!(itr.next(), "b");
        assert!(itr.has_next()); assert_eq!(itr.next(), "c");
        assert!(!itr.has_next());
    }
    #[rustfmt::skip] #[test]
    fn abc_2() {
        let mut itr = CombinationIterator::new("abc".into(), 2);
        assert!(itr.has_next()); assert_eq!(itr.next(), "ab"); // [0,1]
        assert!(itr.has_next()); assert_eq!(itr.next(), "ac"); // [0,2]
        assert!(itr.has_next()); assert_eq!(itr.next(), "bc"); // [1,2]
        assert!(!itr.has_next());
    }
    #[rustfmt::skip] #[test]
    fn abcd_3() {
        let mut itr = CombinationIterator::new("abcd".into(), 3);
        assert!(itr.has_next()); assert_eq!(itr.next(), "abc"); // [0,1,2]
        assert!(itr.has_next()); assert_eq!(itr.next(), "abd"); // [0,1,3]
        assert!(itr.has_next()); assert_eq!(itr.next(), "acd"); // [0,2,3]
        assert!(itr.has_next()); assert_eq!(itr.next(), "bcd"); // [1,2,3]
        assert!(!itr.has_next());
    }
    #[rustfmt::skip] #[test]
    fn abcde_3() {
        let mut itr = CombinationIterator::new("abcde".into(), 3);
        assert!(itr.has_next()); assert_eq!(itr.next(), "abc"); // [0,1,2]
        assert!(itr.has_next()); assert_eq!(itr.next(), "abd"); // [0,1,3]
        assert!(itr.has_next()); assert_eq!(itr.next(), "abe"); // [0,1,4]
        assert!(itr.has_next()); assert_eq!(itr.next(), "acd"); // [0,2,3]
        assert!(itr.has_next()); assert_eq!(itr.next(), "ace"); // [0,2,4]
        assert!(itr.has_next()); assert_eq!(itr.next(), "ade"); // [0,3,4]
        assert!(itr.has_next()); assert_eq!(itr.next(), "bcd"); // [1,2,3]
        assert!(itr.has_next()); assert_eq!(itr.next(), "bce"); // [1,2,4]
        assert!(itr.has_next()); assert_eq!(itr.next(), "bde"); // [1,3,4]
        assert!(itr.has_next()); assert_eq!(itr.next(), "cde"); // [2,3,4]
        assert!(!itr.has_next());
    }
}
