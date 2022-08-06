#![allow(dead_code)]
//! \#458. Poor Pigs
//! ================
//!
//! <https://leetcode.com/problems/poor-pigs>
//!
//! There are `buckets` buckets of liquid, where __exactly one__ of the buckets is poisonous.
//! To figure out which one is poisonous, you feed some number of (poor) pigs the liquid to see
//! whether they will die or not.
//! Unfortunately, you only have `minutesToTest` minutes to determine which bucket is poisonous.
//!
//! You can feed the pigs according to these steps:
//!
//! 1. Choose some live pigs to feed.
//! 2. For each pig, choose which buckets to feed it.
//!    The pig will consume all the chosen buckets simultaneously and will take no time.
//! 3. Wait for `minutes_to_die` minutes. You may __not__ feed any other pigs during this time.
//! 4. After `minutes_to_die` minutes have passed,
//!    any pigs that have been fed the poisonous bucket will die,
//!    and all others will survive.
//! 5. Repeat this process until you run out of time.
//!
//! Given `buckets`, `minutesToDie`, and `minutesToTest`, return
//! _the __minimum__ number of pigs needed to figure out which bucket is poisonous within the allotted time_.
//!
//! ##### Constraints
//!
//! - `1 <= buckets <= 1000`
//! - `1 <= minutesToDie <= minutesToTest <= 100`
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_06::Solution;
//! let buckets = 1000;
//! let minutes_to_die = 15;
//! let minutes_to_test = 60;
//! assert_eq!(Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test), 5);
//! ```
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_06::Solution;
//! let buckets = 4;
//! let minutes_to_die = 15;
//! let minutes_to_test = 15;
//! assert_eq!(Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test), 2);
//! ```
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_08::c2022_08_06::Solution;
//! let buckets = 4;
//! let minutes_to_die = 15;
//! let minutes_to_test = 30;
//! assert_eq!(Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test), 2);
//! ```
//!
//! ##### Hints
//!
//! <div onclick="toggle_hint('leetcode-hint-1')" class="toggle-hint">
//!
//! __Hint 1__
//!
//! <div id="leetcode-hint-1" class="hidden">
//!
//! What if you only have one shot? Eg. `4` buckets, `15` mins to die, and `15` mins to test.
//!
//! </div></div>
//!
//! <div onclick="toggle_hint('leetcode-hint-2')" class="toggle-hint">
//!
//! __Hint 2__
//!
//! <div id="leetcode-hint-2" class="hidden">
//!
//! How many states can we generate with `x` pigs and `T` tests?
//!
//! </div></div>
//!
//! <div onclick="toggle_hint('leetcode-hint-3')" class="toggle-hint">
//!
//! __Hint 3__
//!
//! <div id="leetcode-hint-3" class="hidden">
//!
//! Find minimum `x` such that `(T+1)^x >= N`
//!
//! </div></div>
//!
//! <style>
//! .toggle-hint:hover {
//!   cursor:pointer
//! }
//! </style>
//!
//! <script>
//! function toggle_hint(id) {
//!   const element = document.getElementById(id)
//!   if (hasClass(element, "hidden")) {
//!     removeClass(element, "hidden")
//!   } else {
//!     addClass(element, "hidden")
//!   }
//! }
//! </script>

pub struct Solution;
impl Solution {
    /// Linear search
    pub fn poor_pigs_v1(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let tests = minutes_to_test / minutes_to_die + 1;

        std::iter::successors(Some(0u32), |n| n.checked_add(1))
            .find(|&pigs| tests.pow(pigs) >= buckets)
            .unwrap() as i32
    }

    /// Using logarithms
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let tests = minutes_to_test / minutes_to_die + 1;
        ((buckets as f64).log2() / (tests as f64).log2()).ceil() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn b_1_1_1() { assert_eq!(Solution::poor_pigs(1, 1, 1), 0); }
    /// | pigs\buckets |0|1|
    /// |--------------|-|-|
    /// | 0            | |x|
    #[rustfmt::skip] #[test] fn b_2_1_1() { assert_eq!(Solution::poor_pigs(2, 1, 1), 1); }
    /// | pigs\buckets |0|1|2|
    /// |--------------|-|-|-|
    /// | 0            | |x| |
    /// | 1            | | |x|
    #[rustfmt::skip] #[test] fn b_3_1_1() { assert_eq!(Solution::poor_pigs(3, 1, 1), 2); }
    /// | pigs\buckets |0|1|2|3|
    /// |--------------|-|-|-|-|
    /// | 0            | |x| |x|
    /// | 1            | | |x|x|
    #[rustfmt::skip] #[test] fn b_4_1_1() { assert_eq!(Solution::poor_pigs(4, 1, 1), 2); }
    #[rustfmt::skip] #[test] fn b_5_1_1() { assert_eq!(Solution::poor_pigs(5, 1, 1), 3); }
    #[rustfmt::skip] #[test] fn b_6_1_1() { assert_eq!(Solution::poor_pigs(6, 1, 1), 3); }
    #[rustfmt::skip] #[test] fn b_7_1_1() { assert_eq!(Solution::poor_pigs(7, 1, 1), 3); }
    /// |pigs\buckets |0|1|2|3|4|5|6|7|
    /// |-------------|-|-|-|-|-|-|-|-|
    /// | 0           | |x| |x| |x| |x|
    /// | 1           | | |x|x| | |x|x|
    /// | 2           | | | | |x|x|x|x|
    ///
    /// We can test `2 ** pigs` buckets in a single attempt.
    #[rustfmt::skip] #[test] fn b_8_1_1() { assert_eq!(Solution::poor_pigs(8, 1, 1), 3); }
    #[rustfmt::skip] #[test] fn b_9_1_1() { assert_eq!(Solution::poor_pigs(9, 1, 1), 4); }

    #[rustfmt::skip] #[test] fn b_1_1_2() { assert_eq!(Solution::poor_pigs(1, 1, 2), 0); }
    #[rustfmt::skip] #[test] fn b_2_1_2() { assert_eq!(Solution::poor_pigs(2, 1, 2), 1); }
    /// | pigs\buckets |0|1|2|
    /// |--------------|-|-|-|
    /// | 0            | |a|b|
    #[rustfmt::skip] #[test] fn b_3_1_2() { assert_eq!(Solution::poor_pigs(3, 1, 2), 1); }
    #[rustfmt::skip] #[test] fn b_4_1_2() { assert_eq!(Solution::poor_pigs(4, 1, 2), 2); }
    #[rustfmt::skip] #[test] fn b_5_1_2() { assert_eq!(Solution::poor_pigs(5, 1, 2), 2); }
    #[rustfmt::skip] #[test] fn b_6_1_2() { assert_eq!(Solution::poor_pigs(6, 1, 2), 2); }
    #[rustfmt::skip] #[test] fn b_7_1_2() { assert_eq!(Solution::poor_pigs(7, 1, 2), 2); }
    #[rustfmt::skip] #[test] fn b_8_1_2() { assert_eq!(Solution::poor_pigs(8, 1, 2), 2); }
    /// __Test 1__
    ///
    /// | pigs\buckets |0|1|2|3|4|5|6|7|8|
    /// |--------------|-|-|-|-|-|-|-|-|-|
    /// | 0            | | | | |x|x| | |x|
    /// | 1            | | | | | | |x|x|x|
    ///
    /// __Test 2__
    ///
    /// If no pigs have died - it becomes `poor_pigs(4, 1, 1)`:
    ///
    /// | pigs\buckets |0|1|2|1|
    /// |--------------|-|-|-|-|
    /// | 0            | |x| |x|
    /// | 1            | | |x|x|
    ///
    /// If only one pig has died - it becomes `poor_pigs(2, 1, 1)`:
    ///
    /// | pigs\buckets |4|5|
    /// |--------------|-|-|
    /// | 1            | |x|
    ///
    /// | pigs\buckets |6|7|
    /// |--------------|-|-|
    /// | 0            | |x|
    ///
    /// If both pigs have died - the problem is solved.
    ///
    /// Another representation where `a` and `b` are pigs
    ///
    /// | #test | 0| 1| 2| 3| 4| 5| 6| 7| 8|
    /// |-------|::|::|::|::|::|::|::|::|::|
    /// | 0     |  |  |  |  | a| a| b| b|ab|
    /// | 1     |  | a| b|ab|  | b|  | a|  |
    ///
    /// Another representation where time of death is a digit in base `tests + 1`
    /// and `# pigs` is N of available positions:
    ///
    /// - `0` - doesn't die
    /// - `1` - dies in the first test
    /// - `2` - dies in the second test
    ///
    /// |decimal|pig 1|pig 2|
    /// |-------|:---:|:---:|
    /// | 0     |   0 |   0 |
    /// | 1     |   0 |   1 |
    /// | 2     |   0 |   2 |
    /// | 3     |   1 |   0 |
    /// | 4     |   1 |   1 |
    /// | 5     |   1 |   2 |
    /// | 6     |   2 |   0 |
    /// | 7     |   2 |   1 |
    /// | 8     |   2 |   2 |
    #[rustfmt::skip] #[test] fn b_9_1_2() { assert_eq!(Solution::poor_pigs(9, 1, 2), 2); }
    /// | p\b | 0| 1| 2| 3| 4| 5| 6| 7| 8| 9|10|11|12|13|14|15|16|17|18|19|20|21|22|23|24|25|26|
    /// |:----|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|
    /// | 0   |  |  |  |  |  |  |  |  | x| x| x| x|  |  |  |  | x| x|  |  |  |  | x| x|  |  | x|
    /// | 1   |  |  |  |  |  |  |  |  |  |  |  |  | x| x| x| x| x| x|  |  |  |  |  |  | x| x| x|
    /// | 2   |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | x| x| x| x| x| x| x| x| x|
    ///
    /// After the first round, depending on the number of pigs that have died, the problem becomes either:
    ///
    /// - `poor_pigs(8, 1, 1)` with 3 pigs
    /// - `poor_pigs(4, 1, 1)` with 2 pigs
    /// - `poor_pigs(2, 1, 1)` with 1 pig
    /// - `poor_pigs(1, 1, 1)` with 0 pigs
    ///
    /// | t\b | 0| 1| 2| 3| 4| 5| 6| 7 | 8| 9|10|11|12|13|14|15|16|17|18|19|20|21|22|23|24|25| 26|
    /// |:----|::|::|::|::|::|::|::|:-:|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|:-:|
    /// | 0   |  |  |  |  |  |  |  |   | a| a| a| a| b| b| b| b|ab|ab| c| c| c| c|ab|ab|ac|ac|abc|
    /// | 1   |  | a| b|ab| c|ac|bc|abc|  | b| c|bc|  | a| c|ac|  | c|  | a| b|ab|  | c|  | b|   |
    ///
    /// Another representation where time of death is a digit in base `tests + 1`
    /// and `# pigs` is N of available positions:
    ///
    /// - `0` - doesn't die
    /// - `1` - dies in the first test
    /// - `2` - dies in the second test
    ///
    /// | pig | 0| 1| 2| 3| 4| 5| 6| 7| 8| 9|10|11|12|13|14|15|16|17|18|19|20|21|22|23|24|25|26|
    /// |:---:|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|::|
    /// | 0   | 0| 1| 2| 0| 1| 2| 0| 1| 2| 0| 1| 2| 0| 1| 2| 0| 1| 2| 1| 2| 0| 1| 2| 0| 1| 2| 0|
    /// | 1   | 0| 0| 0| 1| 1| 1| 2| 2| 2| 0| 0| 0| 1| 1| 1| 2| 2| 2| 0| 0| 1| 1| 1| 2| 2| 2| 0|
    /// | 2   | 0| 0| 0| 0| 0| 0| 0| 0| 0| 1| 1| 1| 1| 1| 1| 1| 1| 1| 2| 2| 2| 2| 2| 2| 2| 2| 2|
    ///
    /// So we can have at most `(N of tests + 1) ** pigs` states.
    #[rustfmt::skip] #[test] fn b_27_1_2() { assert_eq!(Solution::poor_pigs(27, 1, 2), 3); }

    /// | pigs\buckets |0|1|2|3|
    /// |--------------|-|-|-|-|
    /// | 0            |0|1|2|3|
    #[rustfmt::skip] #[test] fn b_4_1_3() { assert_eq!(Solution::poor_pigs(4, 1, 3), 1); }

    #[rustfmt::skip] #[test] fn b_4_15_15() { assert_eq!(Solution::poor_pigs(4, 15, 15), 2); }
    #[rustfmt::skip] #[test] fn b_4_15_30() { assert_eq!(Solution::poor_pigs(4, 15, 30), 2); }
    #[rustfmt::skip] #[test] fn b_4_15_44() { assert_eq!(Solution::poor_pigs(4, 15, 44), 2); }
    #[rustfmt::skip] #[test] fn b_4_15_45() { assert_eq!(Solution::poor_pigs(4, 15, 45), 1); }

    #[rustfmt::skip] #[test] fn b_1000_15_60() { assert_eq!(Solution::poor_pigs(1000, 15, 60), 5); }
}
