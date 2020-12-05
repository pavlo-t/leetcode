package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/december-leetcoding-challenge/569/week-1-december-1st-december-7th/3554/]]
 */
//noinspection DuplicatedCode
class c2020_12_04 extends AnyWordSpec with Matchers {

  /**
   * === The kth Factor of n ===
   *
   * Given two positive integers `n` and `k`.
   *
   * A factor of an integer `n` is defined as an integer `i` where `n % i == 0`.
   *
   * Consider a list of all factors of `n` sorted in '''ascending order''',
   * return ''the ''`kth`'' factor'' in this list or return '''-1''' if `n` has less than `k` factors.
   *
   * '''Constraints:'''
   *  - `1 <= k <= n <= 1000`
   */
  object Solution {
    def kthFactor(n: Int, K: Int): Int = {
      var k = K
      val sqrtN = math.sqrt(n)
      val divisors = collection.mutable.ListBuffer[Int]()

      for (x <- 1 to sqrtN.toInt) {
        if (n % x == 0) {
          k -= 1
          if (k == 0)
            return x
          divisors.addOne(x)
        }
      }

      // If n is a perfect square we have to skip the duplicate in the divisor list
      if (sqrtN * sqrtN == n)
        k += 1

      if (k <= divisors.size)
        n / divisors(divisors.size - k)
      else -1
    }
  }

  object SolutionMyTailrec {
    def kthFactor(n: Int, k: Int): Int = {
      @scala.annotation.tailrec
      def loop(i: Int, fc: Int): Int =
        if (i > n) -1
        else if (n % i != 0) loop(i + 1, fc)
        else if (fc < k) loop(i + 1, fc + 1)
        else i

      loop(1, 1)
    }
  }

  import Solution.kthFactor

  "Example 1: (n = 12, k = 3) -> 3" in {
    kthFactor(12, 3) shouldBe 3
    //Explanation: Factors list is [1, 2, 3, 4, 6, 12], the 3rd factor is 3.
  }
  "Example 2: (n = 7, k = 2) -> 7" in {
    kthFactor(7, 2) shouldBe 7
    //Explanation: Factors list is [1, 7], the 2nd factor is 7.
  }
  "Example 3: (n = 4, k = 4) -> -1" in {
    kthFactor(4, 4) shouldBe -1
    //Explanation: Factors list is [1, 2, 4], there is only 3 factors. We should return -1.
  }
  "Example 4: (n = 1, k = 1) -> 1" in {
    kthFactor(1, 1) shouldBe 1
    //Explanation: Factors list is [1], the 1st factor is 1.
  }
  "Example 5: (n = 1000, k = 3) -> 4" in {
    kthFactor(1000, 3) shouldBe 4
    //Explanation: Factors list is [1, 2, 4, 5, 8, 10, 20, 25, 40, 50, 100, 125, 200, 250, 500, 1000].
  }

  "(n = 1000, k = 1000) -> -1" in {
    kthFactor(1000, 1000) shouldBe -1
  }

  "(n = 2_000_000_000, k = 10000) -> -1" in {
    kthFactor(2_000_000_000, 1000) shouldBe -1
  }

}
