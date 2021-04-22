#![allow(dead_code)]
/// Brick Wall
/// ==========
///
/// There is a brick wall in front of you.
/// The wall is rectangular and has several rows of bricks.
/// The bricks have the same height but different width.
/// You want to draw a vertical line from the __top__ to the __bottom__ and cross the __least__ bricks.
///
/// The brick wall is represented by a list of rows.
/// Each row is a list of integers representing the width of each brick in this row from left to right.
///
/// If your line go through the edge of a brick, then the brick is not considered as crossed.
/// You need to find out how to draw the line to cross the least bricks and return the number of crossed bricks.
///
/// __You cannot draw a line just along one of the two vertical edges of the wall,
/// in which case the line will obviously cross no bricks.__
///
/// __Note:__
///
/// 1. The width sum of bricks in different rows are the same and won't exceed INT_MAX.
/// 2. The number of bricks in each row is in range `[1,10_000]`.
///    The height of wall is in range `[1,10_000]`.
///    Total number of bricks of the wall won't exceed `20_000`.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/596/week-4-april-22nd-april-28th/3717/
struct Solution;
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        let height = wall.len() as i32;
        let mut borders_on_widths = HashMap::new();
        for r in wall {
            let mut wsf = 0;
            for brick in &r[..r.len() - 1] {
                wsf += brick;
                *borders_on_widths.entry(wsf).or_default() += 1;
            }
        }
        height - borders_on_widths.values().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_should_produce_2() {
        let wall = vec![
            vec![1, 2, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 4],
            vec![3, 1, 2],
            vec![1, 3, 1, 1],
        ];
        assert_eq!(Solution::least_bricks(wall), 2);
        // Explanation:
        // [1][  2 ][  |2 ][1]
        // [   3   ][1]|[  2 ]
        // [1][   3   ]|[  2 ]
        // [  2 ][     |4    ]
        // [   3   ][1]|[  2 ]
        // [1][   3   ]|[1][1]
    }

    #[test]
    fn test84() {
        let wall = vec![vec![2147483647]; 10000];
        assert_eq!(Solution::least_bricks(wall), 10000);
    }

    mod performance {
        use super::*;

        #[test]
        fn wall_h10000w2_produces_0() {
            let wall = (0..10000).map(|_| vec![1, 1]).collect();
            assert_eq!(Solution::least_bricks(wall), 0);
        }
        #[test]
        fn wall_h2w10000_produces_0() {
            let wall = (0..2).map(|_| vec![1; 10000]).collect();
            assert_eq!(Solution::least_bricks(wall), 0);
        }
        #[test]
        fn wall_h200w100_produces_100() {
            let wall = (0..200)
                .map(|r| {
                    if r % 2 == 0 {
                        let mut r = vec![2; 101];
                        r[0] = 1;
                        r[100] = 1;
                        r
                    } else {
                        vec![2; 100]
                    }
                })
                .collect();
            assert_eq!(Solution::least_bricks(wall), 100);
        }

        #[test]
        fn wall_h1000w1000_produces_0() {
            let wall = (0..1000).map(|_| vec![1; 1000]).collect();
            assert_eq!(Solution::least_bricks(wall), 0);
        }
    }
}
