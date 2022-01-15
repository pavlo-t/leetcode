#![allow(dead_code)]
/// 1345. Jump Game IV
/// ==================
///
/// Given an array of integers `arr`, you are initially positioned at the first index of the array.
///
/// In one step you can jump from index `i` to index:
///
/// - `i + 1` where: `i + 1 < arr.length`.
/// - `i - 1` where: `i - 1 >= 0`.
/// - `j` where: `arr[i] == arr[j]` and `i != j`.
///
/// Return _the minimum number of steps_ to reach the __last index__ of the array.
///
/// Notice that you can not jump outside of the array at any time.
///
/// __Constraints:__
///
/// - `1 <= arr.length <= 50_000`
/// - `-100_000_000 <= arr[i] <= 100_000_000`
///
/// https://leetcode.com/problems/jump-game-iv/
struct Solution;
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        //println!("min_jumps({:?})", arr);
        use std::collections::HashMap;
        use std::iter::once;

        let mut portals: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            portals.entry(v).or_default().push(i);
        }
        let n = arr.len();
        let target = n - 1;
        let mut visited = vec![false; n];
        let mut todo = vec![0];
        let mut jumps = 0;

        loop {
            let mut next = vec![];
            while let Some(i) = todo.pop() {
                if i == target {
                    return jumps;
                }
                for &j in portals
                    .get(&arr[i])
                    .unwrap()
                    .iter()
                    .chain(once(&i.wrapping_sub(1)).chain(once(&(i + 1))))
                {
                    if j < n && !visited[j] {
                        visited[j] = true;
                        next.push(j);
                    }
                }
                portals.entry(arr[i]).or_default().clear();
            }
            jumps += 1;
            todo = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_p100m23m23p404p100p23p23p23p3p404() {
        let a = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
        assert_eq!(Solution::min_jumps(a), 3);
        // Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9.
        // Note that index 9 is the last index of the array.
    }
    #[test]
    fn a_7() {
        let a = vec![7];
        assert_eq!(Solution::min_jumps(a), 0);
        // Explanation: Start index is the last index. You do not need to jump.
    }
    #[test]
    fn a_p7p6p9p6p9p6p9p7() {
        let a = vec![7, 6, 9, 6, 9, 6, 9, 7];
        assert_eq!(Solution::min_jumps(a), 1);
        // Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
    }

    #[test]
    fn a_1to50000() {
        let a = (1..=50000).collect();
        assert_eq!(Solution::min_jumps(a), 49999);
    }
    #[test]
    fn a_1repeat50000() {
        let a = vec![1; 50000];
        assert_eq!(Solution::min_jumps(a), 1);
    }
    #[test]
    fn a_0to9repeat5000() {
        let a = (1..=50000).map(|i| i % 10).collect();
        assert_eq!(Solution::min_jumps(a), 3);
    }
    #[test]
    fn a_0x5000_1x5000_2x5000_and_so_on() {
        let mut a: Vec<i32> = (1..=50000).map(|i| i % 10).collect();
        a.sort_unstable();
        assert_eq!(Solution::min_jumps(a), 19);
    }
    #[test]
    fn a_0x10_1x10_2x10_and_so_on() {
        let mut a: Vec<i32> = (1..=50000).map(|i| i % 5000).collect();
        a.sort_unstable();
        assert_eq!(Solution::min_jumps(a), 9999);
    }
    #[test]
    fn a_7x49999_11x1() {
        let mut a = vec![7; 49999];
        a.push(11);
        assert_eq!(Solution::min_jumps(a), 2);
    }
}
