package contest.w216

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


/**
 * [[https://leetcode.com/contest/weekly-contest-216/problems/ways-to-make-a-fair-array/]]
 */
//noinspection DuplicatedCode
class w216_3 extends AnyWordSpec with Matchers {

  /**
   * === 5607. Ways to Make a Fair Array ===
   *
   * You are given an integer array nums. You can choose exactly one index (0-indexed) and remove the element. Notice that the index of the elements may change after the removal.
   *
   * For example, if nums = [6,1,7,4,1]:
   *  - Choosing to remove index 1 results in nums = [6,7,4,1].
   *  - Choosing to remove index 2 results in nums = [6,1,4,1].
   *  - Choosing to remove index 4 results in nums = [6,1,7,4].
   *
   * An array is fair if the sum of the odd-indexed values equals the sum of the even-indexed values.
   *
   * Return the number of indices that you could choose such that after the removal, nums is fair.
   *
   * Constraints:
   *  - 1 <= nums.length <= 100000
   *  - 1 <= nums[i] <= 10000
   */
object Solution {
  def waysToMakeFair(nums: Array[Int]): Int = if (nums.length < 2) 1 else {
    val el = nums.clone()
    val ol = nums.clone()
    val er = nums.clone()
    val or = nums.clone()
    ol(0) = 0
    if ((nums.length - 1) % 2 != 0) er(nums.length - 1) = 0 else or(nums.length - 1) = 0

    for (i <- 1 until nums.length)
      if (i % 2 == 0) {
        el(i) += el(i - 1)
        ol(i) = ol(i - 1)
      } else {
        el(i) = el(i - 1)
        ol(i) += ol(i - 1)
      }

    for (i <- (nums.length - 2) to 0 by -1)
      if (i % 2 == 0) {
        er(i) += er(i + 1)
        or(i) = or(i + 1)
      } else {
        er(i) = er(i + 1)
        or(i) += or(i + 1)
      }

    var result = 0
    for (i <- nums.indices) {
      if (i == 0) {
        if (or(1) == er(1)) result += 1
      }
      else if (i == nums.length - 1) {
        if (ol(nums.length - 2) == el(nums.length - 2)) result += 1
      }
      else if (ol(i - 1) + er(i + 1) == el(i - 1) + or(i + 1))
        result += 1
    }
    result
  }
}

  import Solution.waysToMakeFair

  "Example 1: (nums = [2,1,6,4]) -> 1" in {
    waysToMakeFair(Array(2, 1, 6, 4)) shouldBe 1
    //Explanation:
    //Remove index 0: [1,6,4] -> Even sum: 1 + 4 = 5. Odd sum: 6. Not fair.
    //Remove index 1: [2,6,4] -> Even sum: 2 + 4 = 6. Odd sum: 6. Fair.
    //Remove index 2: [2,1,4] -> Even sum: 2 + 4 = 6. Odd sum: 1. Not fair.
    //Remove index 3: [2,1,6] -> Even sum: 2 + 6 = 8. Odd sum: 1. Not fair.
    //There is 1 index that you can remove to make nums fair.
  }
  "Example 2: (nums = [1,1,1]) -> 3" in {
    waysToMakeFair(Array(1, 1, 1)) shouldBe 3
    //Explanation: You can remove any index and the remaining array is fair.
  }
  "Example 3: (nums = [1,2,3]) -> 0" in {
    waysToMakeFair(Array(1, 2, 3)) shouldBe 0
    //Explanation: You cannot make a fair array after removing any index.
  }

  "(nums = [1..=10000]) -> 0" in {
    waysToMakeFair((1 to 10000).toArray) shouldBe 0
  }
  "(nums = [1..=100000]) -> 0" in {
    waysToMakeFair((1 to 100000).toArray) shouldBe 0
  }
  "(nums = [1,1..] 100001) ->  100001" in {
    waysToMakeFair(Array.fill(100001)(1)) shouldBe 100001
  }
}
