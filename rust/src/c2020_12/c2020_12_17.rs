#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut ab = HashMap::new();

        for i in 0..a.len() {
            for j in 0..b.len() {
                *ab.entry(a[i] + b[j]).or_default() += 1;
            }
        }

        let mut result = 0;

        for k in 0..c.len() {
            for l in 0..d.len() {
                result += ab.get(&-(c[k] + d[l])).unwrap_or(&0);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a1_2_bm2_m1_cm1_2_d0_2_should_be_2() {
        let a = vec![1, 2];
        let b = vec![-2, -1];
        let c = vec![-1, 2];
        let d = vec![0, 2];

        assert_eq!(Solution::four_sum_count(a, b, c, d), 2);
        //Explanation:
        //The two tuples are:
        //1. (0, 0, 0, 1) -> A[0] + B[0] + C[0] + D[1] = 1 + (-2) + (-1) + 2 = 0
        //2. (1, 1, 0, 0) -> A[1] + B[1] + C[0] + D[0] = 2 + (-1) + (-1) + 0 = 0
    }

    #[test]
    fn test6_am1_m1_bm1_1_cm1_1_d1_m1_should_be_6() {
        let a = vec![-1, -1];
        let b = vec![-1, 1];
        let c = vec![-1, 1];
        let d = vec![1, -1];

        assert_eq!(Solution::four_sum_count(a, b, c, d), 6);
    }

    #[test]
    fn a0b0c0d0_should_be_1() {
        let a = vec![0];
        let b = vec![0];
        let c = vec![0];
        let d = vec![0];

        assert_eq!(Solution::four_sum_count(a, b, c, d), 1);
    }

    #[test]
    fn a1to500_b1to500_c1to500_d1to500_should_be_0() {
        let a = (1..=500).collect();
        let b = (1..=500).collect();
        let c = (1..=500).collect();
        let d = (1..=500).collect();

        assert_eq!(Solution::four_sum_count(a, b, c, d), 0);
    }

    // #[test]
    // fn test_vector_capacity_allocation() {
    //     let mut v = Vec::new();
    //     println!("len: {}; capacity: {}", v.len(), v.capacity());
    //
    //     for i in 0..100 {
    //         v.push(i);
    //         println!("len: {}; capacity: {}", v.len(), v.capacity());
    //     }
    // }
    //
    // #[test]
    // fn test_string_capacity_allocation() {
    //     let mut v = String::new();
    //     println!("len: {}; capacity: {}", v.len(), v.capacity());
    //
    //     for i in 0..100 {
    //         v.push(std::char::from_digit(i % 10, 10).unwrap());
    //         println!("len: {}; capacity: {}", v.len(), v.capacity());
    //     }
    // }
}
