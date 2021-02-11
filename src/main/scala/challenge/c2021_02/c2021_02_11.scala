package challenge.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3636/]]
 */
class c2021_02_11 extends AnyWordSpec with Matchers {
  /**
   * === Valid Anagram ===
   *
   * Given two strings `s` and `t`, write a function to determine if `t` is an anagram of `s`.
   *
   * '''Note:'''
   *
   * You may assume the string contains only lowercase alphabets.
   *
   * '''Follow up:'''
   *
   * What if the inputs contain unicode characters? How would you adapt your solution to such case?
   */
  object Solution {
    def isAnagram(s: String, t: String): Boolean = {
      def rf(f: Int => Int)(o: Option[Int]): Option[Int] = o match {
        case Some(v) => Some(f(v))
        case None    => Some(f(0))
      }
      @scala.annotation.tailrec
      def getCounts(i: Int = 0, rsf: Map[Char, Int] = Map()): Map[Char, Int] =
        if (i == s.length) rsf
        else getCounts(i + 1, rsf.updatedWith(s(i))(rf(_ + 1)).updatedWith(t(i))(rf(_ - 1)))
      s.length == t.length && getCounts().values.forall(_ == 0)
    }
  }

  object SolutionMy {
    def isAnagram(s: String, t: String): Boolean = {
      @scala.annotation.tailrec
      def getCounts(s: String, i: Int = 0, rsf: Map[Char, Int] = Map()): Map[Char, Int] =
        if (i == s.length) rsf
        else getCounts(s, i + 1, rsf.updated(s(i), rsf.getOrElse(s(i), 0) + 1))
      s.length == t.length && getCounts(s) == getCounts(t)
    }
  }

  import Solution.isAnagram

  """Example 1: (s = "anagram", t = "nagaram") -> true""" in {
    isAnagram("anagram", "nagaram") shouldBe true
  }
  """Example 2: (s = "rat", t = "car") -> false""" in {
    isAnagram("rat", "car") shouldBe false
  }

}
