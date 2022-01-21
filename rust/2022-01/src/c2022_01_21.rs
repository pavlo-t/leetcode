#![allow(dead_code)]
/// 134. Gas Station
/// ================
///
/// There are `n` gas stations along a circular route, where the amount of gas at the `i`th station is `gas[i]`.
///
/// You have a car with an unlimited gas tank and
/// it costs `cost[i]` of gas to travel from the `i`th station to its next `(i + 1)`th station.
/// You begin the journey with an empty tank at one of the gas stations.
///
/// Given two integer arrays `gas` and `cost`, return
/// _the starting gas station's index if you can travel around the circuit once in the clockwise direction,
/// otherwise return `-1`_.
/// If there exists a solution, it is __guaranteed__ to be __unique__.
///
/// __Constraints:__
///
/// - `gas.length == n`
/// - `cost.length == n`
/// - `1 <= n <= 100_000`
/// - `0 <= gas[i], cost[i] <= 10_000`
///
/// https://leetcode.com/problems/gas-station/
struct Solution;
impl Solution {
    pub fn can_complete_circuit_brute_force(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        fn can_complete_from(start: usize, gas: &[i32], cost: &[i32]) -> bool {
            let n = gas.len();
            let mut g = 0;
            for i in (0..n)
                .map(|i| i + start)
                .map(|i| if i < n { i } else { i - n })
            {
                g += gas[i] - cost[i];
                if g < 0 {
                    return false;
                }
            }
            true
        }

        for start in 0..gas.len() {
            if can_complete_from(start, &gas, &cost) {
                return start as i32;
            }
        }
        -1
    }

    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut total_gas = 0;
        let mut total_cost = 0;
        let mut max_i = 0;
        let mut max_gas = 0;
        for i in (0..n).rev() {
            total_gas += gas[i];
            total_cost += cost[i];
            let gas_available = total_gas - total_cost;
            if max_gas <= gas_available {
                max_gas = gas_available;
                max_i = i;
            }
        }
        if total_gas < total_cost {
            -1
        } else {
            max_i as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g_12345_c_51234() {
        let g = vec![1, 2, 3, 4, 5];
        let c = vec![5, 1, 2, 3, 4];
        //         [-4, 1, 1, 1, 1]
        //         [-4,-3,-2,-1, 0]
        //         [ 0, 4, 3, 2, 1]
        assert_eq!(Solution::can_complete_circuit(g, c), 1);
    }
    #[test]
    fn g_12345_c_51342() {
        let g = vec![1, 2, 3, 4, 5];
        let c = vec![5, 1, 3, 4, 2];
        //         [-4, 1, 0, 0, 3]
        //         [-4,-3,-3,-3, 0]
        //         [ 0, 4, 3, 3, 3]
        assert_eq!(Solution::can_complete_circuit(g, c), 1);
    }
    #[test]
    fn g_12345_c_34512() {
        let g = vec![1, 2, 3, 4, 5];
        let c = vec![3, 4, 5, 1, 2];
        //         [-2,-2,-2, 3, 3]
        //         [-2,-4,-6,-3, 0]
        //         [ 0, 2, 4, 6, 3]
        //
        //         [ 0,-2,-4,-6,-3,0]
        //         [ 0,-2,-4,-6,-3,0]
        assert_eq!(Solution::can_complete_circuit(g, c), 3);
        // Explanation:
        // Start at station 3 (index 3) and fill up with 4 unit of gas. Your tank = 0 + 4 = 4
        // Travel to station 4. Your tank = 4 - 1 + 5 = 8
        // Travel to station 0. Your tank = 8 - 2 + 1 = 7
        // Travel to station 1. Your tank = 7 - 3 + 2 = 6
        // Travel to station 2. Your tank = 6 - 4 + 3 = 5
        // Travel to station 3. The cost is 5. Your gas is just enough to travel back to station 3.
        // Therefore, return 3 as the starting index.
    }
    #[test]
    fn g_234_c_343() {
        let g = vec![2, 3, 4];
        let c = vec![3, 4, 3];
        assert_eq!(Solution::can_complete_circuit(g, c), -1);
        // Explanation:
        // You can't start at station 0 or 1, as there is not enough gas to travel to the next station.
        // Let's start at station 2 and fill up with 4 unit of gas. Your tank = 0 + 4 = 4
        // Travel to station 0. Your tank = 4 - 3 + 2 = 3
        // Travel to station 1. Your tank = 3 - 3 + 3 = 3
        // You cannot travel back to station 2, as it requires 4 unit of gas but you only have 3.
        // Therefore, you can't travel around the circuit once no matter where you start.
    }

    #[test]
    fn g_12345_c_12354() {
        let g = vec![1, 2, 3, 4, 5];
        let c = vec![1, 2, 3, 5, 4];
        assert_eq!(Solution::can_complete_circuit(g, c), 4);
    }
    #[test]
    fn g_12345_c_24351() {
        let g = vec![1, 2, 3, 4, 5];
        let c = vec![2, 4, 3, 5, 1];
        assert_eq!(Solution::can_complete_circuit(g, c), 4);
    }

    //#[ignore]
    #[test]
    fn g_1to10000_repeat_10_c_1to99998_chain_100000_chain_99999() {
        let g = (1..=10000).cycle().take(100000).collect();
        let mut c: Vec<i32> = (1..=10000).cycle().take(100000).collect();
        c[99998] = 10000;
        c[99999] = 9999;
        assert_eq!(Solution::can_complete_circuit(g, c), 99999);
    }
    //#[ignore]
    #[test]
    fn g_99999x0chain1_c_99998x0_chain_1_chain_0() {
        let mut g = vec![0; 100000];
        let mut c = vec![0; 100000];
        g[99999] = 1;
        c[99998] = 1;
        assert_eq!(Solution::can_complete_circuit(g, c), 99999);
    }
}
