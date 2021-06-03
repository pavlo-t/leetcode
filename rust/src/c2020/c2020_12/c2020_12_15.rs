#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn sorted_squares_2_pointer(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 || nums[0] >= 0 {
            nums.into_iter().map(|n| n * n).collect()
        } else {
            let mut l = 0;
            let mut r = nums.len() - 1;
            let mut result = vec![0; nums.len()];

            while l <= r {
                if -nums[l] >= nums[r] {
                    result[r - l] = nums[l] * nums[l];
                    l += 1;
                } else {
                    result[r - l] = nums[r] * nums[r];
                    r -= 1;
                }
            }

            result
        }
    }

    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums: Vec<_> = nums.into_iter().map(|n| n * n).collect();
        nums.sort();
        nums
    }
}

#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![-4, -1, 0, 3, 10];
        let expected = vec![0, 1, 9, 16, 100];
        assert_eq!(Solution::sorted_squares(nums), expected);
        //Explanation: After squaring, the array becomes [16,1,0,9,100].
        //After sorting, it becomes [0,1,9,16,100].
    }

    #[test]
    fn example2() {
        let nums = vec![-7, -3, 2, 3, 11];
        let expected = vec![4, 9, 9, 49, 121];
        assert_eq!(Solution::sorted_squares(nums), expected);
    }

    #[test]
    fn test10000_nums() {
        let mut nums: Vec<_> = (1..=10000).map(|n| if n % 2 == 0 { -n } else { n }).collect();
        nums.sort();
        let expected: Vec<_> = (1..=10000).map(|n| n * n).collect();
        assert_eq!(Solution::sorted_squares(nums), expected);
    }

    #[test]
    fn test1_000_000_nums() {
        let repeat = 100;

        let mut nums: Vec<_> = (1..=10000).map(|n| if n % 2 == 0 { -n } else { n }).collect();
        nums.sort();
        let nums =
            nums.into_iter()
                .flat_map(|n| std::iter::repeat(n).take(repeat).collect::<Vec<_>>())
                .collect();

        let expected: Vec<_> =
            (1..=10000)
                .map(|n| n * n)
                .flat_map(|n| std::iter::repeat(n).take(repeat).collect::<Vec<_>>())
                .collect();

        assert_eq!(Solution::sorted_squares(nums), expected);
    }
    //"(nums = [-10000,-9998,...,-2,1,3,...,9997,9999]*100) -> [1,4,9,...,100_000_000]*100" in {
    //  val repeat = 100
    //
    //  val nums =
    //    (1 to 10000)
    //      .map(n => if (n % 2 == 0) -n else n)
    //      .flatMap(Array.fill(repeat)(_))
    //      .toArray
    //      .sorted
    //  val expected =
    //    (1 to 10000)
    //      .map(n => n * n)
    //      .flatMap(Array.fill(repeat)(_))
    //      .toArray
    //
    //  sortedSquares(nums) shouldBe expected
    //}
}
