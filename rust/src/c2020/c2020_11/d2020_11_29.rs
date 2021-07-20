// @formatter:off
#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut seen = vec![false; arr.len()];
        let mut todo = Vec::with_capacity(arr.len() * 2);
        todo.push(start);

        while let Some(i) = todo.pop() {
            if i >= 0 && (i as usize) < arr.len() && !seen[i as usize] {
                let v = arr[i as usize];
                if v == 0 { return true }
                seen[i as usize] = true;
                todo.push(i + v);
                todo.push(i - v);
            }
        }
        false
    }
}

struct Solution1;
impl Solution1 {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        use std::collections::HashSet;

        let mut todo = Vec::with_capacity(arr.len() * 2);
        let mut seen = HashSet::with_capacity(arr.len());
        todo.push(start);

        while let Some(i) = todo.pop() {
            if i >= 0 && (i as usize) < arr.len() && !seen.contains(&i) {
                let v = arr[i as usize];
                if v == 0 { return true }
                seen.insert(i);
                todo.push(i + v);
                todo.push(i - v);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn example1_arr4_2_3_0_3_1_2_start5_is_true() {
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
        //Explanation:
        //All possible ways to reach at index 3 with value 0 are:
        //  index 5 -> index 4 -> index 1 -> index 3
        //  index 5 -> index 6 -> index 4 -> index 1 -> index 3
    }
    #[test]
    fn example2_arr4_2_3_0_3_1_2_start0_is_true() {
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0));
        //Explanation:
        //One possible way to reach at index 3 with value 0 is:
        //  index 0 -> index 4 -> index 1 -> index 3
    }
    #[test]
    fn example3_arr3_0_2_1_2_start2_is_false() {
        assert!(!Solution::can_reach(vec![3, 0, 2, 1, 2], 2));
        //Explanation: There is no way to reach at index 1 with value 0.
    }

    #[test]
    fn arr_50000_1s_start0_is_false() {
        assert!(!Solution::can_reach(vec![1; 50000], 0));
    }
    #[test]
    fn arr_50000_1s_start49999_is_false() {
        assert!(!Solution::can_reach(vec![1; 50000], 49999));
    }
    #[test]
    fn arr0_49999_1s_start49999_is_true() {
        let mut arr = vec![1; 50000];
        arr[0] = 0;
        assert!(Solution::can_reach(arr, 49999));
    }
    #[test]
    fn arr_49999_1s_0_start0_is_true() {
        let mut arr = vec![1; 50000];
        arr[49999] = 0;
        assert!(Solution::can_reach(arr, 0));
    }

    #[test]
    fn arr_50000_randoms_start_random_is_true() {
        let mut rng = rand::thread_rng();
        let mut arr = Vec::with_capacity(50000);
        for _ in 0..50000 { arr.push(rng.gen_range(0..=50000)); }

        Solution::can_reach(arr, rng.gen_range(0..50000));
    }
}
