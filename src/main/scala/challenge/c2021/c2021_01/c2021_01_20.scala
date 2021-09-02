package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3610/]]
 */
class c2021_01_20 extends AnyWordSpec with Matchers {
  /**
   * === Valid Parentheses ===
   *
   * Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`,
   * determine if the input string is valid.
   *
   * An input string is valid if:
   *  1. Open brackets must be closed by the same type of brackets.
   *  1. Open brackets must be closed in the correct order.
   *
   * '''Constraints:'''
   *  - `1 <= s.length <= 10_000`
   *  - `s` consists of parentheses only `'()[]{}'`.
   */
  object Solution {
    def isValid(s: String): Boolean = {
      @scala.annotation.tailrec
      def rec(i: Int, q: List[Char]): Boolean =
        if (i == s.length) q.isEmpty
        else s(i) match {
          case c@('(' | '[' | '{') => rec(i + 1, c :: q)
          case _ if q.isEmpty      => false
          case c                   => (q.head, c) match {
            case ('(', ')') | ('[', ']') | ('{', '}') => rec(i + 1, q.tail)
            case _                                    => false
          }
        }

      rec(0, Nil)
    }
  }

  import Solution.isValid

  """Example 1: (s = "()") -> true""" in {
    isValid("()") shouldBe true
  }
  """Example 2: (s = "()[]{}") -> true""" in {
    isValid("()[]{}") shouldBe true
  }
  """Example 3: (s = "(]") -> false""" in {
    isValid("(]") shouldBe false
  }
  """Example 4: (s = "([)]") -> false""" in {
    isValid("([)]") shouldBe false
  }
  """Example 5: (s = "{[]}") -> true""" in {
    isValid("{[]}") shouldBe true
  }

  """(s = "(") -> false""" in {
    isValid("(") shouldBe false
  }
  """(s = ")") -> false""" in {
    isValid(")") shouldBe false
  }
}
