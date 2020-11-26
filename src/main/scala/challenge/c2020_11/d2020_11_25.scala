package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/567/week-4-november-22nd-november-28th/3543/]]
 */
//noinspection DuplicatedCode
class d2020_11_25 extends AnyWordSpec with Matchers {

  /**
   * === Smallest Integer Divisible by K ===
   *
   * Given a positive integer `K`,
   * you need to find the '''length''' of the '''smallest''' positive integer `N` such that `N` is divisible by `K`,
   * and `N` only contains the digit `1`.
   *
   * Return ''the '''length''' of ''`N`.
   * If there is no such `N`, return `-1`.
   *
   * '''Note:''' `N` may not fit in a 64-bit signed integer.
   *
   * '''Constraints:'''
   *   - `1 <= K <= 100_000`
   *
   * @see [[https://en.wikipedia.org/wiki/Modulo_operation#Properties_(identities)]]
   */
  object Solution {
    def smallestRepunitDivByK(K: Int): Int =
      if (K % 2 == 0 || K % 5 == 0) -1
      else {
        @scala.annotation.tailrec
        def countDigits(i: Int = 1, r: Int): Int = {
          if (r % K == 0) i
          else if (i > K) -1
          else countDigits(i + 1, (r * 10 + 1) % K)
        }
        countDigits(1, 1)
      }
  }

  object SolutionMy {
    def smallestRepunitDivByK(K: Int): Int =
      if (K % 2 == 0 || K % 5 == 0) -1
      else {
        @scala.annotation.tailrec
        def countDigits(i: Int = 1, N: BigInt): Int = {
          if (N % K == 0) i
          else countDigits(i + 1, N * 10 + 1)
        }

        countDigits(1, 1)
      }
  }

  import Solution.smallestRepunitDivByK

  "Example 1: (K = 1) -> 1" in {
    smallestRepunitDivByK(1) shouldBe 1
    //Explanation: The smallest answer is N = 1, which has length 1.
  }
  "Example 2: (K = 2) -> -1" in {
    smallestRepunitDivByK(2) shouldBe -1
    //Explanation: There is no such positive integer N divisible by 2.
  }
  "Example 3: (K = 3) -> 3" in {
    smallestRepunitDivByK(3) shouldBe 3
    //Explanation: The smallest answer is N = 111, which has length 3.
  }
  "(K = 4) -> -1" in (smallestRepunitDivByK(4) shouldBe -1)
  "(K = 5) -> -1" in (smallestRepunitDivByK(5) shouldBe -1)
  "(K = 6) -> -1" in (smallestRepunitDivByK(6) shouldBe -1)
  "(K = 7) -> 6" in (smallestRepunitDivByK(7) shouldBe 6)
  "(K = 8) -> -1" in (smallestRepunitDivByK(8) shouldBe -1)
  "(K = 9) -> 9" in (smallestRepunitDivByK(9) shouldBe 9)
  "(K = 10) -> -1" in (smallestRepunitDivByK(10) shouldBe -1)

  "(K = 99233 -> 99232)" in (smallestRepunitDivByK(99233) shouldBe 99232)

  "K <- 10 to 10000" in {
    for (k <- 10 to 10000) smallestRepunitDivByK(k) should be >= -1
  }
  "K <- 90000 to 100000" in {
    for (k <- 90000 to 100000) smallestRepunitDivByK(k) should be >= -1
  }
}
