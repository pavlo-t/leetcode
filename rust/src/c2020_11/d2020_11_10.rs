#![allow(dead_code)]

struct Solution;

struct SolutionBuiltins;

// Builtins
impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        a.iter().map(|r| r.iter().rev().map(|i| i ^ 1).collect()).collect()
    }
}

impl SolutionBuiltins {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = a[0].len();
        let mut result = Vec::with_capacity(len);
        for r in 0..len {
            let mut new_row = vec![0; len];
            for i in 0..(len + 1) / 2 {
                new_row[i] = a[r][len - 1 - i] ^ 1;
                new_row[len - 1 - i] = a[r][i] ^ 1;
            }
            result.push(new_row);
        };
        result
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn example_1() {
        let a =
            vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        let expected =
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];

        assert_eq!(Solution::flip_and_invert_image(a), expected);
    }

    #[test]
    fn example_2() {
        let a =
            vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 1, 1, 1], vec![1, 0, 1, 0]];
        let expected =
            vec![vec![1, 1, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 1], vec![1, 0, 1, 0]];

        assert_eq!(Solution::flip_and_invert_image(a), expected);
    }
}
