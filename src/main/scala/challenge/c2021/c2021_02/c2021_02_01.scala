package challenge.c2021.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3625/]]
 */
//noinspection DuplicatedCode
class c2021_02_01 extends AnyWordSpec with Matchers {
  /**
   * === Number of 1 Bits ===
   *
   * Write a function that takes an unsigned integer and returns the number of '1' bits it has
   * (also known as the [[https://en.wikipedia.org/wiki/Hamming_weight Hamming weight]]).
   *
   * '''Note:'''
   *  - Note that in some languages such as Java, there is no unsigned integer type.
   *    In this case, the input will be given as a signed integer type.
   *    It should not affect your implementation, as the integer's internal binary representation is the same,
   *    whether it is signed or unsigned.
   *  - In Java, the compiler represents the signed integers using
   *    [[https://en.wikipedia.org/wiki/Two%27s_complement 2's complement notation]].
   *    Therefore, in '''Example 3''', the input represents the signed integer, `-3`.
   *
   * '''Follow up''': If this function is called many times, how would you optimize it?
   *
   * '''Constraints:'''
   *  - The input must be a '''binary string''' of length `32`
   */
  object Solution {
    // you need treat n as an unsigned value
    def hammingWeight(n: Int): Int = {
      @scala.annotation.tailrec
      def rec(n: Int, rsf: Int): Int =
        if (n == 0) rsf
        else rec(n & (n - 1), rsf + 1)
      rec(n, 0)
    }
  }

  object SolutionLoopTailrec {
    // you need treat n as an unsigned value
    def hammingWeight(n: Int): Int = {
      @scala.annotation.tailrec
      def rec(n: Int, rsf: Int): Int =
        if (n == 0) rsf
        else rec(n >>> 1, rsf + (n & 1))
      rec(n, 0)
    }
  }
  object SolutionLoopRec {
    // you need treat n as an unsigned value
    def hammingWeight(n: Int): Int = {
      def rec(n: Int): Int =
        if (n == 0) 0
        else (n & 1) + rec(n >>> 1)
      rec(n)
    }
  }

  object SolutionScalaBuiltin {
    // you need treat n as an unsigned value
    def hammingWeight(n: Int): Int = n.toBinaryString.count(_ != '0')
  }

  import Solution.hammingWeight

  "Example 1: (n = 00000000000000000000000000001011) -> 3" in {
    hammingWeight(11) shouldBe 3
    //Explanation: The input binary string 00000000000000000000000000001011 has a total of three '1' bits.
  }
  "Example 2: (n = 00000000000000000000000010000000) -> 1" in {
    hammingWeight(128) shouldBe 1
    //Explanation: The input binary string 00000000000000000000000010000000 has a total of one '1' bit.
  }
  "Example 3: (n = 11111111111111111111111111111101) -> 31" in {
    hammingWeight(-3) shouldBe 31
    //Explanation: The input binary string 11111111111111111111111111111101 has a total of thirty one '1' bits.
  }


}
