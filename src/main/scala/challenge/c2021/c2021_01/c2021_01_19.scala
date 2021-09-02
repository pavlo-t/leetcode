package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3609/]]
 */
class c2021_01_19 extends AnyWordSpec with Matchers {
  /**
   * === Longest Palindromic Substring ===
   *
   * Given a string `s`, return ''the longest palindromic substring'' in `s`.
   *
   * '''Constraints:'''
   *  - `1 <= s.length <= 1000`
   *  - `s` consist of only digits and English letters (lower-case and/or upper-case),
   */
  object Solution {
    // https://en.wikipedia.org/wiki/Longest_palindromic_substring#Manacher's_algorithm
    def longestPalindrome(s: String): String = {
      //val S = s.mkString("|", "|", "|")
      //val p = Array.fill(S.length)(0)
      //
      //// Track the following indices into P or S'
      //var R = 0 // The next element to be examined; index into S
      //var C = 0 // The largest/left-most palindrome whose right boundary is R-1; index into S
      //var i = 1 // The next palindrome to be calculated; index into P
      ////define L  = i − (R − i) // Character candidate for comparing with R; index into S
      ////define i' = C − (i − C) // The palindrome mirroring i from C; index into P
      //
      //while (R < S.length) {
      //  if i is within the palindrome at C (Cases 1 and 2):
      //  Set P[i] = P[i'] // note: recall P is initialized to all 0s
      //
      //  // Expand the palindrome at i (primarily Cases 2 and 3; can be skipped in Case 1,
      //  // though we have already shown that S'[R] ≠ S'[L] because otherwise the palindrome
      //  // at i' would have extended at least to the left edge of the palindrome at C):
      //  while S'[R] == S'[L]:
      //    increment P[i]
      //  increment R
      //
      //  If the palindrome at i extends past the palindrome at C:
      //    update C = i
      //
      //  Else increment i
      //}
      //
      //return max(P)
      s
    }
  }

  object SolutionMyBruteForce {
    def longestPalindrome(s: String): String = {
      @scala.annotation.tailrec
      def isPalindrome(l: Int, r: Int): Boolean =
        if (l > r) true
        else if (s(l) != s(r)) false
        else isPalindrome(l + 1, r - 1)

      @scala.annotation.tailrec
      def rec(l: Int, r: Int, rsf: (Int, Int)): String = {
        if (l == s.length) s.substring(rsf._1, rsf._2 + 1)
        else if (r == s.length) rec(l + 1, l + 1, rsf)
        else if (isPalindrome(l, r) && r - l > rsf._2 - rsf._1) rec(l, r + 1, (l, r))
        else rec(l, r + 1, rsf)
      }

      rec(0, 1, (0, 0))
    }
  }

  import Solution.longestPalindrome

  """Example 1: (s = "babad") -> "bab"""" in {
    longestPalindrome("babad") should (be("bab") or be("aba"))
    //Note: "aba" is also a valid answer.
  }
  """Example 2: (s = "cbbd") -> "bb"""" in {
    longestPalindrome("cbbd") shouldBe "bb"
  }
  """Example 3: (s = "a") -> "a"""" in {
    longestPalindrome("a") shouldBe "a"
  }
  """Example 4: (s = "ac") -> "a""""" in {
    longestPalindrome("ac") should (be("a") or be("c"))
  }

  """(s = "aaaaac") -> "aaaaa""""" in {
    longestPalindrome("aaaaac") shouldBe "aaaaa"
  }

  """(s = "a" * 1000) -> "a" * 1000"""" in {
    longestPalindrome("a" * 1000) shouldBe "a" * 1000
  }

  //"isPalindrome" should {
  //  def isPalindrome(s: String): Boolean = {
  //    @scala.annotation.tailrec
  //    def rec(l: Int, r: Int): Boolean =
  //      if (l > r) true
  //      else if (s(l) != s(r)) false
  //      else rec(l + 1, r - 1)
  //
  //    rec(0, s.length - 1)
  //  }
  //  """isPalindrome("aba") -> true""" in (isPalindrome("aba") shouldBe true)
  //  """isPalindrome("aa") -> true""" in (isPalindrome("aa") shouldBe true)
  //  """isPalindrome("ab") -> false""" in (isPalindrome("ab") shouldBe false)
  //  """isPalindrome("a") -> true""" in (isPalindrome("a") shouldBe true)
  //}

}
