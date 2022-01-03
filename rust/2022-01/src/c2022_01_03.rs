#![allow(dead_code)]
/// 997. Find the Town Judge
/// ========================
///
/// In a town, there are `n` people labeled from `1` to `n`.
/// There is a rumor that one of these people is secretly the town judge.
///
/// If the town judge exists, then:
///
/// 1. The town judge trusts nobody.
/// 2. Everybody (except for the town judge) trusts the town judge.
/// 3. There is exactly one person that satisfies properties __1__ and __2__.
///
/// You are given an array `trust` where `trust[i] = [ai, bi]`
/// representing that the person labeled `ai` trusts the person labeled `bi`.
///
/// Return _the label of the town judge if the town judge exists and can be identified, or return `-1` otherwise_.
///
/// __Constraints:__
///
/// - `1 <= n <= 1000`
/// - `0 <= trust.length <= 10_000`
/// - `trust[i].length == 2`
/// - All the pairs of `trust` are __unique__.
/// - `ai != bi`
/// - `1 <= ai, bi <= n`
///
/// https://leetcode.com/problems/find-the-town-judge/
struct Solution;
impl Solution {
    pub fn find_judge_my_1(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut trusts_to: Vec<Vec<usize>> = vec![vec![]; n];
        let mut trusted_by: Vec<Vec<usize>> = vec![vec![]; n];
        for t in trust {
            let (a, b) = (t[0] as usize - 1, t[1] as usize - 1);
            trusts_to[a].push(b);
            trusted_by[b].push(a);
        }

        trusts_to
            .iter()
            .enumerate()
            .filter(|(_, tt)| tt.len() == 0)
            .find(|&(a, _)| trusted_by[a].len() == n - 1)
            .map(|(a, _)| a as i32 + 1)
            .unwrap_or(-1)
    }

    /// from other submissions https://leetcode.com/submissions/detail/612047823/
    pub fn find_judge_2(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut trusted_by: Vec<usize> = vec![0; n + 1];
        let mut trusts_someone: Vec<bool> = vec![false; n + 1];
        for t in trust {
            trusts_someone[t[0] as usize] = true;
            trusted_by[t[1] as usize] += 1;
        }

        trusts_someone
            .iter()
            .enumerate()
            .skip(1)
            .find(|&(i, trusts_someone)| !trusts_someone && trusted_by[i] == n - 1)
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }

    /// Approach 2: One Array
    /// https://leetcode.com/problems/find-the-town-judge/solution/
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut trust_score: Vec<i32> = vec![0; n as usize + 1];
        for t in trust {
            trust_score[t[0] as usize] -= 1;
            trust_score[t[1] as usize] += 1;
        }

        trust_score
            .into_iter()
            .enumerate()
            .skip(1)
            .find(|&(_, score)| score == n - 1)
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => {vec![$(vec!$x),*]}; }

    #[test]
    fn n_1_empty() {
        assert_eq!(Solution::find_judge(1, vec![]), 1);
    }
    #[test]
    fn n_2_1t2() {
        assert_eq!(Solution::find_judge(2, vv![[1, 2]]), 2);
    }
    #[test]
    fn n_3_1t3_2t3() {
        assert_eq!(Solution::find_judge(3, vv![[1, 3], [2, 3]]), 3);
    }
    #[test]
    fn n_3_1t3_2t3_3t1() {
        assert_eq!(Solution::find_judge(3, vv![[1, 3], [2, 3], [3, 1]]), -1);
    }
}
