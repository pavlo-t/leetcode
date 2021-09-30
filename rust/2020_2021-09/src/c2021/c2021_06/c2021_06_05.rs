#![allow(dead_code)]
/// Maximum Performance of a Team
/// =============================
///
/// You are given two integers `n` and `k` and two integer arrays `speed` and `efficiency` both of length `n`.
/// There are `n` engineers numbered from `1` to `n`.
/// `speed[i]` and `efficiency[i]` represent the speed and efficiency of the `i`th engineer respectively.
///
/// Choose __at most__ `k` different engineers out of the `n` engineers to form a team with the maximum __performance__.
///
/// The performance of a team is the sum of their engineers' speeds multiplied by the minimum efficiency among their engineers.
///
/// Return _the maximum performance of this team_.
/// Since the answer can be a huge number, return it __modulo__ `10^9 + 7`.
///
/// __Constraints:__
///
/// - `1 <= <= k <= n <= 100_000`
/// - `speed.length == n`
/// - `efficiency.length == n`
/// - `1 <= speed[i] <= 100_000`
/// - `1 <= efficiency[i] <= 100_000_000`
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/603/week-1-june-1st-june-7th/3768/
struct Solution;
impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = n as usize;
        let k = k as usize;

        let mut ses = speed.into_iter().zip(efficiency).collect::<Vec<_>>();
        ses.sort_unstable_by_key(|&(_, e)| Reverse(e));

        let (max_speed_sums, _) = ses.iter().fold(
            (Vec::with_capacity(n), BinaryHeap::with_capacity(k)),
            |(mut rsf, mut h), &(s, _)| {
                let s = s as i64;
                let mut r = rsf.last().unwrap_or(&0).to_owned();
                r += s;
                h.push(Reverse(s));
                if h.len() > k {
                    let Reverse(m) = h.pop().unwrap();
                    r -= m;
                }
                rsf.push(r);
                (rsf, h)
            },
        );

        (ses.into_iter()
            .map(|(_, e)| e as i64)
            .zip(max_speed_sums)
            .map(|(e, s)| e * s)
            .max()
            .unwrap()
            % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_n6_k2_produces_60() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 2;
        assert_eq!(Solution::max_performance(n, speed, efficiency, k), 60);
        // Explanation:
        // We have the maximum performance of the team by selecting
        // engineer 2 (with speed=10 and efficiency=4)
        // and engineer 5 (with speed=5 and efficiency=7).
        // That is, performance = (10 + 5) * min(4, 7) = 60.
    }
    #[test]
    fn example2_n6_k3_produces_60() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 3;
        assert_eq!(Solution::max_performance(n, speed, efficiency, k), 68);
        // Explanation:
        // This is the same example as the first but k = 3.
        // We can select engineer 1, engineer 2 and engineer 5 to get the maximum performance of the team.
        // That is, performance = (2 + 10 + 5) * min(5, 4, 7) = 68.
    }
    #[test]
    fn example3_n6_k4_produces_60() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 4;
        assert_eq!(Solution::max_performance(n, speed, efficiency, k), 72);
    }

    #[test]
    fn n5_s_1_1_1_1_1_e_1_1_1_1_100_k5_produces_100() {
        assert_eq!(
            Solution::max_performance(5, vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 100], 5),
            100
        );
    }

    mod performance {
        use super::*;

        #[test]
        fn n100k_s_100k100k_e_100k100m_k100k_produces_49() {
            let n = 100_000;
            let speed = vec![100_000; n as usize];
            let efficiency = vec![100_000_000; n as usize];
            let k = n;
            assert_eq!(Solution::max_performance(n, speed, efficiency, k), 49);
        }
    }
}
