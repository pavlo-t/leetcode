package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/564/week-1-november-1st-november-7th/3518/]]
 */
//noinspection DuplicatedCode
class d2020_11_03 extends AnyWordSpec with Matchers {

  /**
   * <h3>Consecutive Characters</h3>
   *
   * Given a string `s`, the power of the string is the maximum length of a non-empty substring that contains only
   * one unique character.
   *
   * Return <em>the power</em> of the string.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= s.length <= 500`
   * <li> `s` contains only lowercase English letters.
   * </ul>
   */
  object Solution {
    def maxPower(s: String): Int =
      s.foldLeft((1, 1, ' ')) { case ((maxL, curL, prevC), c) =>
        if (c == prevC) {
          val newL = curL + 1
          (maxL max newL, newL, c)
        } else (maxL max curL, 1, c)
      }._1
  }
  object SolutionRec {
    def maxPower(s: String): Int = {
      @scala.annotation.tailrec
      def loop(i: Int, maxL: Int, curL: Int, prev: Char): Int = {
        if (i >= s.length) maxL max curL
        else if (s(i) == prev) {
          val nextL = curL + 1
          loop(i + 1, maxL max nextL, nextL, prev)
        } else loop(i + 1, maxL max curL, 1, s(i))
      }
      loop(1, 1, 1, s.head)
    }
  }
  object SolutionImperative {
    def maxPower(s: String): Int = {
      var max = 0
      var current = 0
      var lastChar = ','
      s.foreach { c =>
        if (c != lastChar) {
          lastChar = c
          if (current > max)
            max = current
          current = 1
        } else {
          current += 1
        }
      }
      if (current > max) max = current
      max
    }
  }

  import Solution.maxPower

  """Example 1: ("leetcode") -> 2""" in {
    maxPower("leetcode") shouldBe 2
    // Explanation: The substring "ee" is of length 2 with the character 'e' only.
  }
  """Example 2: ("abbcccddddeeeeedcba") -> 5""" in {
    maxPower("abbcccddddeeeeedcba") shouldBe 5
    // Explanation: The substring "eeeee" is of length 5 with the character 'e' only.
  }
  """Example 3: ("triplepillooooow") -> 5""" in {
    maxPower("triplepillooooow") shouldBe 5
  }
  """Example 4: ("hooraaaaaaaaaaay") -> 11""" in {
    maxPower("hooraaaaaaaaaaay") shouldBe 11
  }
  """Example 5: ("tourist") -> 1""" in {
    maxPower("tourist") shouldBe 1
  }

  """("a") -> 1""" in {
    maxPower("a") shouldBe 1
  }
  "(max length) -> 1" in {
    val sb = new StringBuilder
    for (i <- 0 until 500) sb.addOne((i + 'a').toChar)

    maxPower(sb.toString()) shouldBe 1
  }
  "(max length) -> 500" in {
    val sb = new StringBuilder
    for (_ <- 0 until 500) sb.addOne('a')

    maxPower(sb.toString()) shouldBe 500
  }

}
