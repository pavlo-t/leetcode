package challenge.c2021.c2021_07

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/608/week-1-july-1st-july-7th/3799/]]
 */
class c2021_07_01 extends AnyWordSpec with Matchers {
  /**
   * == Gray Code ==
   *
   * An '''n-bit gray code sequence''' is a sequence of `2^n` integers where:
   *  - Every integer is in the '''inclusive''' range `[0, 2^n - 1]`,
   *  - The first integer is `0`,
   *  - An integer appears '''no more than once''' in the sequence,
   *  - The binary representation of every pair of '''adjacent''' integers differs by '''exactly one bit''', and
   *  - The binary representation of the '''first''' and '''last''' integers differs by '''exactly one bit'''.
   *
   * Given an integer `n`, return ''any valid '''n-bit gray code sequence'''''.
   *
   * '''Constraints:'''
   *  - `1 <= n <= 16`
   */
  object Solution {
    def grayCode(n: Int): List[Int] = {
      val m = 1 << n
      @scala.annotation.tailrec
      def rec(i: Int, rsf: List[Int]): List[Int] =
        if (i == m) rsf.reverse
        else rec(i + 1, (i ^ (i >> 1)) :: rsf)
      rec(0, Nil)
    }
  }

  import Solution.grayCode

  "(n = 1) -> [0,1]" in (grayCode(1) shouldBe List(0, 1))
  "(n = 2) -> [0,1,3,2]" in {
    grayCode(2) shouldBe List(0, 1, 3, 2)
    //Explanation:
    //The binary representation of [0,1,3,2] is [00,01,11,10].
    //- 00 and 01 differ by one bit
    //- 01 and 11 differ by one bit
    //- 11 and 10 differ by one bit
    //- 10 and 00 differ by one bit
    //[0,2,3,1] is also a valid gray code sequence, whose binary representation is [00,10,11,01].
    //- 00 and 10 differ by one bit
    //- 10 and 11 differ by one bit
    //- 11 and 01 differ by one bit
    //- 01 and 00 differ by one bit
  }
  "(n = 3) -> [0,1,3,2,6,7,5,4]" in (grayCode(3) shouldBe List(0, 1, 3, 2, 6, 7, 5, 4))
  "(n = 4) -> [0,1,3,2,6,7,5,4,12,13,15,14,10,11,9,8]" in (
    grayCode(4) shouldBe List(0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8))
  "(n = 5) -> [0,1,3,2,6,7,5,4,12,13,15,14,10,11,9,8,24,25,27,26,30,31,29,28,20,21,23,22,18,19,17,16]" in (
    grayCode(5) shouldBe List(0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8, 24, 25, 27, 26, 30, 31, 29, 28, 20, 21, 23, 22, 18, 19, 17, 16))

}
