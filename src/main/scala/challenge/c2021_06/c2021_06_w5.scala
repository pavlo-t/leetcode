package challenge.c2021_06

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/607/week-5-june-29th-june-30th/3795/]]
 */
class c2021_06_w5 extends AnyWordSpec with Matchers {
  /**
   * == Armstrong Number ==
   *
   * Given an integer `n`, return `true` ''if and only if it is an '''Armstrong number'''''.
   *
   * The `k`-digit number `n` is an Armstrong number if and only if the `k`th power of each digit sums to `n`.
   *
   * '''Constraints:'''
   *  - `1 <= n <= 100_000_000`
   */
  object Solution {
    def isArmstrong(n: Int): Boolean = {
      val s = n.toString
      s.foldLeft(0) { case (acc, c) => acc + math.pow(c - '0', s.length).toInt } == n
    }
  }

  import Solution.isArmstrong

  "Example 1: (n = 153) -> true" in {
    isArmstrong(153) shouldBe true
    //Explanation: 153 is a 3-digit number, and 153 = 1^3 + 5^3 + 3^3.
  }
  "Example 2: (n = 123) -> false" in {
    isArmstrong(123) shouldBe false
    //Explanation: 123 is a 3-digit number, and 123 != 1^3 + 2^3 + 3^3 = 36.
  }
  "(n = 100_000_000) -> false" in {
    isArmstrong(100_000_000) shouldBe false
  }
}
