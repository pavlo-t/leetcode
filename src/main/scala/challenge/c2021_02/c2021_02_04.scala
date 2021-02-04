package challenge.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3628/]]
 */
class c2021_02_04 extends AnyWordSpec with Matchers {
  /**
   * === Longest Harmonious Subsequence ===
   *
   * We define a harmonious array as an array where the difference between its maximum value and its minimum value
   * is '''exactly''' `1`.
   *
   * Given an integer array `nums`, return
   * ''the length of its longest harmonious subsequence among all its possible subsequences''.
   *
   * A '''subsequence''' of array is a sequence that can be derived from the array
   * by deleting some or no elements without changing the order of the remaining elements.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 20_000`
   *  - `-1_000_000_000 <= nums[i] <= 1_000_000_000`
   */
  object Solution {
    def findLHS(nums: Array[Int]): Int = {
      val counts = nums.foldLeft(Map[Int, Int]().withDefaultValue(0))((acc, i) => acc.updated(i, acc(i) + 1))
      counts.foldLeft(0) {
        case (rsf, (k, v)) => counts(k + 1) match {
          case 0 => rsf
          case w => rsf max (v + w)
        }
      }
    }
  }

  object SolutionMySortPlus2Pointers {
    def findLHS(nums: Array[Int]): Int = {
      val sorted = nums.sorted
      @scala.annotation.tailrec
      def rec(l: Int, r: Int, rsf: Int): Int = {
        if (r >= sorted.length) rsf
        else sorted(r) - sorted(l) match {
          case 0 => rec(l, r + 1, rsf)
          case 1 => rec(l, r + 1, rsf max (r - l + 1))
          case _ => rec(l + 1, r, rsf)
        }
      }
      rec(0, 1, 0)
    }
  }

  import Solution.findLHS

  "Example 1: (nums = [1,3,2,2,5,2,3,7]) -> 5" in {
    findLHS(Array(1, 3, 2, 2, 5, 2, 3, 7)) shouldBe 5
    //Explanation: The longest harmonious subsequence is [3,2,2,2,3].
  }
  "Example 2: (nums = [1,2,3,4]) -> 2" in {
    findLHS(Array(1, 2, 3, 4)) shouldBe 2
  }
  "Example 3: (nums = [1,1,1,1]) -> 0" in {
    findLHS(Array(1, 1, 1, 1)) shouldBe 0
  }

  "(nums = [1]) -> 0" in {
    findLHS(Array(1)) shouldBe 0
  }

}
