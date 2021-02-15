#![allow(dead_code)]

/// ### Kill Process
///
/// Given `n` processes, each process has a unique `PID` (process id) and its `PPID` (parent process id).
///
/// Each process only has one parent process, but may have one or more children processes.
/// This is just like a tree structure.
/// Only one process has `PPID` that is `0`, which means this process has no parent process.
/// All the `PID`s will be distinct positive integers.
///
/// We use two list of integers to represent a list of processes, where the first list
/// contains `PID` for each process and the second list contains the corresponding `PPID`.
///
/// Now given the two lists, and a `PID` representing a process you want to kill,
/// return a list of `PID`s of processes that will be killed in the end.
/// You should assume that when a process is killed, all its children processes will be killed.
/// No order is required for the final answer.
///
/// __Note:__
///
/// 1. The given `kill` id is guaranteed to be one of the given `PID`s.
/// 2. `n >= 1`.
///
/// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/586/week-3-february-15th-february-21st/3640/
struct Solution;

impl Solution {
    pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut tree: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, &pp) in ppid.iter().enumerate() {
            tree.entry(pp).or_insert(Vec::new()).push(pid[i]);
        }

        let mut result = Vec::new();
        let mut kill_q = vec![kill];
        while let Some(p) = kill_q.pop() {
            result.push(p);
            if let Some(children) = tree.get(&p) {
                for &c in children {
                    kill_q.push(c);
                }
            }
        }

        result
    }

    pub fn kill_process_brute_force(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut kill_q = vec![kill];
        while let Some(p) = kill_q.pop() {
            result.push(p);
            for (i, &pp) in ppid.iter().enumerate() {
                if pp == p {
                    kill_q.push(pid[i]);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn example() {
        let pid = vec![1, 3, 10, 5];
        let ppid = vec![3, 0, 5, 3];
        let result = Solution::kill_process(pid, ppid, 5);
        let result = result.into_iter().collect::<HashSet<_>>();
        let expected = vec![5, 10].into_iter().collect::<HashSet<_>>();
        assert_eq!(result, expected)
        // Explanation:
        //            3
        //          /   \
        //         1     5
        //              /
        //             10
        // Kill 5 will also kill 10.
    }

    #[test]
    fn test1() {
        let pid = vec![1, 3, 10, 5];
        let ppid = vec![3, 0, 5, 3];
        let result = Solution::kill_process(pid, ppid, 3);
        let result = result.into_iter().collect::<HashSet<_>>();
        let expected = vec![3, 1, 5, 10].into_iter().collect::<HashSet<_>>();
        assert_eq!(result, expected)
        //    3
        //  /   \
        // 1     5
        //      /
        //     10
    }
}