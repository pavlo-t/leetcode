#![allow(dead_code)]
/// 1101. The Earliest Moment When Everyone Become Friends
/// ======================================================
///
/// There are n people in a social group labeled from `0` to `n - 1`.
/// You are given an array logs where `logs[i] = [timestamp_i, x_i, y_i]`
/// indicates that `x_i` and `y_i` will be friends at the time `timestamp_i`.
///
/// Friendship is __symmetric__.
/// That means if `a` is friends with `b`, then `b` is friends with `a`.
/// Also, person `a` is acquainted with a person `b` if `a` is friends with `b`,
/// or `a` is a friend of someone acquainted with `b`.
///
/// Return _the earliest time for which every person became acquainted with every other person_.
/// If there is no such earliest time, return `-1`.
///
/// __Constraints:__
///
/// - `2 <= n <= 100`
/// - `1 <= logs.length <= 10_000`
/// - `logs[i].length == 3`
/// - `0 <= timestampi <= 1_000_000_000`
/// - `0 <= xi, yi <= n - 1`
/// - `xi != yi`
/// - All the values `timestampi` are __unique__.
/// - All the pairs `(xi, yi)` occur at most one time in the input.
///
/// https://leetcode.com/problems/the-earliest-moment-when-everyone-become-friends/
struct Solution;
impl Solution {
    pub fn earliest_acq(mut logs: Vec<Vec<i32>>, n: i32) -> i32 {
        use std::collections::HashSet;

        let mut groups = vec![HashSet::new(); n as usize];
        for i in 0..(n as usize) {
            groups[i].insert(i);
        }

        logs.sort_unstable();

        for l in logs {
            let (timestamp, a, b) = (l[0], l[1] as usize, l[2] as usize);
            let ia = groups.iter().position(|g| g.contains(&a)).unwrap();
            if !groups[ia].contains(&b) {
                if groups.len() == 2 {
                    return timestamp;
                }

                let ib = groups.iter().position(|g| g.contains(&b)).unwrap();
                let ia = if ia == groups.len() - 1 { ib } else { ia };
                for p in groups.swap_remove(ib) {
                    groups[ia].insert(p);
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn example_1() {
        let l = vv![
            [20190101, 0, 1],
            [20190104, 3, 4],
            [20190107, 2, 3],
            [20190211, 1, 5],
            [20190224, 2, 4],
            [20190301, 0, 3],
            [20190312, 1, 2],
            [20190322, 4, 5]
        ];
        assert_eq!(Solution::earliest_acq(l, 6), 20190301);
        // Explanation:
        // The first event occurs at timestamp = 20190101 and after 0 and 1 become friends
        // we have the following friendship groups [0,1], [2], [3], [4], [5].
        // The second event occurs at timestamp = 20190104 and after 3 and 4 become friends
        // we have the following friendship groups [0,1], [2], [3,4], [5].
        // The third event occurs at timestamp = 20190107 and after 2 and 3 become friends
        // we have the following friendship groups [0,1], [2,3,4], [5].
        // The fourth event occurs at timestamp = 20190211 and after 1 and 5 become friends
        // we have the following friendship groups [0,1,5], [2,3,4].
        // The fifth event occurs at timestamp = 20190224 and as 2 and 4 are already friends nothing happens.
        // The sixth event occurs at timestamp = 20190301 and after 0 and 3 become friends all are friends.
    }
    #[test]
    fn example_1_unsorted() {
        let l = vv![
            [20190322, 4, 5],
            [20190301, 0, 3],
            [20190101, 0, 1],
            [20190312, 1, 2],
            [20190104, 3, 4],
            [20190107, 2, 3],
            [20190211, 1, 5],
            [20190224, 2, 4]
        ];
        assert_eq!(Solution::earliest_acq(l, 6), 20190301);
    }
    #[test]
    fn example_2() {
        let l = vv![[0, 2, 0], [1, 0, 1], [3, 0, 3], [4, 1, 2], [7, 3, 1]];
        assert_eq!(Solution::earliest_acq(l, 4), 3);
    }

    #[test]
    fn l_10000_records_n_100_result_m1() {
        let l = (0..10_000).map(|t| vec![t, 1, 0]).collect();
        assert_eq!(Solution::earliest_acq(l, 100), -1);
    }
}
