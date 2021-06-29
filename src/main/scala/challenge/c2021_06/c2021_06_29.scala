package challenge.c2021_06

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/607/week-5-june-29th-june-30th/3796/]]
 */
class c2021_06_29 extends AnyWordSpec with Matchers {
  /**
   * == Max Consecutive Ones III ==
   *
   * Given a binary array `nums` and an integer `k`,
   * return ''the maximum number of consecutive'' `1`''s in the array if you can flip at most'' `k` `0`''s''.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 100_000`
   *  - `nums[i]` is either `0` or `1`.
   *  - `0 <= k <= nums.length`
   */
  object Solution {
    def longestOnes(nums: Array[Int], k: Int): Int = {
      @scala.annotation.tailrec
      def rec(l: Int, r: Int, ones: Int, rsf: Int): Int =
        if (r >= nums.length)
          rsf
        else if (r - l + 1 - ones - nums(r) <= k)
          rec(l, r + 1, ones + nums(r), rsf max (r - l + 1))
        else
          rec(l + 1, r, ones - nums(l), rsf)

      rec(0, 0, 0, 0)
    }

    def longestOnesRec1(nums: Array[Int], k: Int): Int = {
      @scala.annotation.tailrec
      def rec(l: Int, r: Int, ones: Int, rsf: Int): Int =
        if (r >= nums.length)
          rsf
        else if (nums(r) == 1)
          rec(l, r + 1, ones + 1, rsf max (r - l + 1))
        else if (r - l + 1 - ones > k)
          rec(l + 1, r, ones - nums(l), rsf)
        else
          rec(l, r + 1, ones, rsf max (r - l + 1))

      rec(0, 0, 0, 0)
    }

    def longestOnesLoop(nums: Array[Int], k: Int): Int = {
      var l = 0
      var r = 0
      var ones = 0
      var rsf = 0
      while (r < nums.length) {
        if (nums(r) == 1) {
          ones += 1
          r += 1
        } else if (r - l + 1 - ones > k) {
          if (nums(l) == 1) ones -= 1
          l += 1
        } else {
          r += 1
        }
        rsf = rsf max (r - l)
      }
      rsf
    }
  }

  import Solution.longestOnes

  "Example 1: (nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2) -> 6" in {
    longestOnes(Array(1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0), 2) shouldBe 6
    //Explanation: [1,1,1,0,0,0,1,1,1,1,0]
    //             [1,1,1,0,0,1,1,1,1,1,1]
  }
  "Example 2: (nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3) -> 10" in {
    longestOnes(Array(0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1), 3) shouldBe 10
    //Explanation: [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1]
    //             [0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
  }

  "(nums = [1], k = 9) -> 1" in (longestOnes(Array(1), 9) shouldBe 1)
  "(nums = [0], k = 9) -> 1" in (longestOnes(Array(0), 9) shouldBe 1)
  "(nums = [0], k = 0) -> 0" in (longestOnes(Array(0), 0) shouldBe 0)

  "(nums = [1,0], k = 1) -> 2" in (longestOnes(Array(1, 0), 1) shouldBe 2)
  "(nums = [0,1], k = 1) -> 2" in (longestOnes(Array(0, 1), 1) shouldBe 2)
  "(nums = [1,0,0], k = 1) -> 2" in (longestOnes(Array(1, 0, 0), 1) shouldBe 2)
  "(nums = [0,0,1], k = 1) -> 2" in (longestOnes(Array(0, 0, 1), 1) shouldBe 2)
}
