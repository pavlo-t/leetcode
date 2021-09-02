package challenge.c2021.c2021_08

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/explore/challenge/card/august-leetcoding-challenge-2021/615/week-3-august-15th-august-21st/3891/]] */
class c2021_08_15 extends AnyWordSpec with Matchers {
  /**
   * == Minimum Window Substring ==
   *
   * Given two strings `s` and `t` of lengths `m` and `n` respectively, return ''the '''minimum window substring''' of''
   * `s` ''such that every character in'' `t` ''('''including duplicates''') is included in the window.
   * If there is no such substring, return the empty string'' `""`.
   *
   * The testcases will be generated such that the answer is '''unique'''.
   *
   * A '''substring''' is a contiguous sequence of characters within the string.
   *
   * '''Constraints:'''
   *  - `1 <= s.length, t.length <= 100_000`
   *  - `s` and `t` consist of uppercase and lowercase English letters.
   *
   * '''Follow up:''' Could you find an algorithm that runs in `O(m + n)` time?
   */
  object Solution {
    def minWindow(s: String, t: String): String = {
      def counts(s: String): Map[Char, Int] = {
        @scala.annotation.tailrec
        def tr(i: Int, rsf: Map[Char, Int]): Map[Char, Int] =
          if (i >= s.length) rsf
          else tr(i + 1, rsf.updatedWith(s(i))(_.map(_ + 1).orElse(Some(1))))
        tr(0, Map())
      }
      val tcs = counts(t)

      @inline def containsAll(cs: Map[Char, Int]): Boolean =
        tcs.forall { case (c, cnt) => cs.get(c).exists(_ >= cnt) }
      @inline def updateRsf(l: Int, r: Int, rsf: Option[(Int, Int)]): Option[(Int, Int)] =
        rsf.map { case (pl, pr) => if (pr - pl < r - l) (pl, pr) else (l, r) }.orElse(Some((l, r)))
      @inline def moveL(l: Int, cs: Map[Char, Int]): Option[Map[Char, Int]] =
        Some(cs.updatedWith(s(l))(_.filter(_ > 1).map(_ - 1)))
          .filter(containsAll)
      @inline def moveR(r: Int, cs: Map[Char, Int]): Option[Map[Char, Int]] =
        if (r >= s.length - 1) None
        else Some(cs.updatedWith(s(r + 1))(_.map(_ + 1).orElse(Some(1))))

      @scala.annotation.tailrec
      def minimize(l: Int, r: Int, cs: Map[Char, Int], rsf: Option[(Int, Int)]): Option[(Int, Int)] =
        if (cs == tcs) rsf
        else moveL(l, cs) match {
          case Some(csl) => minimize(l + 1, r, csl, updateRsf(l + 1, r, rsf))
          case _         => moveR(r, cs) match {
            case Some(csr) if containsAll(csr) => minimize(l, r + 1, csr, updateRsf(l, r + 1, rsf))
            case Some(csr)                     => minimize(l, r + 1, csr, rsf)
            case None                          => rsf
          }
        }

      val cs = Map(s(0) -> 1)
      val rsf = if (containsAll(cs)) Some((0, 0)) else None
      minimize(0, 0, cs, rsf).map { case (l, r) => s.substring(l, r + 1) }.getOrElse("")
    }
  }

  """Example 1: (s = "ADOBECODEBANC", t = "ABC") -> "BANC"""" in {
    Solution.minWindow("ADOBECODEBANC", "ABC") shouldBe "BANC"
    //Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
  }
  """Example 2: (s = "a", t = "a") -> "a"""" in {
    Solution.minWindow(s = "a", t = "a") shouldBe "a"
    //Explanation: The entire string s is the minimum window.
  }
  """Example 3: (s = "a", t = "aa") -> """"" in {
    Solution.minWindow(s = "a", t = "aa") shouldBe ""
    //Explanation: Both 'a's from t must be included in the window.
    //Since the largest window of s only has one 'a', return empty string.
  }
  """(s = "BANCADOBECODE", t = "ABC") -> "BANC"""" in {
    Solution.minWindow("BANCADOBECODE", "ABC") shouldBe "BANC"
  }
  """(s = "BANCADOBCAECODE", t = "ABC") -> "BCA"""" in {
    Solution.minWindow("BANCADOBCAECODE", "ABC") shouldBe "BCA"
  }

}
