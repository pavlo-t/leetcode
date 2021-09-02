package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3608/]]
 */
//noinspection DuplicatedCode
class c2021_01_18 extends AnyWordSpec with Matchers {
  /**
   * ===  Max Number of K-Sum Pairs ===
   *
   * You are given an integer array `nums` and an integer `k`.
   *
   * In one operation, you can pick two numbers from the array whose sum equals `k` and remove them from the array.
   *
   * Return ''the maximum number of operations you can perform on the array''.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 100_000`
   *  - `1 <= nums[i] <= 1_000_000_000`
   *  - `1 <= k <= 1_000_000_000`
   */
  object Solution {
    def maxOperations(nums: Array[Int], k: Int): Int = {
      val ns = nums.sorted
      @scala.annotation.tailrec
      def rec(l: Int, r: Int, rsf: Int): Int = {
        if (l >= r) rsf
        else {
          val s = ns(l) + ns(r)
          if (s == k) rec(l + 1, r - 1, rsf + 1)
          else if (s < k) rec(l + 1, r, rsf)
          else rec(l, r - 1, rsf)
        }
      }
      rec(0, nums.length - 1, 0)
    }
  }

  object SolutionMapCounts {
    def maxOperations(nums: Array[Int], k: Int): Int = {
      @scala.annotation.tailrec
      def rec(i: Int, counts: Map[Int, Int]): Int = {
        if (i == nums.length) counts.foldLeft(0) { case (acc, (n, c)) => acc + c.min(counts(k - n)) } / 2
        else rec(i + 1, counts.updated(nums(i), counts(nums(i)) + 1))
      }
      rec(0, Map().withDefaultValue(0))
    }
  }

  import Solution.maxOperations

  "Example 1: (nums = [1,2,3,4], k = 5) -> 2" in {
    maxOperations(Array(1, 2, 3, 4), 5) shouldBe 2
    //Explanation: Starting with nums = [1,2,3,4]:
    // - Remove numbers 1 and 4, then nums = [2,3]
    // - Remove numbers 2 and 3, then nums = []
    // There are no more pairs that sum up to 5, hence a total of 2 operations.
  }
  "Example 2: (nums = [3,1,3,4,3], k = 6) -> 1" in {
    maxOperations(Array(3, 1, 3, 4, 3), 6) shouldBe 1
    //Explanation: Starting with nums = [3,1,3,4,3]:
    // - Remove the first two 3's, then nums = [1,4,3]
    // There are no more pairs that sum up to 6, hence a total of 1 operation.
  }

  "(nums = 1 to 100_000, k = 100_001) -> 50_000" in {
    maxOperations((1 to 100_000).toArray, 100_001) shouldBe 50_000
  }

}
