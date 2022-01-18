#![allow(dead_code)]
/// 253. Meeting Rooms II
/// =====================
///
/// Given an array of meeting time intervals `intervals` where `intervals[i] = [start_i, end_i]`,
/// return _the minimum number of conference rooms required_.
///
/// __Constraints:__
///
/// - `1 <= intervals.length <= 10_000`
/// - `0 <= start_i < endi <= 1_000_000`
///
/// https://leetcode.com/problems/meeting-rooms-ii/
struct Solution;
impl Solution {
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        println!("min_meeting_rooms({:?})", intervals);
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut result = 0;
        let mut rooms_due: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        intervals.sort_unstable();
        for i in intervals {
            let (curr_starts, curr_ends) = (i[0], i[1]);
            while let Some(&Reverse(other_ends)) = rooms_due.peek() {
                if other_ends <= curr_starts {
                    rooms_due.pop();
                } else {
                    break;
                }
            }
            rooms_due.push(Reverse(curr_ends));
            result = result.max(rooms_due.len() as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => {vec![$(vec!$x),*]} }

    #[test]
    fn i_3t5_1t3() {
        assert_eq!(Solution::min_meeting_rooms(vv![[3, 5], [1, 3]]), 1);
    }
    #[test]
    fn i_3t5_1t4() {
        assert_eq!(Solution::min_meeting_rooms(vv![[3, 5], [1, 4]]), 2);
    }
    #[test]
    fn i_7t10_2t4() {
        assert_eq!(Solution::min_meeting_rooms(vv![[7, 10], [2, 4]]), 1);
    }
    #[test]
    fn i_0t30_5t10_15t20() {
        let i = vv![[0, 30], [5, 10], [15, 20]];
        assert_eq!(Solution::min_meeting_rooms(i), 2);
    }
}
