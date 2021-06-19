#![allow(dead_code)]

use std::cell::RefCell;

/// Range Sum Query - Mutable
/// =========================
///
/// Given an integer array `nums`, handle multiple queries of the following types:
///
/// 1. __Update__ the value of an element in `nums`.
/// 2. Calculate the __sum__ of the elements of `nums` between indices
///    `left` and `right` __inclusive__ where `left <= right`.
///
/// Implement the `NumArray` class:
///
/// - `NumArray(int[] nums)` Initializes the object with the integer array `nums`.
/// - `void update(int index, int val)` __Updates__ the value of `nums[index]` to be `val`.
/// - `int sumRange(int left, int right)` Returns the __sum__ of the elements of `nums` between
///   indices `left` and `right` inclusive (i.e. `nums[left] + nums[left + 1] + ... + nums[right]`).
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 30_000`
/// - `-100 <= nums[i] <= 100`
/// - `0 <= index < nums.length`
/// - `-100 <= val <= 100`
/// - `0 <= left <= right < nums.length`
/// - At most `30_000` calls will be made to `update` and `sumRange`.
struct NumArray {
    ns: RefCell<Vec<i32>>,
    bt: RefCell<Vec<i32>>,
}
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut bt = vec![0; nums.len() + 1];
        for (i, &v) in nums.iter().enumerate() {
            add(&mut bt, i + 1, v);
        }
        let bt = RefCell::new(bt);
        let ns = RefCell::new(nums);
        Self { ns, bt }
    }

    fn update(&self, index: i32, val: i32) {
        let i = index as usize;
        let mut bt = self.bt.borrow_mut();
        add(&mut bt, i + 1, val - self.ns.borrow()[i]);
        self.ns.borrow_mut()[i] = val;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum(right as usize + 1) - self.sum(left as usize)
    }

    fn sum(&self, mut i: usize) -> i32 {
        let mut sum = 0;
        let bt = self.bt.borrow();
        while i >= 1 {
            sum += bt[i];
            let ii = i as i32;
            i -= (ii & (-ii)) as usize
        }
        sum
    }
}
fn add(bt: &mut Vec<i32>, mut i: usize, v: i32) {
    while i < bt.len() {
        bt[i] += v;
        let ii = i as i32;
        i = i + (ii & (-ii)) as usize;
    }
}

struct NumArrayBruteForce {
    ns: RefCell<Vec<i32>>,
}
impl NumArrayBruteForce {
    fn new(nums: Vec<i32>) -> Self {
        let ns = RefCell::new(nums);
        Self { ns }
    }

    fn update(&self, index: i32, val: i32) {
        self.ns.borrow_mut()[index as usize] = val;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let ns = self.ns.borrow();
        (left as usize..=right as usize).fold(0, |rsf, i| rsf + ns[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let na = NumArray::new(vec![1, 3, 5]);
        assert_eq!(na.sum_range(0, 2), 9);
        na.update(1, 2);
        assert_eq!(na.sum_range(0, 2), 8);
        // Explanation
        // NumArray numArray = new NumArray([1, 3, 5]);
        // numArray.sumRange(0, 2); // return 1 + 3 + 5 = 9
        // numArray.update(1, 2);   // nums = [1, 2, 5]
        // numArray.sumRange(0, 2); // return 1 + 2 + 5 = 8
    }

    mod performance {
        use super::*;

        // #[ignore]
        #[test]
        fn ns_30k0_15k_updates_15k_sums() {
            let na = NumArray::new(vec![0; 30000]);
            for i in 0..15000 {
                assert_eq!(na.sum_range(0, 29999), i);
                na.update(i, 1);
                assert_eq!(na.sum_range(0, 29999), i + 1);
            }
        }
    }
}
