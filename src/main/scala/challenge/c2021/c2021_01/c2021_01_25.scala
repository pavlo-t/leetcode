package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3616/]]
 */
//noinspection DuplicatedCode
class c2021_01_25 extends AnyWordSpec with Matchers {
  /**
   * === Check If All 1's Are at Least Length K Places Away ===
   *
   * Given an array `nums` of 0s and 1s and an integer `k`,
   * return `True` if all 1's are at least `k` places away from each other,
   * otherwise return `False`.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 100_000`
   *  - `0 <= k <= nums.length`
   *  - `nums[i]` is `0` or `1`
   */
  object Solution {
    def kLengthApart(nums: Array[Int], k: Int): Boolean = {
      @scala.annotation.tailrec
      def rec(i: Int, zeros: Int): Boolean =
        if (i == nums.length) true
        else if (nums(i) == 1)
          if (zeros < k) false
          else rec(i + 1, 0)
        else rec(i + 1, zeros + 1)

      rec(1, if (nums.head == 0) 1 else 0)
    }
  }

  import Solution.kLengthApart

  "Example 1: (nums = [1,0,0,0,1,0,0,1], k = 2) -> true" in {
    kLengthApart(Array(1, 0, 0, 0, 1, 0, 0, 1), 2) shouldBe true
    //Explanation: Each of the 1s are at least 2 places away from each other.
  }
  "Example 2: (nums = [1,0,0,1,0,1], k = 2) -> false" in {
    kLengthApart(Array(1, 0, 0, 1, 0, 1), 2) shouldBe false
    //Explanation: The second 1 and third 1 are only one apart from each other.
  }
  "Example 3: (nums = [1,1,1,1,1], k = 0) -> true" in {
    kLengthApart(Array(1, 1, 1, 1, 1), 0) shouldBe true
  }
  "Example 4: (nums = [0,1,0,1], k = 1) -> true" in {
    kLengthApart(Array(0, 1, 0, 1), 1) shouldBe true
  }

  "(nums = Array.fill(100_000)(1), k = 0) -> true" in {
    kLengthApart(Array.fill(100_000)(1), 0) shouldBe true
  }

}
