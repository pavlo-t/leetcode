#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut points: Vec<(i32, i32)> =
            [p1, p2, p3, p4].iter().map(|p| (p[0], p[1])).collect();
        points.sort();

        for w in points.windows(2) {
            if w[0] == w[1] { return false; }
        }

        let (x1, y1) = points[0];
        let (x2, y2) = points[1];
        let p3 = points[2];
        let p4 = points[3];

        let dx = x2 - x1;
        let dy = y2 - y1;

        if dy < 0 {
            p3 == (x1 - dy, y1 + dx) && p4 == (x2 - dy, y2 + dx)
        } else {
            p3 == (x1 + dy, y1 - dx) && p4 == (x2 + dy, y2 - dx)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_square() {
        assert_eq!(Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]), true);
    }

    #[test]
    fn test_60_degrees_turned_square() {
        assert_eq!(Solution::valid_square(vec![0, 2], vec![1, 0], vec![-2, 1], vec![-1, -1]), true);
    }

    #[test]
    fn test_30_degrees_turned_square() {
        assert_eq!(Solution::valid_square(vec![3, 2], vec![1, 3], vec![2, 0], vec![0, 1]), true);
    }

    #[test]
    fn test_equal_points() {
        assert_eq!(Solution::valid_square(vec![0, 0], vec![0, 0], vec![0, 0], vec![0, 0]), false);
    }

    #[test]
    fn test_not_a_square() {
        assert_eq!(Solution::valid_square(vec![0, 0], vec![0, 1], vec![2, 0], vec![2, 1]), false);
    }
}