#![allow(dead_code)]
/// 568. Maximum Vacation Days
/// ==========================
///
/// LeetCode wants to give one of its best employees the option to travel among `n` cities to collect algorithm problems.
/// But all work and no play makes Jack a dull boy, you could take vacations in some particular cities and weeks.
/// Your job is to schedule the traveling to maximize the number of vacation days you could take,
/// but there are certain rules and restrictions you need to follow.
///
/// Rules and restrictions:
///
/// 1. You can only travel among `n` cities, represented by indexes from `0` to `n - 1`.
///    Initially, you are in the city indexed `0` on __Monday__.
/// 2. The cities are connected by flights.
///    The flights are represented as an `n x n` matrix (not necessarily symmetrical),
///    called `flights` representing the airline status from the city `i` to the city `j`.
///    If there is no flight from the city `i` to the city `j`, `flights[i][j] == 0`;
///    Otherwise, `flights[i][j] == 1`. Also, `flights[i][i] == 0` for all `i`.
/// 3. You totally have `k` weeks (each week has __seven days__) to travel.
///    You can only take flights at most once per day and can only take flights on each week's Monday morning.
///    Since flight time is so short, we do not consider the impact of flight time.
/// 4. For each city, you can only have restricted vacation days in different weeks,
///    given an `n x k` matrix called `days` representing this relationship.
///    For the value of `days[i][j]`,
///    it represents the maximum days you could take a vacation in the city `i` in the week `j`.
/// 5. You could stay in a city beyond the number of vacation days,
///    but you should work on the extra days, which will not be counted as vacation days.
/// 6. If you fly from city `A` to city `B` and take the vacation on that day,
///    the deduction towards vacation days will count towards the vacation days of city `B` in that week.
/// 7. We do not consider the impact of flight hours on the calculation of vacation days.
///
/// Given the two matrices `flights` and `days`,
/// return _the maximum vacation days you could take during `k` weeks_.
///
/// __Constraints:__
///
/// - `n == flights.length`
/// - `n == flights[i].length`
/// - `n == days.length`
/// - `k == days[i].length`
/// - `1 <= n, k <= 100`
/// - `flights[i][j]` is either `0` or `1`.
/// - `0 <= days[i] <= 7`
///
/// https://leetcode.com/problems/maximum-vacation-days/
struct Solution;
impl Solution {
    pub fn max_vacation_days_brute_force(flights: Vec<Vec<i32>>, days: Vec<Vec<i32>>) -> i32 {
        println!("max_vacation_days({:?}, {:?})", flights, days);
        fn rec(c: usize, w: usize, fs: &[Vec<i32>], ds: &[Vec<i32>]) -> i32 {
            if w == ds[0].len() {
                0
            } else {
                (0..fs.len())
                    .filter(|&nc| fs[c][nc] == 1 || nc == c)
                    .map(|nc| ds[nc][w] + rec(nc, w + 1, fs, ds))
                    .max()
                    .unwrap()
            }
        }
        rec(0, 0, &flights, &days)
    }

    pub fn max_vacation_days_rec_with_memo(flights: Vec<Vec<i32>>, days: Vec<Vec<i32>>) -> i32 {
        println!("max_vacation_days({:?}, {:?})", flights, days);
        fn rec(
            c: usize,
            w: usize,
            fs: &[Vec<i32>],
            ds: &[Vec<i32>],
            memo: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if w == ds[0].len() {
                0
            } else if memo[c][w] != -1 {
                memo[c][w]
            } else {
                memo[c][w] = (0..fs.len())
                    .filter(|&nc| fs[c][nc] == 1 || nc == c)
                    .map(|nc| ds[nc][w] + rec(nc, w + 1, fs, ds, memo))
                    .max()
                    .unwrap();
                memo[c][w]
            }
        }

        let (n, k) = (days.len(), days[0].len());
        let mut memo = vec![vec![-1; k]; n];
        rec(0, 0, &flights, &days, &mut memo)
    }

    pub fn max_vacation_days_dp_2d(flights: Vec<Vec<i32>>, days: Vec<Vec<i32>>) -> i32 {
        let (n, k) = (days.len(), days[0].len());
        let mut dp = vec![vec![0; k + 1]; n + 1];
        for c in (0..n).rev() {
            for w in (0..k).rev() {
                for nc in (0..n).filter(|&nc| flights[c][nc] == 1 || nc == c) {
                    dp[c][w] = dp[c][w].max(days[nc][w] + dp[nc][w + 1]);
                }
            }
        }
        dp[0][0]
    }

    pub fn max_vacation_days(flights: Vec<Vec<i32>>, days: Vec<Vec<i32>>) -> i32 {
        let (n, k) = (days.len(), days[0].len());
        let mut prev = vec![0; n];
        let mut curr = vec![0; n];
        for w in (0..k).rev() {
            std::mem::swap(&mut curr, &mut prev);
            for c in (0..n).rev() {
                curr[c] = days[c][w] + prev[c];
                for nc in (0..n).filter(|&nc| flights[c][nc] == 1) {
                    curr[c] = curr[c].max(days[nc][w] + prev[nc]);
                }
            }
        }
        curr[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => {vec![$(vec!$x),*]}; }

    #[test]
    fn f_011_101_110_d_131_603_333() {
        let f = vv![[0, 1, 1], [1, 0, 1], [1, 1, 0]];
        let d = vv![[1, 3, 1], [6, 0, 3], [3, 3, 3]];
        assert_eq!(Solution::max_vacation_days(f, d), 12);
        // Explanation:
        // One of the best strategies is:
        // 1st week : fly from city 0 to city 1 on Monday, and play 6 days and work 1 day.
        // (Although you start at city 0, we could also fly to and start at other cities since it is Monday.)
        // 2nd week : fly from city 1 to city 2 on Monday, and play 3 days and work 4 days.
        // 3rd week : stay at city 2, and play 3 days and work 4 days.
        // Ans = 6 + 3 + 3 = 12.
    }
    #[test]
    fn f_000_000_000_d_111_777_777() {
        let f = vv![[0, 0, 0], [0, 0, 0], [0, 0, 0]];
        let d = vv![[1, 1, 1], [7, 7, 7], [7, 7, 7]];
        assert_eq!(Solution::max_vacation_days(f, d), 3);
        // Explanation:
        // Since there are no flights that enable you to move to another city, you have to stay at city 0 for the whole 3 weeks.
        // For each week, you only have one day to play and six days to work.
        // So the maximum number of vacation days is 3.
        // Ans = 1 + 1 + 1 = 3.
    }
    #[test]
    fn f_011_101_110_d_700_070_007() {
        let f = vv![[0, 1, 1], [1, 0, 1], [1, 1, 0]];
        let d = vv![[7, 0, 0], [0, 7, 0], [0, 0, 7]];
        assert_eq!(Solution::max_vacation_days(f, d), 21);
        // Explanation:
        // One of the best strategies is:
        // 1st week : stay at city 0, and play 7 days.
        // 2nd week : fly from city 0 to city 1 on Monday, and play 7 days.
        // 3rd week : fly from city 1 to city 2 on Monday, and play 7 days.
        // Ans = 7 + 7 + 7 = 21
    }

    #[test]
    fn f_1x100x100_d_1x100x100() {
        let f = vec![vec![1; 100]; 100];
        let d = vec![vec![1; 100]; 100];
        assert_eq!(Solution::max_vacation_days(f, d), 100);
    }
}
