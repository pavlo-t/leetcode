package contest.w213

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class w213_2 extends AnyWordSpec with Matchers {

  /**
   * <h3>5555. Count Sorted Vowel Strings</h3>
   *
   * Given an integer `n`, <em>return the number of strings of length `n` that consist only of vowels `(a, e, i, o, u)`
   * and are <b>lexicographically sorted</b></em>.
   *
   * A string `s` is <b>lexicographically sorted</b> if for all valid `i`,
   * `s[i]` is the same as or comes before `s[i+1]` in the alphabet.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= n <= 50`
   * </ul>
   */
  object Solution {
    def countVowelStrings(n: Int): Int =
      (n + 4) * (n + 3) * (n + 2) * (n + 1) / (4 * 3 * 2 * 1)
  }
  object Solution2 {
    def countVowelStrings(n: Int): Int = {
      var ans = 1
      for (i <- 0 until 4) {
        ans *= n + 4 - i
        ans /= i + 1
      }
      ans
    }
  }
  object SolutionMy {
    def countVowelStrings(n: Int): Int = {
      if (n == 1) 5
      else {
        val chars = "aeiou"

        @scala.annotation.tailrec
        def loop(todo: Seq[String], rsf: Int): Int = {
          todo match {
            case Nil     => rsf
            case s :: ss =>
              if (s.length == n) loop(ss, rsf + 1)
              else {
                val nexts = chars.toSeq.filter(_ <= s.last).map(s.appended(_)).toList
                loop(nexts ++ ss, rsf)
              }
          }
        }

        loop(chars.map(_.toString).toList, 0)
      }
    }
  }

  import Solution.countVowelStrings

  "Example 1: (1) -> 5" in {
    countVowelStrings(1) shouldBe 5
    // Explanation: The 5 sorted strings that consist of vowels only are ["a","e","i","o","u"].
  }
  "Example 2: (2) -> 15" in {
    countVowelStrings(2) shouldBe 15
    // Explanation: The 15 sorted strings that consist of vowels only are
    //   ["aa","ae","ai","ao","au",
    //         "ee","ei","eo","eu",
    //              "ii","io","iu",
    //                   "oo","ou",
    //                        "uu"].
    //   Note that "ea" is not a valid string since 'e' comes after 'a' in the alphabet.
  }
  "Example 3: (33) -> 66045" in {
    countVowelStrings(33) shouldBe 66045
  }

  "(3) -> 35" in {
    countVowelStrings(3) shouldBe 35
    // ["aaa","aae","aai","aao","aau",
    //        "aee","aei","aeo","aeu",
    //              "aii","aio","aiu",
    //                    "aoo","aou",
    //                          "auu",
    //        "eee","eei","eeo","eeu",
    //              "eii","eio","eiu",
    //                    "eoo","eou",
    //                          "euu",
    //              "iii","iio","iiu",
    //                    "ioo","iou",
    //                          "iuu",
    //                    "ooo","oou",
    //                          "ouu",
    //                          "uuu"].
  }
  "(4) -> 70" in {
    countVowelStrings(4) shouldBe 70
  }
  "(43) -> 178365" in {
    countVowelStrings(43) shouldBe 178365
  }
  "(50) -> 316251" in {
    countVowelStrings(50) shouldBe 316251
  }

  "test with 6 vowels" in {
    //countVowelStrings(1) shouldBe 6
    //countVowelStrings(2) shouldBe 21
    //countVowelStrings(3) shouldBe 56
    //countVowelStrings(4) shouldBe 126
    //countVowelStrings(33) shouldBe 501942
    //countVowelStrings(43) shouldBe 1712304
    //countVowelStrings(50) shouldBe 3478761
  }
}
