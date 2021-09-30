#![allow(dead_code)]

use rand::prelude::*;
use std::cell::RefCell;

/// # Generate Random Point in a Circle
///
/// Given the radius and x-y positions of the center of a circle,
/// write a function `randPoint` which generates a uniform random point in the circle.
///
/// Note:
///
/// 1. input and output values are in floating-point.
/// 2. radius and x-y position of the center of the circle is passed into the class constructor.
/// 3. a point on the circumference of the circle is considered to be in the circle.
/// 4. `randPoint` returns a size 2 array containing x-position and y-position of the random point,
///    in that order.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3675/
struct Solution {
    r: f64,
    cx: f64,
    cy: f64,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    shown: RefCell<Vec<Vec<f64>>>,
}
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            r: radius,
            cx: x_center,
            cy: y_center,
            x_min: x_center - radius,
            x_max: x_center + radius,
            y_min: y_center - radius,
            y_max: y_center + radius,
            shown: RefCell::new(Vec::new()),
        }
    }
    fn rand_point(&self) -> Vec<f64> {
        let mut p = self.next_point_in_square();
        while self.shown.borrow().contains(&p) || !self.within_circle(&p) {
            p = self.next_point_in_square();
        }
        self.shown.borrow_mut().push(p.clone());
        p
    }
    fn next_point_in_square(&self) -> Vec<f64> {
        let mut rng = thread_rng();
        let x = rng.gen_range(self.x_min..=self.x_max);
        let y = rng.gen_range(self.y_min..=self.y_max);
        vec![x, y]
    }
    fn within_circle(&self, p: &[f64]) -> bool {
        ((p[0] - self.cx).powi(2) + (p[1] - self.cy).powi(2)) <= self.r.powi(2)
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_r1x0y0() {
        let obj = Solution::new(1.0, 0.0, 0.0);
        let mut seen = Vec::new();
        for _ in 0..1 {
            let r = obj.rand_point();
            assert!(!seen.contains(&r));
            assert!(obj.within_circle(&r));
            seen.push(r);
        }
    }
    #[test]
    fn example2_r10x5ym7p5() {
        let obj = Solution::new(10.0, 5.0, -7.0);
        let mut seen = Vec::new();
        for _ in 0..100 {
            let r = obj.rand_point();
            assert!(!seen.contains(&r));
            assert!(obj.within_circle(&r));
            seen.push(r);
        }
    }
}
