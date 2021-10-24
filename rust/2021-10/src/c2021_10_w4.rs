#![allow(dead_code)]
/// 1066. Campus Bikes II
/// =====================
///
/// On a campus represented as a 2D grid, there are `n` workers and `m` bikes, with `n <= m`.
/// Each worker and bike is a 2D coordinate on this grid.
///
/// We assign one unique bike to each worker
/// so that the sum of the __Manhattan distances__ between each worker and their assigned bike is minimized.
///
/// Return _the minimum possible sum of __Manhattan distances__ between each worker and their assigned bike_.
///
/// The __Manhattan distance__ between two points `p1` and `p2` is `Manhattan(p1, p2) = |p1.x - p2.x| + |p1.y - p2.y|`.
///
/// __Constraints:__
///
/// - `1 <= workers.length <= bikes.length <= 10`
/// - `workers[i].length == bikes[i].length == 2`
/// - `0 <= workers[i][0], workers[i][1], bikes[i][0], bikes[i][1] < 1000
/// - All the workers and the bikes locations are __unique__.
///
/// https://leetcode.com/problems/campus-bikes-ii/
struct Solution;
impl Solution {
    /// Approach 4: Priority Queue (Similar to Dijkstra's Algorithm)
    /// https://leetcode.com/problems/campus-bikes-ii/solution/
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        println!("assign_bikes({:?}, {:?})", workers, bikes);
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashSet};
        fn dist(w: &[i32], b: &[i32]) -> i32 {
            (w[0] - b[0]).abs() + (w[1] - b[1]).abs()
        }

        let mut priority_queue = BinaryHeap::new();
        let mut visited: HashSet<i32> = HashSet::new();
        // Initially both distance sum and mask is 0
        priority_queue.push(Reverse((0, 0)));
        while let Some(Reverse((current_distance_sum, current_mask))) = priority_queue.pop() {
            // Continue if the mask is already traversed
            if visited.contains(&current_mask) {
                continue;
            }

            // Marking the mask as visited
            visited.insert(current_mask);
            // Next Worker index would be equal to the number of 1's in currentMask
            let worker_index = current_mask.count_ones() as usize;

            // Return the current distance sum if all workers are covered
            if worker_index == workers.len() {
                return current_distance_sum;
            }

            for bike_index in 0..bikes.len() {
                // Checking if the bike at bikeIndex has been assigned or not
                let bike_mask = 1 << bike_index;
                if current_mask & bike_mask == 0 {
                    let distance = dist(&workers[worker_index], &bikes[bike_index]);
                    let next_state_distance_sum = current_distance_sum + distance;
                    // Put the next state pair into the priority queue
                    let next_state_mask = current_mask | bike_mask;
                    priority_queue.push(Reverse((next_state_distance_sum, next_state_mask)));
                }
            }
        }
        // This statement will never be executed provided there is at least one bike per worker
        -1
    }
    /// Approach 3: Bottom-Up Dynamic Programming + BitMasking
    /// https://leetcode.com/problems/campus-bikes-ii/solution/
    pub fn assign_bikes_leetcode_dp(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        println!("assign_bikes({:?}, {:?})", workers, bikes);
        fn dist(w: &[i32], b: &[i32]) -> i32 {
            (w[0] - b[0]).abs() + (w[1] - b[1]).abs()
        }

        let mut dp = vec![i32::MAX; 1 << bikes.len()];
        // 0 signifies that no bike has been assigned and Distance sum for not assigning any bike is equal to 0
        dp[0] = 0;
        let mut smallest_distance_sum = i32::MAX;
        for mask in 0..dp.len() {
            let next_worker_index = mask.count_ones() as usize;
            // If mask has more number of 1's than the number of workers then we can update our answer accordingly
            if next_worker_index >= workers.len() {
                smallest_distance_sum = smallest_distance_sum.min(dp[mask]);
                continue;
            }

            for bike_index in 0..bikes.len() {
                // Checking if the bike at bike_index has already been assigned
                let bike_mask = 1 << bike_index;
                if mask & bike_mask == 0 {
                    let new_mask = mask | bike_mask;
                    let distance = dist(&workers[next_worker_index], &bikes[bike_index]);
                    dp[new_mask] = dp[new_mask].min(dp[mask] + distance);
                }
            }
        }
        smallest_distance_sum
    }
    /// 00:56-01:05
    pub fn assign_bikes_dp_vec_improved(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        println!("assign_bikes({:?}, {:?})", workers, bikes);
        fn dist(w: &[i32], b: &[i32]) -> i32 {
            (w[0] - b[0]).abs() + (w[1] - b[1]).abs()
        }

        let (wl, bl) = (workers.len(), bikes.len());
        let taken_len = 1 << bl;
        let mut dp = vec![0; taken_len];
        for w in (0..wl).rev() {
            let taken_min = (1 << w) - 1;
            for t in (taken_min..taken_len).filter(|t| t.count_ones() as usize == w) {
                let mut res = i32::MAX;
                for (b, bt) in (0..bl).map(|b| (b, 1 << b)).filter(|(_, bt)| bt & t == 0) {
                    res = res.min(dist(&workers[w], &bikes[b]).saturating_add(dp[t | bt]));
                }
                dp[t] = res;
            }
        }
        dp[0]
    }
    /// 22:43-22:46
    pub fn assign_bikes_dp_vec(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        println!("assign_bikes({:?}, {:?})", workers, bikes);
        fn dist(w: &[i32], b: &[i32]) -> i32 {
            (w[0] - b[0]).abs() + (w[1] - b[1]).abs()
        }

        let (wl, bl) = (workers.len(), bikes.len());
        let taken_len = 1 << bl;
        let mut dp = vec![0; taken_len];
        for w in (0..wl).rev() {
            for t in 0..taken_len {
                let mut res = i32::MAX;
                for b in 0..bl {
                    let bt = 1 << b;
                    if bt & t == 0 {
                        res = res.min(dist(&workers[w], &bikes[b]).saturating_add(dp[t | bt]));
                    }
                }
                dp[t] = res;
            }
        }
        dp[0]
    }
    /// 22:11-22:33
    pub fn assign_bikes_dp_vec_vec(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        println!("assign_bikes({:?}, {:?})", workers, bikes);
        fn dist(w: &[i32], b: &[i32]) -> i32 {
            (w[0] - b[0]).abs() + (w[1] - b[1]).abs()
        }

        let (wl, bl) = (workers.len(), bikes.len());
        let taken_len = 1 << bl;
        let mut dp = vec![vec![0; taken_len]; wl + 1];
        for w in (0..wl).rev() {
            for t in 0..taken_len {
                dp[w][t] = i32::MAX;
                for b in 0..bl {
                    let bt = 1 << b;
                    if bt & t == 0 {
                        let d = dist(&workers[w], &bikes[b]);
                        dp[w][t] = dp[w][t].min(d.saturating_add(dp[w + 1][t | bt]));
                    }
                }
            }
        }
        dp[0][0]
    }
    /// 22:06-22:11
    pub fn assign_bikes_rec_with_memo(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        println!("assign_bikes({:?}, {:?})", workers, bikes);
        fn dist(w: &[i32], b: &[i32]) -> i32 {
            (w[0] - b[0]).abs() + (w[1] - b[1]).abs()
        }
        #[rustfmt::skip]
        fn rec(i: usize, t: usize, ws: &[Vec<i32>], bs: &[Vec<i32>], memo: &mut Vec<Vec<i32>>) -> i32 {
            if i == ws.len() {
                0
            } else if memo[i][t] >= 0 {
                memo[i][t]
            } else {
                let mut result = i32::MAX;
                for j in 0..bs.len() {
                    let tj = 1 << j;
                    if tj & t == 0 {
                        result = result.min(dist(&ws[i], &bs[j]) + rec(i + 1, t | tj, ws, bs, memo));
                    }
                }
                memo[i][t] = result;
                result
            }
        }
        let mut memo = vec![vec![-1; 1 << bikes.len()]; workers.len()];
        rec(0, 0, &workers, &bikes, &mut memo)
    }
    /// 22:00-22:06
    pub fn assign_bikes_rec(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        println!("assign_bikes({:?}, {:?})", workers, bikes);
        fn dist(w: &[i32], b: &[i32]) -> i32 {
            (w[0] - b[0]).abs() + (w[1] - b[1]).abs()
        }
        fn rec(i: usize, taken: i32, ws: &[Vec<i32>], bs: &[Vec<i32>]) -> i32 {
            if i == ws.len() {
                0
            } else {
                let mut result = i32::MAX;
                for j in 0..bs.len() {
                    let tj = 1 << j;
                    if tj & taken == 0 {
                        result = result.min(dist(&ws[i], &bs[j]) + rec(i + 1, taken | tj, ws, bs));
                    }
                }
                result
            }
        }
        rec(0, 0, &workers, &bikes)
    }
    /// 21:35-21:56
    pub fn assign_bikes_backtracking_recursion(
        workers: Vec<Vec<i32>>,
        bikes: Vec<Vec<i32>>,
    ) -> i32 {
        println!("assign_bikes({:?}, {:?})", workers, bikes);
        fn dist(w: &[i32], b: &[i32]) -> i32 {
            (w[0] - b[0]).abs() + (w[1] - b[1]).abs()
        }
        fn bts(i: usize, ws: &[Vec<i32>], bs: &[Vec<i32>], taken: &mut Vec<bool>) -> i32 {
            if i == ws.len() {
                0
            } else {
                let mut result = i32::MAX;
                for j in 0..bs.len() {
                    if !taken[j] {
                        taken[j] = true;
                        result = result.min(dist(&ws[i], &bs[j]) + bts(i + 1, ws, bs, taken));
                        taken[j] = false;
                    }
                }
                result
            }
        }
        bts(0, &workers, &bikes, &mut vec![false; bikes.len()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn w_00_b_12() {
        let w = vv![[0, 0]];
        let b = vv![[1, 2]];
        assert_eq!(Solution::assign_bikes(w, b), 3);
    }
    #[test]
    fn w_00_21_b_12_33() {
        let w = vv![[0, 0], [2, 1]];
        let b = vv![[1, 2], [3, 3]];
        // [00,11] [01,10]
        assert_eq!(Solution::assign_bikes(w, b), 6);
        // Explanation: We assign bike 0 to worker 0, bike 1 to worker 1.
        // The Manhattan distance of both assignments is 3, so the output is 6.
    }
    #[test]
    fn w_00_11_20_b_10_22_21() {
        let w = vv![[0, 0], [1, 1], [2, 0]];
        let b = vv![[1, 0], [2, 2], [2, 1]];
        // [00,11,22] [00,12,21] [01,10,22] [01,12,20] [12,10,21] [12,11,20]
        assert_eq!(Solution::assign_bikes(w, b), 4);
        // Explanation: We first assign bike 0 to worker 0,
        // then assign bike 1 to worker 1 or worker 2, bike 2 to worker 2 or worker 1.
        // Both assignments lead to sum of the Manhattan distances as 4.
    }
    #[test]
    fn w_00_10_20_30_40_b_0c999_1c999_2c999_3c999_4c999() {
        let w = vv![[0, 0], [1, 0], [2, 0], [3, 0], [4, 0]];
        let b = vv![[0, 999], [1, 999], [2, 999], [3, 999], [4, 999]];
        assert_eq!(Solution::assign_bikes(w, b), 4995);
    }

    #[test]
    fn w_00to99_b_00to99() {
        let w = (0..=9).map(|i| vec![i, i]).collect();
        let b = (0..=9).map(|i| vec![i, i]).collect();
        assert_eq!(Solution::assign_bikes(w, b), 0);
    }
}
