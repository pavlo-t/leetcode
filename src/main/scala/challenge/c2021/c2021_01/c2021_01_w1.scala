package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3588/]]
 */
//noinspection DuplicatedCode
class c2021_01_w1 extends AnyWordSpec with Matchers {
  /**
   * === Palindrome Permutation ===
   *
   * Given a string, determine if a permutation of the string could form a palindrome.
   */
  object Solution {
    import collection.immutable.BitSet

    def canPermutePalindrome(s: String): Boolean =
      s.foldLeft(BitSet()) { case (acc, c) => if (acc.contains(c)) acc - c else acc + c }.size < 2
  }

  object SolutionFoldSet {
    def canPermutePalindrome(s: String): Boolean =
      s.foldLeft(Set[Char]()) { case (acc, c) => if (acc.contains(c)) acc - c else acc + c }.size < 2
  }

  object SolutionFoldMap {
    def canPermutePalindrome(s: String): Boolean =
      s.foldLeft(Map[Char, Int]().withDefaultValue(0)) {
        case (acc, c) => acc.updated(c, acc(c) + 1)
      }.values.map(_ % 2).sum < 2
  }

  object SolutionRec {
    def canPermutePalindrome(s: String): Boolean = {
      @scala.annotation.tailrec
      def rec(i: Int, counts: Map[Char, Int]): Boolean = {
        if (i == s.length) counts.values.map(_ % 2).sum <= 1
        else rec(i + 1, counts.updated(s(i), counts.getOrElse(s(i), 0) + 1))
      }
      rec(0, Map())
    }
  }

  import Solution.canPermutePalindrome

  """Example 1: ("code") -> false""" in {
    canPermutePalindrome("code") shouldBe false
  }
  """Example 2: ("aab") -> true""" in {
    canPermutePalindrome("aab") shouldBe true
  }
  """Example 3: ("carerac") -> true""" in {
    canPermutePalindrome("carerac") shouldBe true
  }

  """("") -> true""" in {
    canPermutePalindrome("") shouldBe true
  }
  """("a") -> true""" in {
    canPermutePalindrome("a") shouldBe true
  }
  """("ab") -> false""" in {
    canPermutePalindrome("ab") shouldBe false
  }

  """("a" * 50000) -> true""" in {
    canPermutePalindrome("a" * 50000) shouldBe true
  }
  """("a to z" * 1924 = 50024) -> true""" in {
    canPermutePalindrome(('a' to 'z').mkString("") * 1924) shouldBe true
  }
  """("a to z" * 1925 = 50050) -> false""" in {
    canPermutePalindrome(('a' to 'z').mkString("") * 1925) shouldBe false
  }
}
