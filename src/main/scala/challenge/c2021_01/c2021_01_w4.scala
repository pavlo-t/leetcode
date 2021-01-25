package challenge.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3612/]]
 */
//noinspection NameBooleanParameters,DuplicatedCode
class c2021_01_w4 extends AnyWordSpec with Matchers {
  /**
   * === One Edit Distance ===
   *
   * Given two strings `s` and `t`, return `true` if they are both one edit distance apart, otherwise return `false`.
   *
   * A string `s` is said to be one distance apart from a string `t` if you can:
   *  - Insert '''exactly one''' character into `s` to get `t`.
   *  - Delete '''exactly one''' character from `s` to get `t`.
   *  - Replace '''exactly one''' character of `s` with '''a different character''' to get `t`.
   *
   * '''Constraints:'''
   *  - `0 <= s.length <= 10_000`
   *  - `0 <= t.length <= 10_000`
   *  - `s` and `t` consist of lower-case letters, upper-case letters '''and/or''' digits.
   */
  object Solution {
    import scala.annotation.tailrec

    @tailrec
    def isOneEditDistance(s: String, t: String): Boolean =
      s.length - t.length match {
        case 0 if t.isEmpty => false
        case 1 if t.isEmpty => true
        case -1             => isOneEditDistance(t, s)

        case 0 =>
          @tailrec
          def rec(i: Int, diff: Boolean): Boolean =
            if (i == s.length) diff
            else if (s(i) == t(i)) rec(i + 1, diff)
            else if (!diff) rec(i + 1, true)
            else false
          rec(0, false)
        case 1 =>
          @tailrec
          def rec(i: Int, j: Int): Boolean =
            if (i == s.length || j == t.length) true
            else if (s(i) == t(j)) rec(i + 1, j + 1)
            else if (i == j) rec(i + 1, j)
            else false
          rec(0, 0)

        case _ => false
      }
  }

  import Solution.isOneEditDistance

  """Example 1: (s = "ab", t = "acb") -> true""" in {
    isOneEditDistance("ab", "acb") shouldBe true
    //Explanation: We can insert 'c' into s to get t.
  }
  """Example 2: (s = "", t = "") -> false""" in {
    isOneEditDistance("", "") shouldBe false
    //Explanation: We cannot get t from s by only one step.
  }
  """Example 3: (s = "a", t = "") -> true""" in {
    isOneEditDistance("a", "") shouldBe true
  }
  """Example 4: (s = "", t = "A") -> true""" in {
    isOneEditDistance("", "A") shouldBe true
  }

  """(s = "ab", t = "ab") -> false""" in {
    isOneEditDistance("ab", "ab") shouldBe false
  }
  """(s = "acb", t = "ab") -> true""" in {
    isOneEditDistance("acb", "ab") shouldBe true
  }

  """(s = "a"*10000, t = "a"*10000) -> false""" in {
    isOneEditDistance("a" * 10000, "a" * 10000) shouldBe false
  }
  """(s = "a"*9999, t = "a"*10000) -> true""" in {
    isOneEditDistance("a" * 9999, "a" * 10000) shouldBe true
  }
}
