package contest.bw38

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class bw38_4 extends AnyWordSpec with Matchers {

  /**
   * <h3>5542. Number of Ways to Form a Target String Given a Dictionary</h3>
   *
   * You are given a list of strings of the <b>same length</b> `words` and a string `target`.
   *
   * Your task is to form `target` using the given `words` under the following rules:<ul>
   * <li> `target` should be formed from left to right.
   * <li> To form the `i`th character (<b>0-indexed</b>) of `target`,
   * you can choose the `k`th character of the `j`th string in `words` if `target[i] = words[j][k]`.
   * <li> Once you use the `k`th character of the `j`th string of `words`,
   * you <b>can no longer</b> use the `x`th character of any string in `words` where `x <= k`.
   * In other words, all characters to the left of or at index `k` become unusable for every string.
   * <li> Repeat the process until you form the string `target`.
   * </ul>
   *
   * <b>Notice</b> that you can use <b>multiple characters</b> from the <b>same string</b> in `words`
   * provided the conditions above are met.
   *
   * Return <em>the number of ways to form `target` from `words`</em>.
   * Since the answer may be too large, return it <b>modulo</b> `10**9 + 7`.<br>
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= words.length <= 1000`
   * <li> `1 <= words[i].length <= 1000`
   * <li> All strings in `words` have the same length.
   * <li> `1 <= target.length <= 1000`
   * <li> `words[i]` and `target` contain only lowercase English letters.
   * </ul>
   */
  object Solution {
    def numWays(words: Array[String], target: String): Int = {
      val Mod = 1_000_000_007

      val wl = words(0).length
      val tl = target.length
      val cache = Array.fill(tl)(Array.fill(wl)(-1))

      def loop(i: Int, l: Int): Int = {
        if (i >= tl) 1
        else if (l >= wl || wl - l < tl - i) 0
        else if (cache(i)(l) != -1) cache(i)(l)
        else {
          var ans = 0

          for (w <- words) {
            if (w(l) == target(i)) {
              ans += loop(i + 1, l + 1)
              ans %= Mod
            }
          }
          ans += loop(i, l + 1)
          ans %= Mod
          cache(i)(l) = ans

          ans
        }
      }

      loop(0, 0)
    }
  }
  object Solution1 {
    def numWays(words: Array[String], target: String): Int = {
      val cnt = Array.fill(1005)(Array.fill(26)(0))

      for (w <- words; j <- w.indices)
        cnt(j)(w(j) - 'a') += 1

      val mod = 1_000_000_007
      val dp = Array.fill(1005)(Array.fill(1005)(0))

      dp(0)(0) = 1
      for (i <- 0 to target.length) {
        for (j <- 0 to words(0).length) {
          if (i != 0 || j != 0) {
            if (i == 0)
              dp(0)(j) += dp(0)(j - 1)
            else if (j != 0) {
              dp(i)(j) += dp(i)(j - 1) + dp(i - 1)(j - 1) * cnt(j - 1)(target(i - 1) - 'a')
              dp(i)(j) %= mod
            }
          }
        }
      }

      dp(target.length)(words(0).length)
    }
  }

  import Solution.numWays

  """Example 1: (words = ["acca","bbbb","caca"], target = "aba") -> 6""" in {
    numWays(Array("acca", "bbbb", "caca"), "aba") shouldBe 6
    // Explanation: There are 6 ways to form target.
    //   "aba" -> index 0 ("acca"), index 1 ("bbbb"), index 3 ("caca")
    //   "aba" -> index 0 ("acca"), index 2 ("bbbb"), index 3 ("caca")
    //   "aba" -> index 0 ("acca"), index 1 ("bbbb"), index 3 ("acca")
    //   "aba" -> index 0 ("acca"), index 2 ("bbbb"), index 3 ("acca")
    //   "aba" -> index 1 ("caca"), index 2 ("bbbb"), index 3 ("acca")
    //   "aba" -> index 1 ("caca"), index 2 ("bbbb"), index 3 ("caca")
  }
  """Example 2: (words = ["abba","baab"], target = "bab") -> 4""" in {
    numWays(Array("abba", "baab"), "bab") shouldBe 4
    // Explanation: There are 4 ways to form target.
    //   "bab" -> index 0 ("baab"), index 1 ("baab"), index 2 ("abba")
    //   "bab" -> index 0 ("baab"), index 1 ("baab"), index 3 ("baab")
    //   "bab" -> index 0 ("baab"), index 2 ("baab"), index 3 ("baab")
    //   "bab" -> index 1 ("abba"), index 2 ("baab"), index 3 ("baab")
  }
  """Example 3: (words = ["abcd"], target = "abcd") -> 1""" in {
    numWays(Array("abcd"), "abcd") shouldBe 1
  }
  """Example 4: (words = ["abab","baba","abba","baab"], target = "abba") -> 16""" in {
    numWays(Array("abab", "baba", "abba", "baab"), "abba") shouldBe 16
  }

  "(max size) -> ???" in {
    import util.Random

    val words = Array.fill(1000) {
      val sb = new StringBuilder
      (1 to 100).foreach(_ => sb.addOne((Random.nextInt('z' - 'a' + 1) + 'a').toChar))
      sb.toString
    }
    val target = {
      val sb = new StringBuilder
      (1 to 100).foreach(_ => sb.addOne((Random.nextInt('z' - 'a' + 1) + 'a').toChar))
      sb.toString
    }

    val result = numWays(words, target)
    val result1 = Solution1.numWays(words, target)
    Thread.sleep(20)
    println(s"$result, $result1")
    result should be > 1
    result1 should be > 1
  }
}
