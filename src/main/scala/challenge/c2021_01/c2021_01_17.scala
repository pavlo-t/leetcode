package challenge.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3607/]]
 */
//noinspection DuplicatedCode
class c2021_01_17 extends AnyWordSpec with Matchers {
  /**
   * === Count Sorted Vowel Strings ===
   *
   * Given an integer `n`, return ''the number of strings of length'' `n`
   * ''that consist only of vowels (''`a`, `e`, `i`, `o`, `u`'') and are '''lexicographically sorted'''.''
   *
   * A string `s` is '''lexicographically sorted''' if for all valid `i`, `s[i]`
   * is the same as or comes before `s[i+1]` in the alphabet.
   *
   * '''Constraints:'''
   *  - `1 <= n <= 50`
   */
  object Solution {
    def countVowelStrings(n: Int): Int =
      (n + 4) * (n + 3) * (n + 2) * (n + 1) / 24
  }
  object SolutionBruteForceKeepLastVowel {
    def countVowelStrings(n: Int): Int = {
      def rec(l: Int, c: Char): Int =
        if (l >= n) 1
        else Seq('a', 'e', 'i', 'o', 'u').withFilter(_ >= c).map(c => rec(l + 1, c)).sum
      rec(0, 'a')
    }
  }
  object SolutionBruteForceBuildString {
    def countVowelStrings(n: Int): Int = {
      def rec(s: String): Int =
        if (s.length >= n) 1
        else Seq('a', 'e', 'i', 'o', 'u').withFilter(_ >= s.lastOption.getOrElse('a')).map(c => rec(s.appended(c))).sum
      rec("")
    }
  }

  import Solution.countVowelStrings

  "Example 1: (n = 1) -> 5" in {
    countVowelStrings(1) shouldBe 5
    //Explanation: The 5 sorted strings that consist of vowels only are ["a","e","i","o","u"].
  }
  "Example 2: (n = 2) -> 15" in {
    countVowelStrings(2) shouldBe 15
    //Explanation: The 15 sorted strings that consist of vowels only are
    //["aa","ae","ai","ao","au","ee","ei","eo","eu","ii","io","iu","oo","ou","uu"].
    //Note that "ea" is not a valid string since 'e' comes after 'a' in the alphabet.
  }
  "Example 3: (n = 33) -> 66045" in {
    countVowelStrings(33) shouldBe 66045
  }
  "(n = 50) -> 316251" in {
    countVowelStrings(50) shouldBe 316251
  }

}
