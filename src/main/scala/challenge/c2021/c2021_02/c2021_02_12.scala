package challenge.c2021.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3637/]]
 */
//noinspection DuplicatedCode
class c2021_02_12 extends AnyWordSpec with Matchers {
  /**
   * === Number of Steps to Reduce a Number to Zero ===
   *
   * Given a non-negative integer `num`, return the number of steps to reduce it to zero.
   * If the current number is even, you have to divide it by 2, otherwise, you have to subtract 1 from it.
   *
   * '''Constraints:'''
   *  - `0 <= num <= 1_000_000`
   */
  object Solution {
    def numberOfSteps(num: Int): Int = {
      @scala.annotation.tailrec
      def rec(n: Int, rsf: Int): Int =
        if (n == 0) rsf
        else if (n == 1) rsf + 1
        else if ((n & 1) == 1) rec(n >> 1, rsf + 2)
        else rec(n >> 1, rsf + 1)
      rec(num, 0)
    }
  }

  object SolutionBitwise1 {
    def numberOfSteps(num: Int): Int =
      if (num == 0) 0
      else {
        @scala.annotation.tailrec
        def rec(bit: Int, rsf: Int): Int = {
          if (bit > num) rsf
          else if ((num & bit) == 0) rec(bit << 1, rsf + 1)
          else rec(bit << 1, rsf + 2)
        }
        rec(1, -1)
      }
  }
  object SolutionToBinaryString {
    def numberOfSteps(num: Int): Int =
      num.toBinaryString.foldLeft(-1) {
        case (rsf, '0') => rsf + 1
        case (rsf, '1') => rsf + 2
      }
  }
  object SolutionSimulation {
    def numberOfSteps(num: Int): Int = {
      @scala.annotation.tailrec
      def rec(n: Int, rsf: Int): Int =
        if (n == 0) rsf
        else if (n % 2 == 0) rec(n / 2, rsf + 1)
        else rec(n - 1, rsf + 1)
      rec(num, 0)
    }
  }

  import Solution.numberOfSteps

  "Example 1: (num = 14) -> 6" in {
    numberOfSteps(14) shouldBe 6
    //Explanation:
    //Step 1) 14 is even; divide by 2 and obtain 7.
    //Step 2) 7 is odd; subtract 1 and obtain 6.
    //Step 3) 6 is even; divide by 2 and obtain 3.
    //Step 4) 3 is odd; subtract 1 and obtain 2.
    //Step 5) 2 is even; divide by 2 and obtain 1.
    //Step 6) 1 is odd; subtract 1 and obtain 0.
  }
  "Example 2: (num = 8) -> 4" in {
    numberOfSteps(8) shouldBe 4
    //Explanation:
    //Step 1) 8 is even; divide by 2 and obtain 4.
    //Step 2) 4 is even; divide by 2 and obtain 2.
    //Step 3) 2 is even; divide by 2 and obtain 1.
    //Step 4) 1 is odd; subtract 1 and obtain 0.
  }
  "Example 3: (num = 123) -> 12" in {
    numberOfSteps(123) shouldBe 12
  }

  "(num = 0) -> 0" in {
    numberOfSteps(0) shouldBe 0
  }
  "(num = 1) -> 1" in {
    numberOfSteps(1) shouldBe 1
  }
  "(num = 1_000_000) -> 26" in {
    numberOfSteps(1_000_000) shouldBe 26
  }

}
