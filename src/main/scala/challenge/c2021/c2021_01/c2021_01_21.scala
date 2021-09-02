package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3611/]]
 */
//noinspection DuplicatedCode
class c2021_01_21 extends AnyWordSpec with Matchers {
  /**
   * === Find the Most Competitive Subsequence ===
   *
   * Given an integer array `nums` and a positive integer `k`,
   * return ''the most '''competitive''' subsequence of'' `nums` ''of size'' `k`.
   *
   * An array's subsequence is a resulting sequence obtained by erasing some (possibly zero) elements from the array.
   *
   * We define that a subsequence `a` is more '''competitive''' than a subsequence `b` (of the same length)
   * if in the first position where `a` and `b` differ,
   * subsequence `a` has a number '''less''' than the corresponding number in `b`.
   * For example, `[1,3,4]` is more competitive than `[1,3,5]` because the first position they differ
   * is at the final number, and `4` is less than `5`.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 100_000`
   *  - `0 <= nums[i] <= 1_000_000_000`
   *  - `1 <= k <= nums.length`
   */
  object Solution {
    def mostCompetitive(nums: Array[Int], k: Int): Array[Int] = {
      val stack = scala.collection.mutable.Stack[Int]()
      for (i <- nums.indices) {
        while (stack.nonEmpty && stack.head > nums(i) && stack.size + nums.length - i > k)
          stack.pop()
        if (stack.size < k)
          stack.push(nums(i))
      }
      stack.toArray.reverse
    }
  }

  object SolutionMyTimeLimitExceeded {
    def mostCompetitive(nums: Array[Int], k: Int): Array[Int] = {
      @scala.annotation.tailrec
      def rec(rsf: List[Int], i: Int): List[Int] = {
        if (i == nums.length) rsf
        else if (rsf.nonEmpty && rsf.head > nums(i) && rsf.size + nums.length - i > k) rec(rsf.tail, i)
        else if (rsf.size < k) rec(nums(i) :: rsf, i + 1)
        else rec(rsf, i + 1)
      }

      rec(Nil, 0).reverse.toArray
    }
  }

  import Solution.mostCompetitive

  "Example 1: (nums = [3,5,2,6], k = 2) -> [2,6]" in {
    mostCompetitive(Array(3, 5, 2, 6), 2) shouldBe Array(2, 6)
    //Explanation:
    // Among the set of every possible subsequence: {[3,5], [3,2], [3,6], [5,2], [5,6], [2,6]},
    // [2,6] is the most competitive.
  }
  "Example 2: (nums = [2,4,3,3,5,4,9,6], k = 4) -> [2,3,3,4]" in {
    mostCompetitive(Array(2, 4, 3, 3, 5, 4, 9, 6), 4) shouldBe Array(2, 3, 3, 4)
  }

}
