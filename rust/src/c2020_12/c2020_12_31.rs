#![allow(dead_code)]

struct Solution;

/// ### Largest Rectangle in Histogram
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = Vec::with_capacity(heights.len());
        let mut max_area = 0;

        for i in 0..heights.len() {
            while !stack.is_empty() && heights[*stack.last().unwrap()] >= heights[i] {
                let height = heights[stack.pop().unwrap()];
                let width = match stack.last() {
                    None => i as i32,
                    Some(&j) => (i - j - 1) as i32
                };
                max_area = max_area.max(height * width);
            }
            stack.push(i);
        }
        while let Some(i) = stack.pop() {
            let height = heights[i];
            let width = match stack.last() {
                None => heights.len() as i32,
                Some(&j) => (heights.len() - j - 1) as i32
            };
            max_area = max_area.max(height * width);
        }

        max_area
    }

    pub fn largest_rectangle_area_divide_and_conquer(heights: Vec<i32>) -> i32 {
        fn dac(hs: &[i32]) -> i32 {
            if hs.len() == 0 {
                0
            } else if hs.len() == 1 {
                hs[0]
            } else {
                let (m, i) =
                    hs.iter().enumerate()
                        .fold((i32::max_value(), 0),
                              |(min, oi), (i, &v)|
                                  if v < min { (v, i) } else { (min, oi) });
                (m * hs.len() as i32)
                    .max(dac(&hs[..i]))
                    .max(dac(&hs[i + 1..]))
            }
        }

        dac(&heights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_h215623_is_10() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn test63_h1_is_1() {
        assert_eq!(Solution::largest_rectangle_area(vec![1]), 1);
    }

    #[test]
    fn test68_h11_is_2() {
        assert_eq!(Solution::largest_rectangle_area(vec![1, 1]), 2);
    }

    #[test]
    fn hn_is_0() {
        assert_eq!(Solution::largest_rectangle_area(vec![]), 0);
    }
}