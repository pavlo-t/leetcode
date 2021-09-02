package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3591/]]
 */
//noinspection DuplicatedCode
class c2021_01_03 extends AnyWordSpec with Matchers {
  /**
   * === Beautiful Arrangement ===
   *
   * Suppose you have `n` integers from `1` to `n`.
   * We define a beautiful arrangement as an array that is constructed by these `n` numbers successfully
   * if one of the following is true for the `i`th position (`1 <= i <= n`) in this array:
   *  - The number at the `i`th position is divisible by `i`.
   *  - `i` is divisible by the number at the `i`th position.
   *
   * Given an integer `n`, return ''the number of the beautiful arrangements that you can construct''.
   *
   * '''Constraints:'''
   *  - `1 <= n <= 15`
   */
  object Solution {
    def countArrangement(n: Int): Int = {
      def isBeautiful(n: Int, i: Int): Boolean = n % i == 0 || i % n == 0

      def bts(ns: Seq[Int], i: Int): Int = {
        if (ns.isEmpty) 1
        else ns.filter(isBeautiful(_, i + 1)).map(n => bts(ns.filterNot(_ == n), i + 1)).sum
      }
      bts(1 to n, 0)
    }
  }

  object SolutionBruteForce {
    def countArrangement(n: Int): Int = {
      def isBeautiful(arr: Seq[Int]): Boolean =
        arr.zipWithIndex.forall { case (n, i) => n % (i + 1) == 0 || (i + 1) % n == 0 }

      def bts(ns: Seq[Int], arr: Seq[Int]): Int = {
        if (ns.isEmpty)
          if (isBeautiful(arr)) 1
          else 0
        else ns.map(n => bts(ns.filterNot(_ == n), arr.appended(n))).sum
      }
      bts(1 to n, Seq())
    }
  }

  import Solution.countArrangement

  "Example 2: (n = 1) -> 1" in {
    countArrangement(1) shouldBe 1
  }
  "Example 1: (n = 2) -> 2" in {
    countArrangement(2) shouldBe 2
    //Explanation:
    // The first beautiful arrangement is [1, 2]:
    // Number at the 1st position (i=1) is 1, and 1 is divisible by i (i=1).
    // Number at the 2nd position (i=2) is 2, and 2 is divisible by i (i=2).
    // The second beautiful arrangement is [2, 1]:
    // Number at the 1st position (i=1) is 2, and 2 is divisible by i (i=1).
    // Number at the 2nd position (i=2) is 1, and i (i=2) is divisible by 1.
  }
  "(n = 3) -> 3" in (countArrangement(3) shouldBe 3)
  "(n = 4) -> 8" in (countArrangement(4) shouldBe 8)
  "(n = 5) -> 10" in (countArrangement(5) shouldBe 10)
  "(n = 10) -> 700" in (countArrangement(10) shouldBe 700)
  "(n = 15) -> 24679" in (countArrangement(15) shouldBe 24679)

}
