#![allow(dead_code)]

/// ### Beautiful Arrangement
///
/// https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3591/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        fn is_beautiful(i: i32, j: i32) -> bool { i % j == 0 || j % i == 0 }

        fn see(seen: u16, j: i32) -> u16 { seen | (1 << (j - 1)) }

        fn backtrack(seen: u16, i: i32, n: i32) -> i32 {
            if i > n {
                1
            } else {
                let mut result = 0;
                for j in 1..=n {
                    if (seen >> (j - 1)) & 1 == 0 && is_beautiful(i, j) {
                        result += backtrack(see(seen, j), i + 1, n);
                    }
                }
                result
            }
        }

        backtrack(0, 1, n)
    }

    pub fn count_arrangement_rec_vec_bool(n: i32) -> i32 {
        fn is_beautiful(i: i32, j: i32) -> bool { i % j == 0 || j % i == 0 }

        fn backtrack(seen: &mut Vec<bool>, i: i32, n: i32) -> i32 {
            if i > n {
                1
            } else {
                let mut result = 0;
                for j in 0..seen.len() {
                    if !seen[j] && is_beautiful(i, (j + 1) as i32) {
                        seen[j] = true;
                        result += backtrack(seen, i + 1, n);
                        seen[j] = false;
                    }
                }
                result
            }
        }

        let mut seen = vec![false; n as usize];
        backtrack(&mut seen, 1, n)
    }
    pub fn count_arrangement_rec_vec_i32(n: i32) -> i32 {
        fn is_beautiful(v: i32, i: i32) -> bool { v % i == 0 || i % v == 0 }

        fn backtrack(nums: &[i32], i: i32, n: i32) -> i32 {
            if i > n {
                1
            } else {
                nums.iter()
                    .filter(|&&v| is_beautiful(v, i))
                    .map(|&v| {
                        let ns = nums.iter().filter(|&&n| n != v).map(|&n| n).collect::<Vec<_>>();
                        backtrack(&ns, i + 1, n)
                    }).sum()
            }
        }

        let nums = (1..=n).collect::<Vec<_>>();
        backtrack(&nums, 1, n)
    }
    pub fn count_arrangement_rec_brute_force(n: i32) -> i32 {
        fn is_beautiful_arr(arr: &[i32]) -> bool {
            arr.iter().enumerate().all(|(i, &v)| (i + 1) as i32 % v == 0 || v % (i + 1) as i32 == 0)
        }

        fn backtrack(nums: &[i32], arr: &mut Vec<i32>) -> i32 {
            if nums.is_empty() {
                if is_beautiful_arr(arr) { 1 } else { 0 }
            } else {
                let mut result = 0;
                for &n in nums {
                    arr.push(n);
                    let nns = nums.iter().filter(|&&nn| nn != n).map(|&n| n).collect::<Vec<_>>();
                    result += backtrack(&nns, arr);
                    arr.pop();
                }
                result
            }
        }

        let nums = (1..=n).collect::<Vec<_>>();
        backtrack(&nums, &mut Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example2_n1_is_1() {
        assert_eq!(Solution::count_arrangement(1), 1);
    }

    #[test]
    fn example1_n2_is_2() {
        assert_eq!(Solution::count_arrangement(2), 2);
        // Explanation:
        // The first beautiful arrangement is [1,2]:
        //     - perm[1] = 1 is divisible by i = 1
        //     - perm[2] = 2 is divisible by i = 2
        // The second beautiful arrangement is [2,1]:
        //     - perm[1] = 2 is divisible by i = 1
        //     - i = 2 is divisible by perm[2] = 1
    }

    #[test]
    fn n3_is_3() { assert_eq!(Solution::count_arrangement(3), 3); }

    #[test]
    fn n15_is_24679() { assert_eq!(Solution::count_arrangement(15), 24679); }

    // #[test]
    // fn test_generic_largest() {
    //     fn largest<T: PartialOrd>(list: &[T]) -> &T {
    //         let mut largest = &list[0];
    //
    //         for item in list {
    //             if item > largest {
    //                 largest = &item;
    //             }
    //         }
    //
    //         largest
    //     }
    //
    //     let number_list = vec![34, 50, 25, 100, 65];
    //
    //     let result = largest(&number_list);
    //     println!("The largest number is {}", result);
    //
    //     let char_list = vec!['y', 'm', 'a', 'q'];
    //
    //     let result = largest(&char_list);
    //     println!("The largest char is {}", result);
    // }
}
