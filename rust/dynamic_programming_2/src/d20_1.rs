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
    /// 21:36-21:51
    pub fn combination_sum_rec(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        println!("combination_sum({:?}, {})", candidates, target);
        fn rec(i: usize, t: i32, cs: &[i32]) -> Vec<Vec<i32>> {
            if t == 0 {
                vec![vec![]]
            } else if t < 0 || i == cs.len() {
                vec![]
            } else {
                let mut pick = rec(i, t - cs[i], cs);
                for j in 0..pick.len() {
                    pick[j].push(cs[i]);
                }
                let mut skip = rec(i + 1, t, cs);
                pick.append(&mut skip);
                pick
            }
        }
        rec(0, target, &candidates)
    }
    /// 21:51-22:12
    pub fn combination_sum_rec_with_memo(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        println!("combination_sum({:?}, {})", candidates, target);
        use std::collections::HashMap;

        fn rec<'a>(
            i: usize,
            t: i32,
            cs: &[i32],
            memo: &'a mut HashMap<(usize, i32), Vec<Vec<i32>>>,
        ) -> &'a Vec<Vec<i32>> {
            if memo.contains_key(&(i, t)) {
                memo.get(&(i, t)).unwrap()
            } else {
                let result = if t == 0 {
                    vec![vec![]]
                } else if t < 0 || i == cs.len() {
                    vec![]
                } else {
                    let mut pick = rec(i, t - cs[i], cs, memo).to_owned();
                    for j in 0..pick.len() {
                        pick[j].push(cs[i]);
                    }
                    let mut skip = rec(i + 1, t, cs, memo).to_owned();
                    pick.append(&mut skip);
                    pick.to_owned()
                };
                memo.insert((i, t), result);
                memo.get(&(i, t)).unwrap()
            }
        }
        let mut memo = HashMap::new();
        let r = rec(0, target, &candidates, &mut memo);
        r.to_owned()
    }

    /// Backtracking from other submissions
    /// https://leetcode.com/submissions/detail/592149834/
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        println!("combination_sum({:?}, {})", candidates, target);
        fn combination_sum_rec(
            candidates: &mut Vec<i32>,
            index: usize,
            sum_combo: &mut Vec<i32>,
            sum: i32,
            combos: &mut Vec<Vec<i32>>,
            target: i32,
        ) {
            if sum == target {
                combos.push(sum_combo.clone());
                return;
            }
            if sum > target {
                return;
            }

            let n = candidates.len();

            for i in index..n {
                let v = candidates[i];
                sum_combo.push(v);
                combination_sum_rec(candidates, i, sum_combo, sum + v, combos, target);
                sum_combo.pop();
            }
        }

        let mut combos = Vec::new();

        combination_sum_rec(&mut candidates, 0, &mut vec![], 0, &mut combos, target);

        return combos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn c2367_t7() {
        let c = vec![2, 3, 6, 7];
        let t = 7;
        let e = vv![[2, 2, 3], [7]];
        //let e = vv![[3, 2, 2], [7]];
        assert_eq!(Solution::combination_sum(c, t), e);
        // Explanation:
        // 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
        // 7 is a candidate, and 7 = 7.
        // These are the only two combinations.
    }
    #[test]
    fn c235_t8() {
        let c = vec![2, 3, 5];
        let t = 8;
        let e = vv![[2, 2, 2, 2], [2, 3, 3], [3, 5]];
        //let e = vv![[2, 2, 2, 2], [3, 3, 2], [5, 3]];
        assert_eq!(Solution::combination_sum(c, t), e);
    }
    #[test]
    fn c2_t1() {
        let c = vec![2];
        let t = 1;
        let e: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combination_sum(c, t), e);
    }
    #[test]
    fn c1_t1() {
        let c = vec![1];
        let t = 1;
        let e = vv![[1]];
        assert_eq!(Solution::combination_sum(c, t), e);
    }
    #[test]
    fn c1_t2() {
        let c = vec![1];
        let t = 2;
        let e = vv![[1, 1]];
        assert_eq!(Solution::combination_sum(c, t), e);
    }

    #[test]
    fn c_1to30_t_30() {
        let c = (1..=30).collect();
        let t = 30;
        let r = Solution::combination_sum(c, t);
        assert_eq!(r.len(), 5604);
    }
    #[test]
    fn c_470to500_t_500() {
        let c = (470..=500).collect();
        let t = 500;
        let e = vv![[500]];
        assert_eq!(Solution::combination_sum(c, t), e);
    }

    #[test]
    fn c_1to30_t_500() {
        let c = (1..=30).collect();
        let t = 500;
        let r = Solution::combination_sum(c, t);
        assert_eq!(r.len(), 5604);
    }
}
