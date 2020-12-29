package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3581/]]
 */
//noinspection DuplicatedCode
class c2020_12_26 extends AnyWordSpec with Matchers {
  /**
   * === Decode Ways ===
   *
   * A message containing letters from `A-Z` is being encoded to numbers using the following mapping:
   *
   * {{{
   * 'A' -> 1
   * 'B' -> 2
   * ...
   * 'Z' -> 26
   * }}}
   *
   * Given a '''non-empty''' string containing only digits, determine the total number of ways to decode it.
   *
   * The answer is guaranteed to fit in a '''32-bit''' integer.
   *
   * '''Constraints:'''
   *  - `1 <= s.length <= 100`
   *  - `s` contains only digits and may contain leading zero(s).
   */
  object Solution {
    def numDecodings(s: String): Int = {
      val cache = Array.fill(s.length)(-1)

      def rec(i: Int): Int =
        if (i == s.length) 1
        else if (s(i) == '0') 0
        else if (i == s.length - 1) 1
        else if (cache(i) != -1) cache(i)
        else {
          cache(i) =
            if (s(i) > '2' || (s(i) == '2' && s(i + 1) > '6')) rec(i + 1)
            else rec(i + 1) + rec(i + 2)
          cache(i)
        }

      rec(0)
    }
  }

  object SolutionRecCacheMapStringInt {
    def numDecodings(s: String): Int = {
      val cache = collection.mutable.Map[String, Int]()

      def rec(s: String): Int =
        if (s.isEmpty) 1
        else if (s(0) == '0') 0
        else if (s.length == 1) 1
        else if (cache.contains(s)) cache(s)
        else {
          cache(s) =
            if (s(0) > '2' || (s(0) == '2' && s(1) > '6')) rec(s.tail)
            else rec(s.tail) + rec(s.tail.tail)
          cache(s)
        }

      rec(s)
    }
  }

  object SolutionBruteForce {
    def numDecodings(s: String): Int = {
      if (s.isEmpty) 1
      else if (s(0) == '0') 0
      else if (s.length == 1) 1
      else if (s(0) == '0' || s(0) > '2' || (s(0) == '2' && s(1) > '6')) numDecodings(s.tail)
      else numDecodings(s.tail) + numDecodings(s.tail.tail)
    }
  }

  import Solution.numDecodings

  """Example 1: (s = "12") -> 2""" in {
    numDecodings("12") shouldBe 2
    //Explanation: It could be decoded as "AB" (1 2) or "L" (12).
  }
  """Example 2: (s = "226") -> 3""" in {
    numDecodings("226") shouldBe 3
    //Explanation: It could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
  }
  """Example 3: (s = "0") -> 0""" in {
    numDecodings("0") shouldBe 0
    //Explanation:
    // There is no character that is mapped to a number starting with '0'.
    // We cannot ignore a zero when we face it while decoding.
    // So, each '0' should be part of "10" --> 'J' or "20" --> 'T'.
  }
  """Example 4: (s = "1") -> 1""" in (numDecodings("1") shouldBe 1)

  """Test 200: (s = "1201234") -> 3""" in (numDecodings("1201234") shouldBe 3)
  """Test 235: (s = "27") -> 1""" in (numDecodings("27") shouldBe 1)

  // @formatter:off
  """(s = "1")       ->  1""" in (numDecodings("1")       shouldBe  1)
  """(s = "11")      ->  2""" in (numDecodings("11")      shouldBe  2)
  """(s = "111")     ->  3""" in (numDecodings("111")     shouldBe  3)
  """(s = "1111")    ->  5""" in (numDecodings("1111")    shouldBe  5)
  """(s = "11111")   ->  8""" in (numDecodings("11111")   shouldBe  8)
  """(s = "111111")  -> 13""" in (numDecodings("111111")  shouldBe 13)
  """(s = "1111111") -> 21""" in (numDecodings("1111111") shouldBe 21)

  """(s = "000011") -> 0""" in (numDecodings("000011") shouldBe 0)
  """(s = "10101010") -> 0""" in (numDecodings("10101010") shouldBe 1)

  """(s = "3" * 100) -> 1""" in (numDecodings("3" * 100) shouldBe 1)
  """(s = "1" * 100) -> ???""" in (numDecodings("1" * 100) shouldBe -1869596475)
  // @formatter:on
}
