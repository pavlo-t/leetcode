package challenge.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3631/]]
 */
//noinspection DuplicatedCode
class c2021_02_07 extends AnyWordSpec with Matchers {
  /**
   * === Shortest Distance to a Character ===
   *
   * Given a string `s` and a character `c` that occurs in `s`, return
   * ''an array of integers'' `answer` ''where'' `answer.length == s.length` ''and'' `answer[i]` ''is the shortest
   * distance from'' `s[i]` ''to the character'' `c` ''in'' `s`.
   *
   * '''Constraints:'''
   *  - `1 <= s.length <= 10_000`
   *  - `s[i]` and `c` are lowercase English letters.
   *  - `c` occurs at least once in `s`.
   */
  object Solution {
    def shortestToChar(s: String, c: Char): Array[Int] = {
      @scala.annotation.tailrec
      def leftDs(s: String, i: Int = 0, rsf: Seq[Int] = Seq(10_001)): Seq[Int] =
        if (i == s.length) rsf.tail
        else if (s(i) == c) leftDs(s, i + 1, rsf :+ 0)
        else leftDs(s, i + 1, rsf :+ rsf(i) + 1)

      leftDs(s)
        .zip(leftDs(s.reverse).reverse)
        .map { case (l, r) => l min r }
        .toArray
    }
  }

  object SolutionOptimized {
    def shortestToChar(s: String, c: Char): Array[Int] = {
      val rds = Array.fill(s.length + 1)(10_001)
      for (i <- s.indices.reverse)
        rds(i) =
          if (s(i) == c) 0
          else rds(i + 1) + 1

      val result = Array.fill(s.length)(10_001)
      for (i <- s.indices)
        result(i) =
          if (s(i) == c) 0
          else if (i == 0) rds(i)
          else rds(i) min (result(i - 1) + 1)

      result
    }
  }

  import Solution.shortestToChar

  """Example 1: (s = "loveleetcode", c = "e") -> [3,2,1,0,1,0,0,1,2,2,1,0]""" in {
    shortestToChar("loveleetcode", 'e') shouldBe Array(3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0)
  }
  """Example 2: (s = "aaab", c = "b") -> [3,2,1,0]""" in {
    shortestToChar("aaab", 'b') shouldBe Array(3, 2, 1, 0)
  }

  """(s = "b", c = "b") -> [0]""" in {
    shortestToChar("b", 'b') shouldBe Array(0)
  }

}
