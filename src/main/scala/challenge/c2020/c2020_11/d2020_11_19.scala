package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/566/week-3-november-15th-november-21st/3536/]]
 */
class d2020_11_19 extends AnyWordSpec with Matchers {
  /**
   * === Decode String ===
   *
   * Given an encoded string, return its decoded string.
   *
   * The encoding rule is: `k[encoded_string]`, where the ''encoded_string'' inside the square brackets
   * is being repeated exactly ''k'' times.
   * Note that ''k'' is guaranteed to be a positive integer.
   *
   * You may assume that the input string is always valid;
   * No extra white spaces, square brackets are well-formed, etc.
   *
   * Furthermore, you may assume that the original data does not contain any digits
   * and that digits are only for those repeat numbers, ''k''.
   * For example, there won't be input like `3a` or `2[4]`.
   *
   * '''Constraints:'''
   *  - `1 <= s.length <= 30`
   *  - `s` consists of lowercase English letters, digits, and square brackets `'[]'`.
   *  - `s` is guaranteed to be a '''valid''' input.
   *  - All the integers in `s` are in the range `[1, 300]`.
   */
  object Solution {
    private val pattern = "(.*[a-z\\[]|)(\\d+)\\[([a-z]+)](.*)".r

    @scala.annotation.tailrec
    def decodeString(s: String): String = {
      pattern.findFirstMatchIn(s) match {
        case None    => s
        case Some(m) =>
          val List(pre, n, ss, suf) = m.subgroups
          decodeString(pre + (ss * n.toInt) + suf)
      }
    }
  }

  import Solution.decodeString

  """Example 1: (s = "3[a]2[bc]") -> "aaabcbc"""" in {
    decodeString("3[a]2[bc]") shouldBe "aaabcbc"
  }
  """Example 2: (s = "3[a2[c]]") -> "accaccacc"""" in {
    decodeString("3[a2[c]]") shouldBe "accaccacc"
  }
  """Example 3: (s = "2[abc]3[cd]ef") -> "abcabccdcdcdef"""" in {
    decodeString("2[abc]3[cd]ef") shouldBe "abcabccdcdcdef"
  }
  """Example 4: (s = "abc3[cd]xyz") -> "abccdcdcdxyz"""" in {
    decodeString("abc3[cd]xyz") shouldBe "abccdcdcdxyz"
  }

  """Test 21: (s = "100[leetcode]") -> leetcode * 100""" in {
    decodeString("100[leetcode]") shouldBe ("leetcode" * 100)
  }

  """(s = "2[a2[b2[c2[d2[e2[f2[g2[h2[i2[j]]]]]]]]]]") -> long string""" in {
    val expected =
      "abcdefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjefghijjijjhijjijjghijjijjhijjijjfghijjijjhijj" +
        "ijjghijjijjhijjijjdefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjefghijjijjhijjijjghijjijjhij" +
        "jijjfghijjijjhijjijjghijjijjhijjijjcdefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjefghijjijj" +
        "hijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjdefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijj" +
        "hijjijjefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjbcdefghijjijjhijjijjghijjijjhijjijjfghij" +
        "jijjhijjijjghijjijjhijjijjefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjdefghijjijjhijjijjghi" +
        "jjijjhijjijjfghijjijjhijjijjghijjijjhijjijjefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjcdef" +
        "ghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjg" +
        "hijjijjhijjijjdefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjefghijjijjhijjijjghijjijjhijjijj" +
        "fghijjijjhijjijjghijjijjhijjijjabcdefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjefghijjijjhi" +
        "jjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjdefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhi" +
        "jjijjefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjcdefghijjijjhijjijjghijjijjhijjijjfghijjij" +
        "jhijjijjghijjijjhijjijjefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjdefghijjijjhijjijjghijji" +
        "jjhijjijjfghijjijjhijjijjghijjijjhijjijjefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjbcdefgh" +
        "ijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghi" +
        "jjijjhijjijjdefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjefghijjijjhijjijjghijjijjhijjijjfg" +
        "hijjijjhijjijjghijjijjhijjijjcdefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjefghijjijjhijjij" +
        "jghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijjdefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjij" +
        "jefghijjijjhijjijjghijjijjhijjijjfghijjijjhijjijjghijjijjhijjijj"

    decodeString("2[a2[b2[c2[d2[e2[f2[g2[h2[i2[j]]]]]]]]]]") shouldBe expected
  }
}
