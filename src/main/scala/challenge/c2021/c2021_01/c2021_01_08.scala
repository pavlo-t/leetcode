package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3597/]]
 */
//noinspection DuplicatedCode
class c2021_01_08 extends AnyWordSpec with Matchers {
  /**
   * === Check If Two String Arrays are Equivalent ===
   *
   * Given two string arrays `word1` and `word2`,
   * return `true` ''if the two arrays '''represent''' the same string, and'' `false` ''otherwise''.
   *
   * A string is '''represented''' by an array if the array elements concatenated '''in order''' forms the string.
   *
   * '''Constraints:'''
   *  - `1 <= word1.length, word2.length <= 1000`
   *  - `1 <= word1[i].length, word2[i].length <= 1000`
   *  - `1 <= sum(word1[i].length), sum(word2[i].length) <= 1000`
   *  - `word1[i]` and `word2[i]` consist of lowercase letters.
   */
  object SolutionRecO1Space {
    def arrayStringsAreEqual(w1: Array[String], w2: Array[String]): Boolean = {
      @scala.annotation.tailrec
      def rec(i1: Int, j1: Int, i2: Int, j2: Int): Boolean = {
        (i1 == w1.length, i2 == w2.length) match {
          case (true, true)   => true
          case (false, false) => (j1 == w1(i1).length, j2 == w2(i2).length) match {
            case (false, false) if w1(i1)(j1) == w2(i2)(j2) => rec(i1, j1 + 1, i2, j2 + 1)
            case (true, true)                               => rec(i1 + 1, 0, i2 + 1, 0)
            case (true, false)                              => rec(i1 + 1, 0, i2, j2)
            case (false, true)                              => rec(i1, j1, i2 + 1, 0)
            case _                                          => false
          }
          case _              => false
        }
      }

      rec(0, 0, 0, 0)
    }
  }

  object Solution {
    def arrayStringsAreEqual(word1: Array[String], word2: Array[String]): Boolean = {
      @scala.annotation.tailrec
      def rec(i1: Int, j1: Int, i2: Int, j2: Int): Boolean = {
        if (i1 == word1.length && i2 == word2.length)
          true
        else if (i1 == word1.length || i2 == word2.length)
          false

        else if (j1 == word1(i1).length && j2 == word2(i2).length)
          rec(i1 + 1, 0, i2 + 1, 0)
        else if (j1 == word1(i1).length)
          rec(i1 + 1, 0, i2, j2)
        else if (j2 == word2(i2).length)
          rec(i1, j1, i2 + 1, 0)

        else if (word1(i1)(j1) == word2(i2)(j2))
          rec(i1, j1 + 1, i2, j2 + 1)

        else
          false
      }

      rec(0, 0, 0, 0)
    }
  }

  object SolutionMkString {
    def arrayStringsAreEqual(word1: Array[String], word2: Array[String]): Boolean =
      word1.mkString("") == word2.mkString("")
  }

  import Solution.arrayStringsAreEqual

  """Example 1: (word1 = ["ab", "c"], word2 = ["a", "bc"]) -> true""" in {
    val word1 = Array("ab", "c")
    val word2 = Array("a", "bc")

    arrayStringsAreEqual(word1, word2) shouldBe true
    //Explanation:
    // word1 represents string "ab" + "c" -> "abc"
    // word2 represents string "a" + "bc" -> "abc"
    // The strings are the same, so return true.
  }
  """Example 2: (word1 = ["a", "cb"], word2 = ["ab", "c"]) -> false""" in {
    val word1 = Array("a", "cb")
    val word2 = Array("ab", "c")

    arrayStringsAreEqual(word1, word2) shouldBe false
  }
  """Example 3: (word1 = ["abc", "d", "defg"], word2 = ["abcddefg"]) -> true""" in {
    val word1 = Array("abc", "d", "defg")
    val word2 = Array("abcddefg")

    arrayStringsAreEqual(word1, word2) shouldBe true
  }

  """(word1 = ["a",...,"a"] 1000, word2 = ["a",...,"a"]) -> true""" in {
    val size = 1000
    val word1 = Array.fill(size)("a")
    val word2 = Array.fill(size)("a")

    arrayStringsAreEqual(word1, word2) shouldBe true
  }
  """(word1 = ["a",...,"a"] 1000, word2 = ["a",...,"b"]) -> false""" in {
    val size = 1000
    val word1 = Array.fill(size)("a")
    val word2 = Array.fill(size)("a")
    word2(word2.length - 1) = "b"

    arrayStringsAreEqual(word1, word2) shouldBe false
  }

  """(word1 = ["a",...,"a"] 1_000_000, word2 = ["a",...,"a"]) -> true""" in {
    val size = 10_000_000
    val word1 = Array.fill(size)("a")
    val word2 = Array.fill(size)("a")

    arrayStringsAreEqual(word1, word2) shouldBe true
  }
  """(word1 = ["a",...,"a"] 1_000_000, word2 = ["a",...,"b"]) -> false""" in {
    val size = 10_000_000
    val word1 = Array.fill(size)("a")
    val word2 = Array.fill(size)("a")
    word2(word2.length - 1) = "b"

    arrayStringsAreEqual(word1, word2) shouldBe false
  }

}
