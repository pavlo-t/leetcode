#![allow(dead_code)]
/// Keys and Rooms
/// ==============
///
/// There are `N` rooms and you start in room `0`.
/// Each room has a distinct number in `0, 1, 2, ..., N-1`,
/// and each room may have some keys to access the next room.
///
/// Formally, each room `i` has a list of keys `rooms[i]`,
/// and each key `rooms[i][j]` is an integer in `[0, 1, ..., N-1]` where `N = rooms.length`.
/// A key `rooms[i][j] = v` opens the room with number `v`.
///
/// Initially, all the rooms start locked (except for room `0`).
///
/// You can walk back and forth between rooms freely.
///
/// Return `true` if and only if you can enter every room.
///
/// __Note:__
///
/// - `1 <= rooms.length <= 1000`
/// - `0 <= rooms[i].length <= 1000`
/// - The number of keys in all rooms combined is at most `3000`.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3677/
struct Solution;
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        visited[0] = true;
        let mut to_see = rooms[0].iter().map(|&i| i as usize).collect::<Vec<usize>>();
        to_see.iter().for_each(|&r| visited[r] = true);

        while let Some(r) = to_see.pop() {
            for &i in &rooms[r] {
                let nr = i as usize;
                if !visited[nr] {
                    visited[nr] = true;
                    to_see.push(nr);
                }
            }
        }

        visited.into_iter().all(|v| v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];
        assert!(Solution::can_visit_all_rooms(rooms));
        // Explanation:
        // We start in room 0, and pick up key 1.
        // We then go to room 1, and pick up key 2.
        // We then go to room 2, and pick up key 3.
        // We then go to room 3.  Since we were able to go to every room, we return true.
    }
    #[test]
    fn example2() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        assert!(!Solution::can_visit_all_rooms(rooms));
        // Explanation: We can't enter the room with number 2.
    }
}
