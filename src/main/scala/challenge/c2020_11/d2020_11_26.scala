package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/567/week-4-november-22nd-november-28th/3544/]]
 */
//noinspection DuplicatedCode
class d2020_11_26 extends AnyWordSpec with Matchers {

  /**
   * === Longest Substring with At Least K Repeating Characters ===
   *
   * Given a string `s` and an integer `k`,
   * return ''the length of the longest substring of'' `s`
   * ''such that the frequency of each character in this substring is greater than or equal to'' `k`.
   *
   * '''Constraints:'''
   *  - `1 <= s.length <= 10_000`
   *  - `s` consists of only lowercase English letters.
   *  - `1 <= k <= 100_000`
   */
  object Solution {
    def longestSubstring(s: String, k: Int): Int =
      if (s.length < k) 0
      else {
        val invalidChars =
          s.foldLeft(Map[Char, Int]())((acc, c) => acc.updated(c, acc.getOrElse(c, 0) + 1))
            .filter { case (_, count) => count > 0 && count < k }
            .keySet

        if (invalidChars.isEmpty) s.length
        else
          s.split(invalidChars.toArray)
            .map(longestSubstring(_, k))
            .maxOption.getOrElse(0)
      }
  }

  object SolutionTwoPointersImperative {
    def longestSubstring(s: String, k: Int): Int = {
      val countMap = Array.ofDim[Int](26)
      val maxUnique = s.distinct.length
      var result = 0

      for (curUnique <- 1 to maxUnique) {
        // Reset countMap
        countMap.mapInPlace(_ => 0)
        var windowStart = 0
        var windowEnd = 0
        var unique = 0
        var countAtLeastK = 0
        while (windowEnd < s.length) {
          if (unique <= curUnique) {
            // expand the sliding window
            val idx = s(windowEnd) - 'a'
            if (countMap(idx) == 0) unique += 1
            countMap(idx) += 1
            if (countMap(idx) == k) countAtLeastK += 1
            windowEnd += 1
          } else {
            // shrink the sliding window
            val idx = s(windowStart) - 'a'
            if (countMap(idx) == k) countAtLeastK -= 1
            countMap(idx) -= 1
            if (countMap(idx) == 0) unique -= 1
            windowStart += 1
          }
          if (unique == curUnique && unique == countAtLeastK)
            result = result max (windowEnd - windowStart)
        }
      }

      result
    }
  }

  object SolutionMy {
    def longestSubstring(s: String, k: Int): Int =
      if (s.length < k) 0
      else {
        @scala.annotation.tailrec
        def loop(i: Int, cs: Set[Map[Char, Int]], rsf: Int): Int =
          if (i >= s.length) rsf
          else {
            val nc1 = cs.flatMap { c =>
              if (c.contains(s(i))) Set(c.updated(s(i), c(s(i)) + 1))
              else if (cs.exists(_.keySet == (c.keySet + s(i)))) Set()
              else Set(c.updated(s(i), 1))
            }
            val ncs = if (nc1.exists(_.keySet == Set(s(i)))) nc1 else nc1 + Map(s(i) -> 1)
            val nr = rsf max ncs.filter(_.forall(_._2 >= k)).map(_.values.sum).maxOption.getOrElse(0)

            loop(i + 1, ncs, nr)
          }

        loop(0, Set(Map()), 0)
      }
  }

  import Solution.longestSubstring

  """Example 1: (s = "aaabb", k = 3) -> 3""" in {
    longestSubstring(s = "aaabb", k = 3) shouldBe 3
    //Explanation: The longest substring is "aaa", as 'a' is repeated 3 times.
  }
  """Example 2: (s = "ababbc", k = 2) -> 5""" in {
    longestSubstring(s = "ababbc", k = 2) shouldBe 5
    //Explanation: The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is repeated 3 times.
  }

  """Test 4: ("weitong", 2) -> 0""" in (longestSubstring("weitong", 2) shouldBe 0)

  """(s = "a", k = 2) -> 0""" in (longestSubstring("a", 2) shouldBe 0)
  """(s = "ababcab", k = 2) -> 4""" in (longestSubstring("ababcaba", 2) shouldBe 4)
  """(s = "cababab", k = 2) -> 6""" in (longestSubstring("cabababa", 2) shouldBe 7)

  """(s = "a" * 10_000, k = 10_001) -> 0""" in (longestSubstring("a" * 10000, 10001) shouldBe 0)
  """(s = "a" * 10_000, k = 10_000) -> 10_000""" in (longestSubstring("a" * 10000, 10000) shouldBe 10000)
  """(s = "a" * 10_000, k = 10) -> 10_000""" in (longestSubstring("a" * 10000, 10) shouldBe 10000)
  """(s = "abc..xyz" * 385, k = 1) -> 10010""" in {
    val s = (0 until 26).map(_ + 'a').map(_.toChar).mkString("") * 385
    longestSubstring(s, 1) shouldBe 10010
  }
  """(s = 10_000 random chars, k = 1) -> >= 26""" in {
    val s = {
      val sb = new StringBuilder
      for (_ <- 1 to 10000) sb.addOne((util.Random.nextInt(26) + 'a').toChar)
      sb.toString()
    }
    longestSubstring(s, 1) should be >= 26
  }

}
