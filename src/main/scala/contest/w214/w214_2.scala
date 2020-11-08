package contest.w214

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class w214_2 extends AnyWordSpec with Matchers {

  /**
   * <h3>5562. Minimum Deletions to Make Character Frequencies Unique</h3>
   *
   * A string `s` is called <b>good</b> if there are no two different characters in `s`
   * that have the same <b>frequency</b>.
   *
   * Given a string `s`, return
   * <em>the <b>minimum</b> number of characters you need to delete to make `s` <b>good</b></em>.
   *
   * The <b>frequency</b> of a character in a string is the number of times it appears in the string.
   * For example, in the string `"aab"`, the <b>frequency</b> of `'a'` is `2`,
   * while the <b>frequency</b> of `'b'` is `1`.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= s.length <= 100_000`
   * <li> `s` contains only lowercase English letters.
   * </ul>
   */
  object Solution {
    def minDeletions(s: String): Int = {
      val fs = Array.fill(26)(0)
      for (c <- s) fs(c - 'a') += 1
      def notGood(): Boolean = fs.filter(_ > 0).groupBy(identity).exists(_._2.length > 1)

      var deletions = 0

      while (notGood()) {
        val f = fs.filter(_ > 0).groupBy(identity).maxBy(_._2.length)._1
        val i = fs.indexWhere(_ == f)
        fs(i) -= 1
        deletions += 1
      }

      deletions
    }
  }

  import Solution.minDeletions

  """Example 1: (s = "aab") -> 0""" in {
    minDeletions("aab") shouldBe 0
    //Explanation: s is already good.
  }
  """Example 2: (s = "aaabbbcc") -> 2""" in {
    minDeletions("aaabbbcc") shouldBe 2
    // Explanation: You can delete two 'b's resulting in the good string "aaabcc".
    // Another way is to delete one 'b' and one 'c' resulting in the good string "aaabbc".
  }
  """Example 3: (s = "ceabaacb") -> 2""" in {
    minDeletions("ceabaacb") shouldBe 2
    // Explanation: You can delete both 'c's resulting in the good string "eabaab".
    // Note that we only care about characters that are still in the string at the end (i.e. frequency of 0 is ignored).
  }
  "(s = random of length 100_000) -> ???" in {
    val sb = new StringBuilder()
    for (_ <- 1 to 100_000) sb.addOne((util.Random.nextInt(26) + 'a').toChar)

    minDeletions(sb.toString()) should be >= 0
  }
  "(s = [a..z] of length 100_000) -> 303" in {
    val sb = new StringBuilder()
    for (i <- 1 to 100_000) sb.addOne(('a' + (i % 26)).toChar)

    minDeletions(sb.toString()) shouldBe 303
  }
}
