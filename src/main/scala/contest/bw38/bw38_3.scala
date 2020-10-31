package contest.bw38

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class bw38_3 extends AnyWordSpec with Matchers {

  /**
   * <h3>5541. Count Substrings That Differ by One Character</h3>
   *
   * Given two strings `s` and `t`, find the number of ways you can choose a non-empty substring of `s` and replace
   * a <b>single character</b> by a different character such that the resulting substring is a substring of `t`.
   * In other words, find the number of substrings in `s` that differ from some substring in `t`
   * by <b>exactly</b> one character.
   *
   * For example, `"compute"` and `"computa"` in `"computer"` and `"computation"`
   * only differ by the `'e'`/`'a'`, so this is a valid way.
   *
   * Return <em>the number of substrings that satisfy the condition above</em>.
   *
   * A <b>substring</b> is a contiguous sequence of characters within a string.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= s.length, t.length <= 100`
   * <li> `s` and `t` consist of lowercase English letters only.
   * </ul>
   */
  object Solution {
    def countSubstrings(s: String, t: String): Int = {
      var cnt = 0

      def updateSubstringsCount(ss: String): Unit = {
        for (i <- 0 to (t.length - ss.length)) {
          var j = 0
          var diff = 0
          while (j < ss.length && diff < 2) {
            if (ss(j) != t(i + j))
              diff += 1
            j += 1
          }
          if (diff == 1) cnt += 1
        }
      }

      val minL = s.length min t.length

      for (i <- s.indices) {
        for (l <- 1 to (minL min (s.length - i))) {
          updateSubstringsCount(s.substring(i, i + l))
        }
      }

      cnt
    }
  }

  import Solution.countSubstrings

  """Example 1: ("aba","baba") -> 6""" in {
    countSubstrings("aba", "baba") shouldBe 6
    // Explanation: The following are the pairs of substrings from s and t that differ by exactly 1 character:
    //   ("`a`ba", "`b`aba")
    //   ("`a`ba", "ba`b`a")
    //   ("ab`a`", "`b`aba")
    //   ("ab`a`", "ba`b`a")
    //   ("a`b`a", "b`a`ba")
    //   ("a`b`a", "bab`a`")
    //   The underlined portions are the substrings that are chosen from s and t.
  }
  """Example 2: ("ab","bb") -> 3""" in {
    countSubstrings("ab", "bb") shouldBe 3
    // Explanation: The following are the pairs of substrings from s and t that differ by 1 character:
    //   ("`a`b", "`b`b")
    //   ("`a`b", "b`b`")
    //   ("`ab`", "`bb`")
    //   The underlined portions are the substrings that are chosen from s and t.
  }
  """Example 3: ("a","a") -> 0""" in {
    countSubstrings("a", "a") shouldBe 0
  }
  """Example 4: ("abe","bbc") -> 10""" in {
    countSubstrings("abe", "bbc") shouldBe 10
  }

  "(max size) -> ???" in {
    import util.Random

    val sb = new StringBuilder
    (1 to 100).foreach(_ => sb.addOne((Random.nextInt('z' - 'a' + 1) + 'a').toChar))
    val tb = new StringBuilder
    (1 to 100).foreach(_ => tb.addOne((Random.nextInt('z' - 'a' + 1) + 'a').toChar))

    countSubstrings(sb.toString(), tb.toString()) should be > 1
  }
}
