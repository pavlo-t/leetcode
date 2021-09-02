package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3605/]]
 */
//noinspection DuplicatedCode
class c2021_01_15 extends AnyWordSpec with Matchers {
  /**
   * === Get Maximum in Generated Array ===
   *
   * You are given an integer `n`.
   * An array `nums` of length `n + 1` is generated in the following way:
   *  - `nums[0] = 0`
   *  - `nums[1] = 1`
   *  - `nums[2 * i] = nums[i]` when `2 <= 2 * i <= n`
   *  - `nums[2 * i + 1] = nums[i] + nums[i + 1]` when `2 <= 2 * i + 1 <= n`
   *
   * Return ''the '''maximum''' integer in the array'' `nums`.
   *
   * '''Constraints:'''
   *  - `0 <= n <= 100`
   */
  object Solution {
    def getMaximumGenerated(n: Int): Int = {
      @scala.annotation.tailrec
      def buildArr(i: Int, rsf: Seq[Int]): Seq[Int] = {
        if (i > n) rsf
        else if (i == 0) buildArr(1, rsf :+ 0)
        else if (i == 1) buildArr(2, rsf :+ 1)
        else if (i % 2 == 0) buildArr(i + 1, rsf :+ rsf(i / 2))
        else buildArr(i + 1, rsf :+ (rsf(i / 2) + rsf(i / 2 + 1)))
      }

      buildArr(0, Nil).max
    }
  }

  import Solution.getMaximumGenerated

  "Example 1: (n = 7) -> 3" in {
    getMaximumGenerated(7) shouldBe 3
    //Explanation: According to the given rules:
    //  nums[0] = 0
    //  nums[1] = 1
    //  nums[(1 * 2) = 2] = nums[1] = 1
    //  nums[(1 * 2) + 1 = 3] = nums[1] + nums[2] = 1 + 1 = 2
    //  nums[(2 * 2) = 4] = nums[2] = 1
    //  nums[(2 * 2) + 1 = 5] = nums[2] + nums[3] = 1 + 2 = 3
    //  nums[(3 * 2) = 6] = nums[3] = 2
    //  nums[(3 * 2) + 1 = 7] = nums[3] + nums[4] = 2 + 1 = 3
    //Hence, nums = [0,1,1,2,1,3,2,3], and the maximum is 3.
  }
  "Example 2: (n = 2) -> 1" in {
    getMaximumGenerated(2) shouldBe 1
    //Explanation: According to the given rules, the maximum between nums[0], nums[1], and nums[2] is 1.
  }
  "Example 3: (n = 3) -> 2" in {
    getMaximumGenerated(3) shouldBe 2
    //Explanation: According to the given rules, the maximum between nums[0], nums[1], nums[2], and nums[3] is 2.
  }

  "(n = 0) -> 0" in (getMaximumGenerated(0) shouldBe 0)
  "(n = 1) -> 1" in (getMaximumGenerated(1) shouldBe 1)
  "(n = 2) -> 1" in (getMaximumGenerated(2) shouldBe 1)
  "(n = 3) -> 2" in (getMaximumGenerated(3) shouldBe 2)
  "(n = 4) -> 2" in (getMaximumGenerated(4) shouldBe 2)
  "(n = 5) -> 3" in (getMaximumGenerated(5) shouldBe 3)
  "(n = 6) -> 3" in (getMaximumGenerated(6) shouldBe 3)
  "(n = 7) -> 3" in (getMaximumGenerated(7) shouldBe 3)
  "(n = 8) -> 3" in (getMaximumGenerated(8) shouldBe 3)

  "(n = 100) -> 21" in (getMaximumGenerated(100) shouldBe 21)

}
