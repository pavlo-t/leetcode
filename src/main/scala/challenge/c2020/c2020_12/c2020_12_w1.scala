package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/december-leetcoding-challenge/569/week-1-december-1st-december-7th/3550/]]
 */
//noinspection DuplicatedCode
class c2020_12_w1 extends AnyWordSpec with Matchers {

  /**
   * === Shortest Word Distance ===
   *
   * Given a list of words and two words ''word1'' and ''word2'',
   * return the shortest distance between these two words in the list.
   *
   * '''Note:''' You may assume that:
   *  - ''word1'' '''does not equal to''' ''word2''
   *  - ''word1'' and ''word2'' are both in the list
   */
  object Solution {
    def shortestDistance(words: Array[String], word1: String, word2: String): Int = {
      words.foldLeft((Int.MaxValue, Int.MinValue, Int.MinValue)) {
        case ((1, _, _), _)                            => return 1
        case ((rsf, _, c2), w) if w == word1 && c2 > 0 => (rsf min c2, 1, c2 + 1)
        case ((rsf, _, c2), w) if w == word1           => (rsf, 1, c2 + 1)
        case ((rsf, c1, _), w) if w == word2 && c1 > 0 => (rsf min c1, c1 + 1, 1)
        case ((rsf, c1, _), w) if w == word2           => (rsf, c1 + 1, 1)
        case ((rsf, c1, c2), _)                        => (rsf, c1 + 1, c2 + 1)
      }._1
    }
  }

  object Solution1 {
    def shortestDistance(words: Array[String], word1: String, word2: String): Int = {
      words.foldLeft((Int.MaxValue, Int.MinValue, Int.MinValue)) {
        case ((rsf, _, c2), w) if w == word1 => (if (c2 > 0) rsf min c2 else rsf, 1, c2 + 1)
        case ((rsf, c1, _), w) if w == word2 => (if (c1 > 0) rsf min c1 else rsf, c1 + 1, 1)
        case ((rsf, c1, c2), _)              => (rsf, c1 + 1, c2 + 1)
      }._1
    }
  }

  import Solution.shortestDistance

  """Example: words = ["practice", "makes", "perfect", "coding", "makes"]""" should {
    val words = Array("practice", "makes", "perfect", "coding", "makes")
    """(word1 = "coding", word2 = "practice") -> 3""" in {
      shortestDistance(words, "coding", "practice") shouldBe 3
    }
    """(word1 = "makes", word2 = "coding") -> 1""" in {
      shortestDistance(words, "makes", "coding") shouldBe 1
    }
  }
}
