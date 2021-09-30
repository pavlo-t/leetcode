#![allow(dead_code)]
/// Open the Lock
/// =============
///
/// You have a lock in front of you with 4 circular wheels.
/// Each wheel has 10 slots: `'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'`.
/// The wheels can rotate freely and wrap around:
/// for example we can turn `'9'` to be `'0'`, or `'0'` to be `'9'`.
/// Each move consists of turning one wheel one slot.
///
/// The lock initially starts at `'0000'`, a string representing the state of the 4 wheels.
///
/// You are given a list of `deadends` dead ends,
/// meaning if the lock displays any of these codes,
/// the wheels of the lock will stop turning and you will be unable to open it.
///
/// Given a `target` representing the value of the wheels that will unlock the lock,
/// return the minimum total number of turns required to open the lock, or `-1` if it is impossible.
///
/// __Constraints:__
///
/// - `1 <= deadends.length <= 500`
/// - `deadends[i].length == 4`
/// - `target.length == 4`
/// - `target` __will not be__ in the list `deadends`.
/// - `target` and `deadends[i]` consist of digits only.
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/603/week-1-june-1st-june-7th/3767/
struct Solution;
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        use std::collections::{HashSet, VecDeque};

        const NEXT_MOVE: i32 = -1;

        let ds = deadends
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<HashSet<_>>();
        let t = target.parse::<i32>().unwrap();
        let mut q = VecDeque::new();
        q.push_back(0000);
        q.push_back(NEXT_MOVE);
        let mut seen = HashSet::new();
        seen.insert(0000);
        let mut moves = 0;

        fn push_next(i: i32, m: i32, q: &mut VecDeque<i32>, seen: &mut HashSet<i32>) {
            let (upper, lower) = match m {
                10000 => (9000, 1000),
                1000 => (900, 100),
                100 => (90, 10),
                10 => (9, 1),
                _ => unreachable!(),
            };
            let r = i % m;
            let n = if r >= upper { i - upper } else { i + lower };
            if !seen.contains(&n) {
                seen.insert(n);
                q.push_back(n);
            }
            let n = if r < lower { i + upper } else { i - lower };
            if !seen.contains(&n) {
                seen.insert(n);
                q.push_back(n);
            }
        }

        while let Some(i) = q.pop_front() {
            if i == NEXT_MOVE {
                moves += 1;
                q.push_back(NEXT_MOVE);
                if q.front().unwrap() == &NEXT_MOVE {
                    return -1;
                }
            } else if i == t {
                return moves;
            } else if !ds.contains(&i) {
                push_next(i, 10000, &mut q, &mut seen);
                push_next(i, 1000, &mut q, &mut seen);
                push_next(i, 100, &mut q, &mut seen);
                push_next(i, 10, &mut q, &mut seen);
            }
        }

        unreachable!()
    }

    pub fn open_lock_2(deadends: Vec<String>, target: String) -> i32 {
        use std::collections::{HashSet, VecDeque};

        const NEXT_MOVE: i32 = -1;

        let ds = deadends
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<HashSet<_>>();
        let t = target.parse::<i32>().unwrap();
        let mut q = VecDeque::new();
        q.push_back(0000);
        q.push_back(NEXT_MOVE);
        let mut seen = HashSet::new();
        seen.insert(0000);
        let mut moves = 0;

        let mut ns = Vec::with_capacity(8);

        while let Some(i) = q.pop_front() {
            if i == NEXT_MOVE {
                moves += 1;
                q.push_back(NEXT_MOVE);
                if q.front().unwrap() == &NEXT_MOVE {
                    return -1;
                }
            } else if i == t {
                return moves;
            } else if !ds.contains(&i) {
                if i >= 9000 {
                    ns.push(i - 9000);
                } else {
                    ns.push(i + 1000);
                }
                if i < 1000 {
                    ns.push(i + 9000)
                } else {
                    ns.push(i - 1000);
                }

                if i % 1000 >= 900 {
                    ns.push(i - 900);
                } else {
                    ns.push(i + 100);
                }
                if i % 1000 < 100 {
                    ns.push(i + 900);
                } else {
                    ns.push(i - 100);
                }

                if i % 100 >= 90 {
                    ns.push(i - 90);
                } else {
                    ns.push(i + 10);
                }
                if i % 100 < 10 {
                    ns.push(i + 90);
                } else {
                    ns.push(i - 10);
                }

                if i % 10 >= 9 {
                    ns.push(i - 9);
                } else {
                    ns.push(i + 1);
                }
                if i % 10 < 1 {
                    ns.push(i + 9);
                } else {
                    ns.push(i - 1);
                }

                while let Some(n) = ns.pop() {
                    if !seen.contains(&n) {
                        seen.insert(n);
                        q.push_back(n);
                    }
                }
            }
        }

        unreachable!()
    }

    pub fn open_lock_1(deadends: Vec<String>, target: String) -> i32 {
        use std::collections::{HashSet, VecDeque};

        const NEXT_MOVE: i32 = -1;

        let ds = deadends
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<HashSet<_>>();
        let t = target.parse::<i32>().unwrap();
        let mut q = VecDeque::new();
        q.push_back(0000);
        q.push_back(NEXT_MOVE);
        let mut seen = HashSet::new();
        seen.insert(0000);
        let mut moves = 0;

        while let Some(i) = q.pop_front() {
            if i == NEXT_MOVE {
                moves += 1;
                q.push_back(NEXT_MOVE);
                if q.front().unwrap() == &NEXT_MOVE {
                    return -1;
                }
            } else if i == t {
                return moves;
            } else if !ds.contains(&i) {
                let mut ns = vec![];
                if i >= 9000 {
                    ns.push(i - 9000);
                } else {
                    ns.push(i + 1000);
                }
                if i < 1000 {
                    ns.push(i + 9000)
                } else {
                    ns.push(i - 1000);
                }

                if i % 1000 >= 900 {
                    ns.push(i - 900);
                } else {
                    ns.push(i + 100);
                }
                if i % 1000 < 100 {
                    ns.push(i + 900);
                } else {
                    ns.push(i - 100);
                }

                if i % 100 >= 90 {
                    ns.push(i - 90);
                } else {
                    ns.push(i + 10);
                }
                if i % 100 < 10 {
                    ns.push(i + 90);
                } else {
                    ns.push(i - 10);
                }

                if i % 10 >= 9 {
                    ns.push(i - 9);
                } else {
                    ns.push(i + 1);
                }
                if i % 10 < 1 {
                    ns.push(i + 9);
                } else {
                    ns.push(i - 1);
                }

                ns.into_iter().for_each(|n| {
                    if !seen.contains(&n) {
                        seen.insert(n);
                        q.push_back(n);
                    }
                })
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}

    #[test]
    fn ds_0201_0101_0102_1212_2002_t_0202_produces_6() {
        let deadends = vs!["0201", "0101", "0102", "1212", "2002"];
        let target = "0202".to_string();
        assert_eq!(Solution::open_lock(deadends, target), 6);
        // Explanation:
        // A sequence of valid moves would be "0000" -> "1000" -> "1100" -> "1200" -> "1201" -> "1202" -> "0202".
        // Note that a sequence like "0000" -> "0001" -> "0002" -> "0102" -> "0202" would be invalid,
        // because the wheels of the lock become stuck after the display becomes the dead end "0102".
    }
    #[test]
    fn ds_8888_t_0009_produces_1() {
        let deadends = vs!["8888"];
        let target = "0009".to_string();
        assert_eq!(Solution::open_lock(deadends, target), 1);
        // Explanation:
        // We can turn the last wheel in reverse to move from "0000" -> "0009".
    }
    #[test]
    fn ds_8887_8889_8878_8898_8788_8988_7888_9888_t_8888_produces_m1() {
        let deadends = vs!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"];
        let target = "8888".to_string();
        assert_eq!(Solution::open_lock(deadends, target), -1);
        // Explanation:
        // We can't reach the target without getting stuck.
    }
    #[test]
    fn ds_0000_t_8888_produces_m1() {
        assert_eq!(Solution::open_lock(vs!["0000"], "8888".to_string()), -1);
    }
}
