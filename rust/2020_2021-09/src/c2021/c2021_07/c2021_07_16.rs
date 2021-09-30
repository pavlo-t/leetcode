#![allow(dead_code)]
/// 4Sum
/// ====
///
/// Given an array `nums` of `n` integers, return _an array of all the __unique__ quadruplets_
/// `[nums[a], nums[b], nums[c], nums[d]]` such that:
///
/// - `0 <= a, b, c, d < n`
/// - `a`, `b`, `c`, and `d` are __distinct__.
/// - `nums[a] + nums[b] + nums[c] + nums[d] == target`
///
/// You may return the answer in __any order__.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 200`
/// - `-10^9 <= nums[i] <= 10^9`
/// - `-10^9 <= target <= 10^9`
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/610/week-3-july-15th-july-21st/3816/
struct Solution;
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn two_sum(ns: &[i32], t: i32, s: usize) -> Vec<Vec<i32>> {
            let mut result = vec![];
            let mut l = s;
            let mut r = ns.len() - 1;
            while l < r {
                let sum = ns[l] + ns[r];
                if sum < t || (l > s && ns[l] == ns[l - 1]) {
                    l += 1;
                } else if sum > t || (r < ns.len() - 1 && ns[r] == ns[r + 1]) {
                    r -= 1;
                } else {
                    result.push(vec![ns[l], ns[r]]);
                    l += 1;
                    r -= 1;
                }
            }
            result
        }
        fn k_sum(ns: &[i32], t: i32, s: usize, k: i32) -> Vec<Vec<i32>> {
            if s >= ns.len() || ns[s] * k > t || ns[ns.len() - 1] * k < t {
                vec![]
            } else if k == 2 {
                two_sum(ns, t, s)
            } else {
                let mut result = vec![];
                for i in s..ns.len() {
                    if i == s || ns[i - 1] != ns[i] {
                        for mut v in k_sum(ns, t - ns[i], i + 1, k - 1) {
                            v.insert(0, ns[i]);
                            //v.push(ns[i]);
                            result.push(v);
                        }
                    }
                }
                result
            }
        }

        let k = 4;
        if nums.len() < k {
            vec![]
        } else {
            nums.sort_unstable();
            k_sum(&nums, target, 0, 4)
        }
    }

    pub fn four_sum_hash_set_optimized(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            vec![]
        } else {
            use std::collections::HashSet;

            nums.sort_unstable();
            let mut ns = vec![];
            for n in nums {
                if ns.len() < 4 || ns[ns.len() - 4] < n {
                    ns.push(n);
                }
            }
            let n = ns.len();
            let mut result = HashSet::new();
            for a in 0..n - 3 {
                if ns[a] * 4 > target {
                    break;
                }
                for b in a + 1..n - 2 {
                    let sum2 = ns[a] + ns[b];
                    if sum2 + ns[b] * 2 > target {
                        break;
                    }
                    for c in b + 1..n - 1 {
                        let sum3 = sum2 + ns[c];
                        if sum3 + ns[c] > target {
                            break;
                        }
                        for d in c + 1..n {
                            let sum4 = sum3 + ns[d];
                            if sum4 > target {
                                break;
                            } else if sum4 == target {
                                result.insert(vec![ns[a], ns[b], ns[c], ns[d]]);
                            }
                        }
                    }
                }
            }
            let mut result = result.into_iter().collect::<Vec<_>>();
            result.sort_unstable();
            result
        }
    }
    pub fn four_sum_hash_map_1(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            vec![]
        } else {
            use std::collections::HashSet;

            nums.sort_unstable();
            let mut ns = vec![];
            for n in nums {
                if ns.len() < 4 || ns[ns.len() - 4] < n {
                    ns.push(n);
                }
            }
            let n = ns.len();
            let mut result = HashSet::new();
            for a in 0..n - 3 {
                for b in a + 1..n - 2 {
                    for c in b + 1..n - 1 {
                        for d in c + 1..n {
                            if ns[a] + ns[b] + ns[c] + ns[d] == target {
                                result.insert(vec![ns[a], ns[b], ns[c], ns[d]]);
                            }
                        }
                    }
                }
            }
            let mut result = result.into_iter().collect::<Vec<_>>();
            result.sort_unstable();
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //Example 1:
    #[test]
    fn n_1_0_m1_0_m2_2_t_0_produces_m2m1p1p2_m2p0p0p2_m1p0p0p1() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        assert_eq!(
            Solution::four_sum(nums, 0),
            [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
        );
    }
    #[test]
    fn n_2_2_2_2_2_t_8_produces_p2p2p2p2() {
        let nums = vec![2, 2, 2, 2, 2];
        assert_eq!(Solution::four_sum(nums, 8), [[2, 2, 2, 2]]);
    }

    #[test]
    fn n_m2m1m1p1p1p2p2_t_0_produces_m2m1p1p2_m1m1p1p1_test_171() {
        let nums = vec![-2, -1, -1, 1, 1, 2, 2];
        assert_eq!(
            Solution::four_sum(nums, 0),
            [[-2, -1, 1, 2], [-1, -1, 1, 1]]
        );
    }
    #[test]
    fn n_m5m4m3m2m1p0p0p1p2p3p4p5_t_0_produces_29_results_test_237() {
        let nums = vec![-5, -4, -3, -2, -1, 0, 0, 1, 2, 3, 4, 5];
        assert_eq!(
            Solution::four_sum(nums, 0),
            [
                [-5, -4, 4, 5],
                [-5, -3, 3, 5],
                [-5, -2, 2, 5],
                [-5, -2, 3, 4],
                [-5, -1, 1, 5],
                [-5, -1, 2, 4],
                [-5, 0, 0, 5],
                [-5, 0, 1, 4],
                [-5, 0, 2, 3],
                [-4, -3, 2, 5],
                [-4, -3, 3, 4],
                [-4, -2, 1, 5],
                [-4, -2, 2, 4],
                [-4, -1, 0, 5],
                [-4, -1, 1, 4],
                [-4, -1, 2, 3],
                [-4, 0, 0, 4],
                [-4, 0, 1, 3],
                [-3, -2, 0, 5],
                [-3, -2, 1, 4],
                [-3, -2, 2, 3],
                [-3, -1, 0, 4],
                [-3, -1, 1, 3],
                [-3, 0, 0, 3],
                [-3, 0, 1, 2],
                [-2, -1, 0, 3],
                [-2, -1, 1, 2],
                [-2, 0, 0, 2],
                [-1, 0, 0, 1]
            ]
        );
    }

    mod performance {
        use super::*;

        #[test]
        fn n_1repeat200_t_4_produces_p1p1p1p1() {
            let nums = vec![1; 200];
            assert_eq!(Solution::four_sum(nums, 4), [[1, 1, 1, 1]]);
        }
        //#[ignore]
        #[test]
        fn n_1to200_t_200_produces_a_lot() {
            let nums = (1..=200).collect();
            //println!("{:?}", nums);
            let result = Solution::four_sum(nums, 200);
            assert_eq!(result.len(), 51_488);
        }
    }
}
