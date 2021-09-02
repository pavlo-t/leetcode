package challenge.c2021.c2021_08

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/explore/challenge/card/august-leetcoding-challenge-2021/615/week-3-august-15th-august-21st/3892/]] */
class c2021_08_16 extends AnyWordSpec with Matchers {
  /**
   * == Range Sum Query - Immutable ==
   *
   * Given an integer array `nums`, handle multiple queries of the following type:
   *  1. Calculate the '''sum''' of the elements of `nums` between indices `left` and `right` '''inclusive'''
   *     where `left <= right`.
   *
   * Implement the `NumArray` class:
   *  - `NumArray(int[] nums)`
   *    Initializes the object with the integer array `nums`.
   *  - `int sumRange(int left, int right)`
   *    Returns the '''sum''' of the elements of `nums` between indices `left` and `right` inclusive
   *    (i.e. `nums[left] + nums[left + 1] + ... + nums[right]`).
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 10_000`
   *  - `-100_000 <= nums[i] <= 100_000`
   *  - `0 <= left <= right < nums.length`
   *  - At most `10_000` calls will be made to `sumRange`.
   */
  class NumArray(_nums: Array[Int]) {
    private val ls = _nums.iterator.foldLeft(Seq[Int](0))((a, c) => a.appended(a.last + c))
    def sumRange(left: Int, right: Int): Int = ls(right + 1) - ls(left)
  }

  "Example 1: NumArray([-2,0,3,-5,2,-1]), sumRange(0,2): 1, sumRange(2,5): -1, sumRange(0,5): -3" in {
    val na = new NumArray(Array(-2, 0, 3, -5, 2, -1))
    na.sumRange(0, 2) shouldBe 1
    na.sumRange(2, 5) shouldBe -1
    na.sumRange(0, 5) shouldBe -3
    // Explanation
    // NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
    // numArray.sumRange(0, 2); // return (-2) + 0 + 3 = 1
    // numArray.sumRange(2, 5); // return 3 + (-5) + 2 + (-1) = -1
    // numArray.sumRange(0, 5); // return (-2) + 0 + 3 + (-5) + 2 + (-1) = -3
  }
}
