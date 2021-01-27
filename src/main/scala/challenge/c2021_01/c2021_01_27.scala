package challenge.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3618/]]
 */
class c2021_01_27 extends AnyWordSpec with Matchers {
  /**
   * === Concatenation of Consecutive Binary Numbers ===
   *
   * Given an integer `n`, return ''the '''decimal value''' of the binary string formed by concatenating
   * the binary representations of'' `1` ''to'' `n` ''in order, '''modulo''''' `10^9 + 7`.
   *
   * '''Constraints:'''
   *  - `1 <= n <= 100_000`
   */
  object Solution {
    def concatenatedBinary(n: Int): Int = {
      @scala.annotation.tailrec
      def rec(i: Int, shift: Int, rsf: Long): Int =
        if (i > n) rsf.toInt
        else if (i == 1 << shift) rec(i, shift + 1, rsf)
        else rec(i + 1, shift, ((rsf << shift) + i) % 1_000_000_007)

      rec(1, 1, 0)
    }
  }

  object SolutionMy {
    def concatenatedBinary(n: Int): Int = {
      @scala.annotation.tailrec
      def rec(i: Int, rsf: Long): Int =
        if (i > n) rsf.toInt
        else rec(i + 1, ((rsf << i.toBinaryString.length) + i) % 1_000_000_007)

      rec(1, 0)
    }
  }

  import Solution.concatenatedBinary

  "(n =  1) ->  1" in (concatenatedBinary(1) shouldBe 1)
  //Explanation: "1" in binary corresponds to the decimal value 1.
  "(n =  2) ->  6" in (concatenatedBinary(2) shouldBe 6)
  "(n =  3) -> 27" in (concatenatedBinary(3) shouldBe 27)
  //Explanation: In binary, 1, 2, and 3 corresponds to "1", "10", and "11".
  //After concatenating them, we have "11011", which corresponds to the decimal value 27.
  "(n =  4) ->       220" in (concatenatedBinary(4) shouldBe 220)
  "(n =  5) ->      1765" in (concatenatedBinary(5) shouldBe 1765)
  "(n =  6) ->     14126" in (concatenatedBinary(6) shouldBe 14126)
  "(n =  7) ->    113015" in (concatenatedBinary(7) shouldBe 113015)
  "(n =  8) ->   1808248" in (concatenatedBinary(8) shouldBe 1808248)
  "(n =  9) ->  28931977" in (concatenatedBinary(9) shouldBe 28931977)
  "(n = 10) -> 462911642" in (concatenatedBinary(10) shouldBe 462911642)
  "(n = 11) -> 406586234" in (concatenatedBinary(11) shouldBe 406586234)
  "(n = 12) -> 505379714" in (concatenatedBinary(12) shouldBe 505379714)
  //Explanation: The concatenation results in "1101110010111011110001001101010111100".
  //The decimal value of that is 118505380540.
  //After modulo 10^9 + 7, the result is 505379714.
  "(n = 13) ->  86075381" in (concatenatedBinary(13) shouldBe 86075381)
  "(n = 14) -> 377206103" in (concatenatedBinary(14) shouldBe 377206103)
  "(n = 15) ->  35297621" in (concatenatedBinary(15) shouldBe 35297621)

  "(n =  10_000) -> 356435599" in (concatenatedBinary(10_000) shouldBe 356435599)
  "(n = 100_000) -> 757631812" in (concatenatedBinary(100_000) shouldBe 757631812)

}
