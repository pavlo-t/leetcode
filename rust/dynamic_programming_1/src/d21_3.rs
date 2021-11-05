#![allow(dead_code)]
/// 279. Perfect Squares
/// ====================
///
/// Given an integer `n`, return _the least number of perfect square numbers that sum to `n`_.
///
/// A __perfect square__ is an integer that is the square of an integer;
/// in other words, it is the product of some integer with itself.
/// For example, `1`, `4`, `9`, and `16` are perfect squares while `3` and `11` are not.
///
/// __Constraints:__
///
/// - `1 <= n <= 10_000`
///
/// https://leetcode.com/problems/perfect-squares/
struct Solution;
impl Solution {
    /// Approach 5: Mathematics
    /// https://leetcode.com/problems/perfect-squares/solution/
    pub fn num_squares(mut n: i32) -> i32 {
        println!("num_squares({})", n);
        fn is_square(n: i32) -> bool {
            let sqrt = (n as f64).sqrt() as i32;
            n == sqrt * sqrt
        }

        // four-square and three-square theorems.
        while n % 4 == 0 {
            n /= 4;
        }
        if n % 8 == 7 {
            4
        } else if is_square(n) {
            1
        } else {
            // enumeration to check if the number can be decomposed into sum of two squares.
            for i in 1..=n {
                if is_square(n - i * i) {
                    return 2;
                }
            }
            // bottom case of three-square theorem.
            3
        }
    }

    /// Approach 4: Greedy + BFS (Breadth-First Search)
    /// https://leetcode.com/problems/perfect-squares/solution/
    pub fn num_squares_leetcode_greedy_bfs(n: i32) -> i32 {
        println!("num_squares({})", n);
        use std::collections::VecDeque;

        let square_numbers = (1..=i32::MAX)
            .map(|i| i * i)
            .take_while(|&i| i <= n)
            .collect::<Vec<i32>>();

        let mut queue = VecDeque::new();
        queue.push_back((n, 1));

        while let Some((remainder, level)) = queue.pop_front() {
            for &square in square_numbers.iter() {
                if square == remainder {
                    return level;
                } else if square > remainder {
                    break;
                } else {
                    queue.push_back((remainder - square, level + 1));
                }
            }
        }
        unreachable!();
    }

    /// Approach 3: Greedy Enumeration
    /// https://leetcode.com/problems/perfect-squares/solution/
    pub fn num_squares_leetcode_greedy(n: i32) -> i32 {
        println!("num_squares({})", n);
        use std::collections::HashSet;

        fn is_divided_by(n: i32, count: i32, sqs: &HashSet<i32>) -> bool {
            if count == 1 {
                sqs.contains(&n)
            } else {
                for &square in sqs.iter() {
                    if is_divided_by(n - square, count - 1, sqs) {
                        return true;
                    }
                }
                false
            }
        }

        let square_numbers = (1..=i32::MAX)
            .map(|i| i * i)
            .take_while(|&i| i <= n)
            .collect::<HashSet<i32>>();

        let mut count = 1;
        while count < n {
            if is_divided_by(n, count, &square_numbers) {
                return count;
            }
            count += 1;
        }
        count
    }

    /// 20:45-20:55
    pub fn num_squares_rec_with_memo_with_optimization(n: i32) -> i32 {
        println!("num_squares({})", n);
        fn rec(n: usize, memo: &mut Vec<i32>) -> i32 {
            if memo[n] >= 0 {
                memo[n]
            } else {
                memo[n] = match (n as f64).sqrt() as usize {
                    base if base * base == n => 1,
                    mut base => {
                        let mut square = base * base;
                        let mut result = i32::MAX;
                        while base > 0 {
                            result = result.min(1 + rec(n - square, memo));
                            if result == 2 {
                                break;
                            }
                            base -= 1;
                            square = base * base;
                        }
                        result
                    }
                };
                memo[n]
            }
        }
        let n = n as usize;
        let mut memo = vec![-1; n + 1];
        rec(n, &mut memo)
    }
    /// 20:35-20:45
    pub fn num_squares_dp_bottom_up_with_optimization(n: i32) -> i32 {
        println!("num_squares({})", n);
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[1] = 1;
        for n in 2..=n {
            let base = (n as f64).sqrt();
            dp[n] = match base.trunc() {
                base_trunc if base_trunc == base => 1,
                base_trunc => {
                    let mut base = base_trunc as usize;
                    let mut square = base * base;
                    let mut result = i32::MAX;
                    while square > 0 {
                        result = result.min(1 + dp[n - square]);
                        if result == 2 {
                            break;
                        }
                        base -= 1;
                        square = base * base;
                    }
                    result
                }
            };
        }
        dp[n]
    }
    /// 20:29-20:35
    pub fn num_squares_dp_bottom_up(n: i32) -> i32 {
        println!("num_squares({})", n);
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[1] = 1;
        for n in 2..=n {
            let mut base = 1;
            let mut square = 1;
            while square <= n {
                if n == square {
                    dp[n] = 1;
                    break;
                } else {
                    dp[n] = dp[n].min(1 + dp[n - square]);
                }
                base += 1;
                square = base * base;
            }
        }
        dp[n]
    }
    /// 20:23-20:29
    pub fn num_squares_rec_with_memo(n: i32) -> i32 {
        println!("num_squares({})", n);
        fn rec(n: usize, memo: &mut Vec<i32>) -> i32 {
            if memo[n] >= 0 {
                memo[n]
            } else {
                let mut base = 1;
                let mut square = 1;
                let mut result = i32::MAX;
                while square <= n {
                    if n == square {
                        result = 1;
                        break;
                    } else {
                        result = result.min(1 + rec(n - square, memo));
                    }
                    base += 1;
                    square = base * base;
                }
                memo[n] = result;
                result
            }
        }
        let n = n as usize;
        let mut memo = vec![-1; n + 1];
        rec(n, &mut memo)
    }
    /// 20:00-20:19
    pub fn num_squares_rec(n: i32) -> i32 {
        println!("num_squares({})", n);
        fn rec(n: i32) -> i32 {
            let mut base = 1;
            let mut square = 1;
            let mut result = i32::MAX;
            while square <= n {
                if n == square {
                    return 1;
                } else {
                    result = result.min(1 + rec(n - square));
                }
                base += 1;
                square = base * base;
            }
            result
        }
        rec(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]#[test]fn n1(){assert_eq!(Solution::num_squares(1),1);}
    #[rustfmt::skip]#[test]fn n2(){assert_eq!(Solution::num_squares(2),2);}
    #[rustfmt::skip]#[test]fn n3(){assert_eq!(Solution::num_squares(3),3);}
    #[rustfmt::skip]#[test]fn n4(){assert_eq!(Solution::num_squares(4),1);}
    #[rustfmt::skip]#[test]fn n5(){assert_eq!(Solution::num_squares(5),2);} //4+1
    #[rustfmt::skip]#[test]fn n6(){assert_eq!(Solution::num_squares(6),3);} //4+f(2)
    #[rustfmt::skip]#[test]fn n7(){assert_eq!(Solution::num_squares(7),4);} //4+f(3)
    #[rustfmt::skip]#[test]fn n8(){assert_eq!(Solution::num_squares(8),2);} //4+4
    #[rustfmt::skip]#[test]fn n9(){assert_eq!(Solution::num_squares(9),1);}
    #[rustfmt::skip]#[test]fn n10(){assert_eq!(Solution::num_squares(10),2);} //9+1
    #[rustfmt::skip]#[test]fn n11(){assert_eq!(Solution::num_squares(11),3);} //9+f(2)
    #[rustfmt::skip]#[test]fn n12(){assert_eq!(Solution::num_squares(12),3);} //4+4+4
    #[rustfmt::skip]#[test]fn n13(){assert_eq!(Solution::num_squares(13),2);} //9+4
    #[rustfmt::skip]#[test]fn n14(){assert_eq!(Solution::num_squares(14),3);} //9+4+1

    #[rustfmt::skip]#[test]fn  n1000(){assert_eq!(Solution::num_squares( 1000),2);} //100+900
    #[rustfmt::skip]#[test]fn  n9998(){assert_eq!(Solution::num_squares( 9998),3);}
    #[rustfmt::skip]#[test]fn  n9999(){assert_eq!(Solution::num_squares( 9999),4);}
    #[rustfmt::skip]#[test]fn n10000(){assert_eq!(Solution::num_squares(10000),1);}
}
