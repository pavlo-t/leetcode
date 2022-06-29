#![allow(dead_code)]
//! \#406. Queue Reconstruction by Height
//! =====================================
//!
//! You are given an array of people, `people`, which are the attributes of some people in a queue
//! (not necessarily in order).
//! Each `people[i] = [hi, ki]` represents the `i`th person of height `hi` with __exactly__ `ki`
//! other people in front who have a height greater than or equal to `hi`.
//!
//! Reconstruct and return _the queue that is represented by the input array `people`_.
//! The returned queue should be formatted as an array `queue`,
//! where `queue[j] = [hj, kj]` is the attributes of the `j`th person in the queue
//! (`queue[0]` is the person at the front of the queue).
//!
//! Constraints:
//!
//! - `1 <= people.length <= 2000`
//! - `0 <= hi <= 1_000_000`
//! - `0 <= ki < people.length`
//! - It is guaranteed that the queue can be reconstructed.
//!
//! <https://leetcode.com/problems/queue-reconstruction-by-height>

pub struct Solution;
impl Solution {
    /// using `Vec<Option<Vec<i32>>>`
    pub fn reconstruct_queue_my_v1(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable();
        let mut result: Vec<Option<Vec<i32>>> = vec![None; people.len()];
        for p in people {
            let mut same_or_higher = 0;
            for i in 0..result.len() {
                if result[i].is_none() && same_or_higher == p[1] {
                    result[i] = Some(p);
                    break;
                }
                // += 1 if is not lower
                same_or_higher += result[i].as_ref().filter(|o| o[0] < p[0]).is_none() as i32;
            }
        }
        result.into_iter().map(|r| r.unwrap()).collect()
    }

    /// using empty `Vec<i32>` to indicate unset values
    pub fn reconstruct_queue_my_v2(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable();
        let mut result: Vec<Vec<i32>> = vec![vec![]; people.len()];
        for p in people {
            let mut same_or_higher = 0;
            for i in 0..result.len() {
                if result[i].is_empty() && same_or_higher == p[1] {
                    result[i] = p;
                    break;
                }
                same_or_higher += (result[i].is_empty() || result[i][0] >= p[0]) as i32;
            }
        }
        result
    }

    /// <https://leetcode.com/problems/queue-reconstruction-by-height/solution>
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by_key(|v| (-v[0], v[1]));
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(people.len());
        for p in people {
            result.insert(p[1] as usize, p);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn p_h1b0() {
        let p = vv![[1, 0]];
        let e = vv![[1, 0]];
        assert_eq!(Solution::reconstruct_queue(p), e);
    }
    #[test]
    fn p_h7b0_h4b4_h7b1_h5b0_h6b1_h5b2() {
        let p = vv![[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]];
        let e = vv![[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]];
        assert_eq!(Solution::reconstruct_queue(p), e);
        // [(), (), (), (), (), ()]
        // [(), (), (), (), (4,4), ()]
        // [(5,0), (), (), (), (4,4), ()]
        // [(5,0), (), (5,2), (), (4,4), ()]
        // [(5,0), (), (5,2), (6,1), (4,4), ()]
        // [(5,0), (7,0), (5,2), (6,1), (4,4), ()]
        // [(5,0), (7,0), (5,2), (6,1), (4,4), (7,1)]
        // Explanation:
        // Person 0 has height 5 with no other people taller or the same height in front.
        // Person 1 has height 7 with no other people taller or the same height in front.
        // Person 2 has height 5 with two persons taller or the same height in front, which is person 0 and 1.
        // Person 3 has height 6 with one person taller or the same height in front, which is person 1.
        // Person 4 has height 4 with four people taller or the same height in front, which are people 0, 1, 2, and 3.
        // Person 5 has height 7 with one person taller or the same height in front, which is person 1.
        // Hence [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]] is the reconstructed queue.
    }
    #[test]
    fn p_h6b0_h5b0_h4b0_h3b2_h2b2_h1b4() {
        let p = vv![[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]];
        let e = vv![[4, 0], [5, 0], [2, 2], [3, 2], [1, 4], [6, 0]];
        assert_eq!(Solution::reconstruct_queue(p), e);
        // [(), (), (), (), (), ()]
        // [(), (), (), (), (1,4), ()]
        // [(), (), (2,2), (), (1,4), ()]
        // [(), (), (2,2), (3,2), (1,4), ()]
        // [(4,0), (), (2,2), (3,2), (1,4), ()]
        // [(4,0), (5,0), (2,2), (3,2), (1,4), ()]
        // [(4,0), (5,0), (2,2), (3,2), (1,4), (6,0)]
    }
}
