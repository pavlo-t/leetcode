package contest.w216

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


/**
 * [[https://leetcode.com/contest/weekly-contest-216/problems/smallest-string-with-a-given-numeric-value/]]
 */
//noinspection DuplicatedCode
class w216_2 extends AnyWordSpec with Matchers {

  /**
   * === 5606. Smallest String With A Given Numeric Value ===
   *
   * The numeric value of a lowercase character is defined as its position (1-indexed) in the alphabet,
   * so the numeric value of a is 1, the numeric value of b is 2, the numeric value of c is 3, and so on.
   *
   * The numeric value of a string consisting of lowercase characters is defined as the sum of its
   * characters' numeric values.
   * For example, the numeric value of the string "abe" is equal to 1 + 2 + 5 = 8.
   *
   * You are given two integers n and k.
   * Return the lexicographically smallest string with length equal to n and numeric value equal to k.
   *
   * Note that a string x is lexicographically smaller than string y if x comes before y in dictionary order,
   * that is, either x is a prefix of y, or if i is the first position such that x[i] != y[i],
   * then x[i] comes before y[i] in alphabetic order.
   *
   * Constraints:
   *  - 1 <= n <= 100_000
   *  - n <= k <= 26 * n
   */
  object Solution {
    def getSmallestString(n: Int, k: Int): String = {
      var zs = k / 26
      while (k - zs * 26 < n - zs) {
        zs -= 1
      }
      var midChar = 0
      while (k - (zs * 26 + midChar) > n - zs - (if (midChar > 0) 1 else 0)) {
        midChar += 1
      }
      val as = n - zs - (if (midChar > 0) 1 else 0)

      "a" * as + (if (midChar > 0) ('a' - 1 + midChar).toChar else "") + "z" * zs
    }
  }
  object Solution2 {
    def getSmallestString(n: Int, k: Int): String = {
      val sb = new StringBuilder
      var sum = n
      for (_ <- 1 to n) sb.addOne('a')

      var i = n - 1
      while (sum < k) {
        if (sb(i) == 'z') i -= 1
        else if (k - sum >= 25) {
          sb.update(i, 'z')
          sum += 25
        } else {
          sb.update(i, (sb(i) + 1).toChar)
          sum += 1
        }
      }

      sb.toString()
    }
  }

  import Solution.getSmallestString

  """Example 1: (n = 3, k = 27) -> "aay"""" in {
    getSmallestString(3, 27) shouldBe "aay"
    //Explanation: The numeric value of the string is 1 + 1 + 25 = 27,
    // and it is the smallest string with such a value and length equal to 3.
  }
  """Example 2: (n = 5, k = 73) -> "aaszz"""" in {
    getSmallestString(5, 73) shouldBe "aaszz"
  }

  """(n = 1, k = 1) -> "a"""" in {
    getSmallestString(1, 1) shouldBe "a"
  }
  """(n = 1, k = 2) -> "b"""" in {
    getSmallestString(1, 2) shouldBe "b"
  }
  """(n = 1, k = 26) -> "z"""" in {
    getSmallestString(1, 26) shouldBe "z"
  }

  """(n = 63379, k = 933414) -> "z" * 100000""" in {
    getSmallestString(63379, 933414) shouldBe ("a" * 28577 + "k" + "z" * 34801)
  }
  """(n = 96014, k = 2095650) -> "z" * 100000""" in {
    getSmallestString(96014, 2095650) shouldBe ("a" * 16028 + "l" + "z" * 79985)
  }

  """(n = 100_000, k = 2600_000) -> "z" * 100000""" in {
    getSmallestString(100000, 2600000) shouldBe "z" * 100000
  }
  """(n = 100_000, k = 100_000) -> "a" * 100000""" in {
    getSmallestString(100000, 100000) shouldBe "a" * 100000
  }
  """(n = 100_000, k = 300_000) -> "a" * 100000""" in {
    getSmallestString(100000, 300000) shouldBe ("a" * 92000 + "z" * 8000)
  }
}
