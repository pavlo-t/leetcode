package contest.w216

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


/**
 * [[https://leetcode.com/contest/weekly-contest-216/problems/check-if-two-string-arrays-are-equivalent/]]
 */
//noinspection DuplicatedCode
class w216_1 extends AnyWordSpec with Matchers {

  /**
   * === 5605. Check If Two String Arrays are Equivalent ===
   *
   * Given two string arrays word1 and word2, return true if the two arrays represent the same string, and false otherwise.
   *
   * A string is represented by an array if the array elements concatenated in order forms the string.
   *
   * Constraints:
   *  - 1 <= word1.length, word2.length <= 1000
   *  - 1 <= word1[i].length, word2[i].length <= 1000
   *  - 1 <= sum(word1[i].length), sum(word2[i].length) <= 1000
   *  - word1[i] and word2[i] consist of lowercase letters.
   */
  object Solution {
    def arrayStringsAreEqual(word1: Array[String], word2: Array[String]): Boolean =
      word1.mkString("") == word2.mkString("")
  }

  //Example 1:
  //
  //Input: word1 = ["ab", "c"], word2 = ["a", "bc"]
  //Output: true
  //Explanation:
  //word1 represents string "ab" + "c" -> "abc"
  //word2 represents string "a" + "bc" -> "abc"
  //The strings are the same, so return true.
  //Example 2:
  //
  //Input: word1 = ["a", "cb"], word2 = ["ab", "c"]
  //Output: false
  //Example 3:
  //
  //Input: word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
  //Output: true
}
