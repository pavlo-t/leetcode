package challenge.c2021_06

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/606/week-4-june-22nd-june-28th/3794/]]
 */
class c2021_06_28 extends AnyWordSpec with Matchers {

  /**
   * == Remove All Adjacent Duplicates In String ==
   *
   * You are given a string `s` consisting of lowercase English letters.
   * A '''duplicate removal''' consists of choosing two ''adjacent'' and ''equal'' letters and removing them.
   *
   * We repeatedly make '''duplicate removals''' on `s` until we no longer can.
   *
   * Return ''the final string after all such duplicate removals have been made''.
   * It can be proven that the answer is '''unique'''.
   *
   * '''Constraints:'''
   *  - `1 <= s.length <= 100_000`
   *  - `s` consists of lowercase English letters.
   */
  object Solution {
    def removeDuplicates(s: String): String = {
      val sb = new StringBuilder()
      s.foreach(c => if (sb.nonEmpty && sb.last == c) sb.deleteCharAt(sb.size - 1) else sb.addOne(c))
      sb.toString()
    }

    def removeDuplicatesMy(s: String): String = {
      val stack = collection.mutable.Stack[Char]()
      s.foreach(c => if (stack.nonEmpty && stack.top == c) stack.pop() else stack.push(c))

      val sb = new StringBuilder()
      stack.reverseIterator.foreach(sb.addOne)
      sb.toString()
    }
  }

  import Solution.removeDuplicates

  """Example 1: (s = "abbaca") -> "ca"""" in {
    removeDuplicates("abbaca") shouldBe "ca"
    // Explanation: For example,
    // in "abbaca" we could remove "bb" since the letters are adjacent and equal, and this is the only possible move.
    // The result of this move is that the string is "aaca",
    // of which only "aa" is possible, so the final string is "ca".
  }
  """Example 2: (s = "azxxzy") -> "ay"""" in {
    removeDuplicates("azxxzy") shouldBe "ay"
  }

  "performance" should {
    """(s = 100_000_a) -> """"" in {
      removeDuplicates("a" * 100000) shouldBe ""
    }
    """(s = 100_000 chars) -> 100_000 chars""" in {
      val sb = new StringBuilder()
      (0 until 100000).map(i => ((i % 27) + 'a').toChar).foreach(sb.addOne)
      val s = sb.toString()
      removeDuplicates(s) shouldBe s
    }
  }

}
