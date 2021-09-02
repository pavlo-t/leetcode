package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3613/]]
 */
class c2021_01_22 extends AnyWordSpec with Matchers {
  /**
   * === Determine if Two Strings Are Close ===
   *
   * Two strings are considered '''close''' if you can attain one from the other using the following operations:
   *  - Operation 1: Swap any two '''existing''' characters.
   *    - For example, `abcde -> aecdb`
   *  - Operation 2: Transform '''every''' occurrence of one '''existing''' character into another '''existing'''
   *    character, and do the same with the other character.
   *    - For example, `aacabb -> bbcbaa` (all `a`'s turn into `b`'s, and all `b`'s turn into `a`'s)
   *
   * You can use the operations on either string as many times as necessary.
   *
   * Given two strings, `word1` and `word2`, return `true` ''if'' `word1` ''and'' `word2` ''are '''close''',
   * and'' `false` ''otherwise''.
   *
   * '''Constraints:'''
   *  - `1 <= word1.length, word2.length <= 100_000`
   *  - `word1` and `word2` contain only lowercase English letters.
   */
  object Solution {
    def closeStrings(word1: String, word2: String): Boolean =
      if (word1.length != word2.length) false
      else {
        def count(word: String): (Seq[Char], Seq[Int]) = {
          val map = word.foldLeft(Map[Char, Int]().withDefaultValue(0))((acc, c) => acc.updated(c, acc(c) + 1))
          (map.keys.toSeq.sorted, map.values.toSeq.sorted)
        }
        count(word1) == count(word2)
      }
  }

  import Solution.closeStrings

  """Example 1: (word1 = "abc", word2 = "bca") -> true""" in {
    closeStrings("abc", "bca") shouldBe true
    //Explanation: You can attain word2 from word1 in 2 operations.
    //Apply Operation 1: "abc" -> "acb"
    //Apply Operation 2: "acb" -> "bca"
  }
  """Example 2: (word1 = "a", word2 = "aa") -> false""" in {
    closeStrings("a", "aa") shouldBe false
    //Explanation: It is impossible to attain word2 from word1, or vice versa, in any number of operations.
  }
  """Example 3: (word1 = "cabbba", word2 = "abbccc") -> true""" in {
    closeStrings("cabbba", "abbccc") shouldBe true
    //Explanation: You can attain word2 from word1 in 3 operations.
    //Apply Operation 1: "cabbba" -> "caabbb"
    //Apply Operation 2: "caabbb" -> "baaccc"
    //Apply Operation 3: "baaccc" -> "abbccc"
  }
  """Example 4: (word1 = "cabbba", word2 = "aabbss") -> false""" in {
    closeStrings("cabbba", "aabbss") shouldBe false
    //Explanation: It is impossible to attain word2 from word1, or vice versa, in any amount of operations.
  }

  """("cbbbba", "abbccc") -> false""" in {
    closeStrings("cbbbba", "abbccc") shouldBe false
  }
  """("uau", "ssx") -> false""" in {
    closeStrings("uau", "ssx") shouldBe false
  }
}
