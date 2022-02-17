#![allow(dead_code)]
/// 39. Combination Sum
/// ===================
///
/// Given an array of __distinct__ integers `candidates` and a target integer `target`,
/// return _a list of all __unique combinations__ of `candidates` where the chosen numbers sum to `target`_.
/// You may return the combinations in __any order__.
///
/// The __same__ number may be chosen from `candidates` an __unlimited number of times__.
/// Two combinations are unique if the frequency of at least one of the chosen numbers is different.
///
/// It is __guaranteed__ that the number of unique combinations that sum up to `target`
/// is less than `150` combinations for the given input.
///
/// __Constraints:__
///
/// - `1 <= candidates.length <= 30`
/// - `1 <= candidates[i] <= 200`
/// - All elements of candidates are __distinct__.
/// - `1 <= target <= 500`
///
/// https://leetcode.com/problems/combination-sum/
struct Solution;
impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        println!("combination_sum({candidates:?}, {target})");
        fn bts(
            i: usize,
            target: i32,
            candidates: &[i32],
            curr: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if i < candidates.len() {
                if candidates[i] < target {
                    curr.push(candidates[i]);
                    bts(i, target - candidates[i], candidates, curr, result);
                    curr.pop();
                    bts(i + 1, target, candidates, curr, result);
                } else if candidates[i] == target {
                    curr.push(candidates[i]);
                    result.push(curr.clone());
                    curr.pop();
                }
            }
        }

        candidates.sort_unstable();
        let mut result = vec![];
        bts(0, target, &candidates, &mut vec![], &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn c_2_3_6_7_t_7() {
        let c = vec![2, 3, 6, 7];
        let t = 7;
        let e = vv![[2, 2, 3], [7]];
        assert_eq!(Solution::combination_sum(c, t), e);
        // Explanation:
        // 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
        // 7 is a candidate, and 7 = 7.
        // These are the only two combinations.
    }
    #[test]
    fn c_2_3_5_t_8() {
        let c = vec![2, 3, 5];
        let t = 8;
        let e = vv![[2, 2, 2, 2], [2, 3, 3], [3, 5]];
        assert_eq!(Solution::combination_sum(c, t), e);
    }
    #[test]
    fn c_2_t_1() {
        let c = vec![2];
        let t = 1;
        let e: Vec<Vec<i32>> = vv![];
        assert_eq!(Solution::combination_sum(c, t), e);
    }
    #[test]
    fn c_2_t_2() {
        let c = vec![2];
        let t = 2;
        let e = vv![[2]];
        assert_eq!(Solution::combination_sum(c, t), e);
    }
    #[test]
    fn c_1to30_t_30() {
        let c = (1..=30).collect::<Vec<i32>>();
        let t = 30;
        let r = Solution::combination_sum(c, t);
        assert_eq!(r.len(), 5604);
        let s = r.into_iter().collect::<HashSet<_>>();
        assert_eq!(s.len(), 5604);
    }
}
