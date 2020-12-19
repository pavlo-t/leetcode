package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/571/week-3-december-15th-december-21st/3570/]]
 */
class c2020_12_18 extends AnyWordSpec with Matchers {

  /**
   * === Increasing Triplet Subsequence ===
   *
   * Given an integer array `nums`, return `true` ''if there exists a triple of indices'' `(i, j, k)`
   * ''such that'' `i < j < k` ''and'' `nums[i] < nums[j] < nums[k]`.
   * If no such indices exists, return `false`.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 100_000`
   *  - `-2^31 <= nums[i] <= 2^31 - 1`
   *
   * '''Follow up:''' Could you implement a solution that runs in `O(n)` time complexity and `O(1)` space complexity?
   */
  object Solution {
    def increasingTriplet(A: Array[Int]): Boolean = {
      @scala.annotation.tailrec
      def rec(i: Int, min: Int, mid: Int): Boolean = {
        if (i == A.length) false
        else if (A(i) <= min) rec(i + 1, A(i), mid)
        else if (A(i) <= mid) rec(i + 1, min, A(i))
        else true
      }
      rec(1, A(0), Int.MaxValue)
    }
  }

  import Solution.increasingTriplet

  "Example 1: (nums = [1,2,3,4,5]) -> true" in {
    increasingTriplet(Array(1, 2, 3, 4, 5)) shouldBe true
    //Explanation: Any triplet where i < j < k is valid.
  }
  "Example 2: (nums = [5,4,3,2,1]) -> false" in {
    increasingTriplet(Array(5, 4, 3, 2, 1)) shouldBe false
    //Explanation: No triplet exists.
  }
  "Example 3: (nums = [2,1,5,0,4,6]) -> true" in {
    increasingTriplet(Array(2, 1, 5, 0, 4, 6)) shouldBe true
    //Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] == 4 < nums[5] == 6.
  }

  "(nums = [1,1,1]) -> false" in {
    increasingTriplet(Array(1, 1, 1)) shouldBe false
  }
  "(nums = [2,1,3]) -> false" in {
    increasingTriplet(Array(2, 1, 3)) shouldBe false
  }
  "(nums = [2,2,5,5,4,4]) -> false" in {
    increasingTriplet(Array(2, 2, 5, 5, 4, 4)) shouldBe false
  }
  "(nums = [2,5,4,5,4]) -> true" in {
    increasingTriplet(Array(2, 5, 4, 5, 4)) shouldBe true
  }
}
