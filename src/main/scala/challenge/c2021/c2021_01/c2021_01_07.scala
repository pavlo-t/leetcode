package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3595/]]
 */
class c2021_01_07 extends AnyWordSpec with Matchers {
  /**
   * === Longest Substring Without Repeating Characters ===
   *
   * Given a string `s`, find the length of the '''longest substring''' without repeating characters.
   *
   * '''Constraints:'''
   *  - `0 <= s.length <= 50_000`
   *  - `s` consists of English letters, digits, symbols and spaces.
   */
object Solution {
  def lengthOfLongestSubstring(s: String): Int = {
    @scala.annotation.tailrec
    def rec(l: Int, r: Int, chars: Set[Char], rsf: Int): Int = {
      if (r == s.length) rsf
      else if (chars.contains(s(r))) rec(l + 1, r, chars - s(l), rsf)
      else rec(l, r + 1, chars + s(r), rsf max (r - l + 1))
    }
    rec(0, 0, Set(), 0)
  }
}

  import Solution.lengthOfLongestSubstring

  """Example 1: (s = "abcabcbb") -> 3""" in {
    lengthOfLongestSubstring("abcabcbb") shouldBe 3
    //Explanation: The answer is "abc", with the length of 3.
  }
  """Example 2: (s = "bbbbb") -> 1""" in {
    lengthOfLongestSubstring("bbbbb") shouldBe 1
    //Explanation: The answer is "b", with the length of 1.
  }
  """Example 3: (s = "pwwkew") -> 3""" in {
    lengthOfLongestSubstring("pwwkew") shouldBe 3
    //Explanation: The answer is "wke", with the length of 3.
    //Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
  }
  """Example 4: (s = "") -> 0""" in {
    lengthOfLongestSubstring("") shouldBe 0
  }

  """(s = "a" * 50000) -> 1""" in {
    lengthOfLongestSubstring("a" * 50000) shouldBe 1
  }
  """(s = "a to z" * 1924 = 50024) -> 26""" in {
    lengthOfLongestSubstring(('a' to 'z').mkString("") * 1924) shouldBe 26
  }

}
