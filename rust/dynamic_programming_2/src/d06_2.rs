#![allow(dead_code)]
/// 1230. Toss Strange Coins
/// ========================
///
/// You have some coins.
/// The `i`-th coin has a probability `prob[i]` of facing heads when tossed.
///
/// Return the probability that the number of coins facing heads equals `target` if you toss every coin exactly once.
///
/// Constraints:
///
/// - `1 <= prob.length <= 1000`
/// - `0 <= prob[i] <= 1`
/// - `0 <= target <= prob.length`
/// - Answers will be accepted as correct if they are within `10**-5` of the correct answer.
///
/// https://leetcode.com/problems/toss-strange-coins/
struct Solution;
impl Solution {
    /// 22:27-23:32
    pub fn probability_of_heads_rec(prob: Vec<f64>, target: i32) -> f64 {
        println!("probability_of_heads({:?}, {})", prob, target);
        fn rec(i: usize, t: usize, ps: &[f64]) -> f64 {
            if i == ps.len() {
                match t {
                    0 => 1.0,
                    _ => 0.0,
                }
            } else if t == 0 {
                (1.0 - ps[i]) * rec(i + 1, t, ps)
            } else {
                ps[i] * rec(i + 1, t - 1, ps) + (1.0 - ps[i]) * rec(i + 1, t, ps)
            }
        }
        rec(0, target as usize, &prob)
    }
    /// 23:32-23:40
    pub fn probability_of_heads_rec_with_memo(prob: Vec<f64>, target: i32) -> f64 {
        println!("probability_of_heads({:?}, {})", prob, target);
        fn rec(i: usize, t: usize, ps: &[f64], memo: &mut Vec<Vec<f64>>) -> f64 {
            if memo[i][t] != -1.0 {
                memo[i][t]
            } else {
                memo[i][t] = if i == ps.len() {
                    match t {
                        0 => 1.0,
                        _ => 0.0,
                    }
                } else if t == 0 {
                    (1.0 - ps[i]) * rec(i + 1, t, ps, memo)
                } else {
                    ps[i] * rec(i + 1, t - 1, ps, memo) + (1.0 - ps[i]) * rec(i + 1, t, ps, memo)
                };
                memo[i][t]
            }
        }
        let t = target as usize;
        let mut memo = vec![vec![-1.0; t + 1]; prob.len() + 1];
        rec(0, t, &prob, &mut memo)
    }
    /// 23:40-23:48
    pub fn probability_of_heads_dp_vec_vec(prob: Vec<f64>, target: i32) -> f64 {
        println!("probability_of_heads({:?}, {})", prob, target);
        let (n, t) = (prob.len(), target as usize);
        let mut dp = vec![vec![0.0; t + 1]; n + 1];
        dp[n][0] = 1.0;
        for i in (0..n).rev() {
            let (heads, eagles) = (prob[i], 1.0 - prob[i]);
            dp[i][0] = eagles * dp[i + 1][0];
            for t in 1..=t {
                dp[i][t] = heads * dp[i + 1][t - 1] + eagles * dp[i + 1][t];
            }
        }
        dp[0][t]
    }
    /// 23:48-23:54
    pub fn probability_of_heads(prob: Vec<f64>, target: i32) -> f64 {
        println!("probability_of_heads({:?}, {})", prob, target);
        let target = target as usize;
        let mut dp = vec![0.0; target + 1];
        dp[0] = 1.0;
        for (i, heads) in prob.into_iter().enumerate() {
            let eagles = 1.0 - heads;
            for t in (1..=target.min(i + 1)).rev() {
                dp[t] = heads * dp[t - 1] + eagles * dp[t];
            }
            dp[0] *= eagles;
        }
        dp[target]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `10**-5`
    const MARGIN: f64 = 0.00001;

    fn check(p: Vec<f64>, t: i32, e: f64) {
        let r = Solution::probability_of_heads(p, t);
        assert!((r - e).abs() < MARGIN, "result != expected: {} != {}", r, e);
    }

    #[rustfmt::skip] #[test] fn p_p4_t_0() { check(vec![0.4], 0, 0.6); }
    #[rustfmt::skip] #[test] fn p_p4_t_1() { check(vec![0.4], 1, 0.4); }

    #[rustfmt::skip] #[test] fn p_p4p4_t_0() {check(vec![0.4,0.4], 0, 0.36); } // .6*.6
    #[rustfmt::skip] #[test] fn p_p4p4_t_1() {check(vec![0.4,0.4], 1, 0.48); } // .6*.4+.4*.6
    #[rustfmt::skip] #[test] fn p_p4p4_t_2() {check(vec![0.4,0.4], 2, 0.16); } // .4*.4

    #[rustfmt::skip] #[test] fn p_p4p4p4_t_0() {check(vec![0.4,0.4,0.4],0,0.216); } // .6*.6*.6
    #[rustfmt::skip] #[test] fn p_p4p4p4_t_1() {check(vec![0.4,0.4,0.4],1,0.432); } // .4*.6*.6 + .6*.4*.6 + .6*.6*.4
    #[rustfmt::skip] #[test] fn p_p4p4p4_t_2() {check(vec![0.4,0.4,0.4],2,0.288); } // .4*.4*.6 + .4*.6*.4 + .6*.4*.4
    #[rustfmt::skip] #[test] fn p_p4p4p4_t_3() {check(vec![0.4,0.4,0.4],3,0.064); } // .4*.4*.4

    #[rustfmt::skip] #[test] fn p_p1p2p3_t_0() {check(vec![0.1,0.2,0.3],0,0.504); } // .9*.8*.7
    #[rustfmt::skip] #[test] fn p_p1p2p3_t_1() {check(vec![0.1,0.2,0.3],1,0.398); } // .1*.8*.7 + .9*.2*.7 + .9*.8*.3
    #[rustfmt::skip] #[test] fn p_p1p2p3_t_2() {check(vec![0.1,0.2,0.3],2,0.092); } // .1*.2*.7 + .1*.8*.3 + .9*.2*.3
    #[rustfmt::skip] #[test] fn p_p1p2p3_t_3() {check(vec![0.1,0.2,0.3],3,0.006); } // .1*.2*.3

    #[rustfmt::skip] #[test] fn p_p4p4p4p4_t_0() {check(vec![0.4,0.4,0.4,0.4],0,0.1296); } // .6*.6*.6*.6
    #[rustfmt::skip] #[test] fn p_p4p4p4p4_t_1() {check(vec![0.4,0.4,0.4,0.4],1,0.3456); } // .4*.6*.6*.6 * 4
    #[rustfmt::skip] #[test] fn p_p4p4p4p4_t_2() {check(vec![0.4,0.4,0.4,0.4],2,0.3456); } // .4*.6*.6*.6 * 4
    #[rustfmt::skip] #[test] fn p_p4p4p4p4_t_3() {check(vec![0.4,0.4,0.4,0.4],3,0.1536); } // .4*.4*.4*.6 * 4
    #[rustfmt::skip] #[test] fn p_p4p4p4p4_t_4() {check(vec![0.4,0.4,0.4,0.4],4,0.0256); } // .4*.4*.4*.4

    #[rustfmt::skip] #[test] fn p_p5p5p5p5p5_t_0(){check(vec![0.5,0.5,0.5,0.5,0.5],0,0.03125);} // .5**5
    #[rustfmt::skip] #[test] fn p_p5p5p5p5p5_t_1(){check(vec![0.5,0.5,0.5,0.5,0.5],1,0.15625);} //
    #[rustfmt::skip] #[test] fn p_p5p5p5p5p5_t_2(){check(vec![0.5,0.5,0.5,0.5,0.5],2,0.31250);} //
    #[rustfmt::skip] #[test] fn p_p5p5p5p5p5_t_3(){check(vec![0.5,0.5,0.5,0.5,0.5],3,0.31250);} //
    #[rustfmt::skip] #[test] fn p_p5p5p5p5p5_t_4(){check(vec![0.5,0.5,0.5,0.5,0.5],4,0.15625);} //
    #[rustfmt::skip] #[test] fn p_p5p5p5p5p5_t_5(){check(vec![0.5,0.5,0.5,0.5,0.5],5,0.03125);}

    #[test]
    fn p_p5_repeat_1000_t_500() {
        check(vec![0.5; 1000], 500, 0.02522);
    }
}
