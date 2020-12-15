package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/570/week-2-december-8th-december-14th/3565/]]
 */
//noinspection DuplicatedCode
class c2020_12_14 extends AnyWordSpec with Matchers {

  /**
   * === Palindrome Partitioning ===
   *
   * Given a string `s`, partition `s` such that every substring of the partition is a '''palindrome'''.
   * Return all possible palindrome partitioning of `s`.
   *
   * A '''palindrome''' string is a string that reads the same backward as forward.
   *
   * '''Constraints:'''
   *  - `1 <= s.length <= 16`
   *  - `s` contains only lowercase English letters.
   */
  object SolutionBacktrackingWithDp {
    import collection.mutable

    /** [[https://leetcode.com/problems/palindrome-partitioning/solution/]] */
    def partition(s: String): List[List[String]] = {
      val dp = Array.fill(s.length, s.length)(false)
      val result = mutable.ListBuffer[List[String]]()

      def dfs(start: Int, currentList: mutable.ListBuffer[String]): Unit = {
        if (start >= s.length) result += currentList.toList
        else {
          for (end <- start until s.length) {
            if (s.charAt(start) == s.charAt(end) && (end - start <= 2 || dp(start + 1)(end - 1))) {
              dp(start)(end) = true
              currentList.addOne(s.substring(start, end + 1))
              dfs(end + 1, currentList)
              currentList.dropRightInPlace(1)
            }
          }
        }
      }

      dfs(0, mutable.ListBuffer())

      result.toList
    }
  }

  object Solution {
    def partition(s: String): List[List[String]] = {
      def isPalindrome(s: String): Boolean = {
        if (s.length <= 1) true
        else {
          for (i <- 0 to s.length / 2) {
            if (s(i) != s(s.length - i - 1))
              return false
          }
          true
        }
      }

      @scala.annotation.tailrec
      def rec(i: Int, rsf: List[List[String]]): List[List[String]] = {
        if (i == s.length) rsf.filter(_.forall(isPalindrome))
        else rec(i + 1, rsf.flatMap(ss => List(ss :+ s(i).toString, ss.updated(ss.size - 1, ss.last + s(i)))))
      }

      rec(1, List(List(s(0).toString)))
    }
  }

  object SolutionMyBruteForce {
    def partition(s: String): List[List[String]] = {
      def isPalindrome(s: String): Boolean = s == s.reverse

      @scala.annotation.tailrec
      def rec(i: Int, rsf: List[List[String]]): List[List[String]] = {
        if (i == s.length) rsf.filter(_.forall(isPalindrome))
        else rec(i + 1, rsf.flatMap(ss => List(ss :+ s(i).toString, ss.updated(ss.size - 1, ss.last + s(i)))))
      }

      rec(1, List(List(s(0).toString)))
    }
  }

  import Solution.partition

  """Example 1: (s = "aab") -> [["a","a","b"],["aa","b"]]""" in {
    partition("aab") shouldBe List(List("a", "a", "b"), List("aa", "b"))
  }
  """Example 2: (s = "a") -> [["a"]]""" in {
    partition("a") shouldBe List(List("a"))
  }

  """("aaa") -> [["a","a","a"],["aa","a"],["a","aa"],["aaa"]]""" in {
    partition("aaa") shouldBe List(List("a", "a", "a"), List("a", "aa"), List("aa", "a"), List("aaa"))
  }

  """("a" * 16).size -> 2^15""" in {
    partition("a" * 16).size shouldBe 32768
  }

}
