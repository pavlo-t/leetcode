package contest.w215

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/contest/weekly-contest-215/problems/determine-if-two-strings-are-close/]]
 */
//noinspection DuplicatedCode
class w215_2 extends AnyWordSpec with Matchers {
  /**
   * === 5603. Determine if Two Strings Are Close ===
   *
   */
  object Solution {
    def closeStrings(word1: String, word2: String): Boolean = {
      def getCounts(w: String): Map[Char, Int] =
        w.groupMapReduce(identity)(_ => 1)(_ + _)

      val cnt1 = getCounts(word1)
      val cnt2 = getCounts(word2)

      cnt1.keySet == cnt2.keySet && (cnt1.values.toSeq.sorted == cnt2.values.toSeq.sorted)
    }
  }

  import Solution.closeStrings

  """Example 1: (word1 = "abc", word2 = "bca") -> true""" in {
    closeStrings("abc", "bca") shouldBe true
    //Explanation: You can attain word2 from word1 in 2 operations.
    //Apply Operation 1: "abc" -> "acb"
    //Apply Operation 1: "acb" -> "bca"
  }
  """Example 2: (word1 = "a", word2 = "aa") -> false""" in {
    closeStrings("a", "aa") shouldBe false
    //Explanation: It is impossible to attain word2 from word1, or vice versa, in any number of operations.
  }
  """Example 3: (word1 = "cabbba", word2 = "abbccc") -> true""" in {
    closeStrings("cabbba", "abbccc") shouldBe true
    //Explanation: You can attain word2 from word1 in 3 operations.
    //Apply Operation 1: "cabbba" -> "caabbb"
    //Apply Operation 2: "caabbb" -> "baaccc"
    //Apply Operation 2: "baaccc" -> "abbccc"
  }
  """Example 4: (word1 = "cabbba", word2 = "aabbss") -> false""" in {
    closeStrings("cabbba", "aabbss") shouldBe false
    //Explanation: It is impossible to attain word2 from word1, or vice versa, in any amount of operations.
  }

  """Test: (word1 = "uau", word2 = "ssx") -> false""" in {
    closeStrings("uau", "ssx") shouldBe false
  }
}
