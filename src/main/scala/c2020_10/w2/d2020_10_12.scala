package c2020_10.w2

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


//noinspection DuplicatedCode
class d2020_10_12 extends AnyWordSpec with Matchers {

  /**
   * Buddy Strings
   *
   * Given two strings `A` and `B` of lowercase letters,
   * return `true` <em>if you can swap two letters in </em>`A`<em> so the result is equal to </em>`B`<em>,
   * otherwise, return </em>`false`.
   *
   * Swapping letters is defined as taking two indices `i` and `j` (0-indexed)
   * such that `i != j` and swapping the characters at `A[i]` and `A[j]`.
   * For example, swapping at indices `0` and `2` in `"abcd"` results in `"cbad"`.
   *
   *
   * <b>Constraints:</b><ul>
   * <li> `0 <= A.length <= 20000`
   * <li> `0 <= B.length <= 20000`
   * <li> `A` and `B` consist of lowercase letters.
   * </ul>
   */
  object Solution {
    def buddyStrings(A: String, B: String): Boolean =
      if (A.length < 2 || A.length != B.length) false
      else if (A == B) {
        var i = 0
        val charCounts = collection.mutable.Map[Char, Int]().withDefaultValue(0)

        while (i < A.length) {
          charCounts(A(i)) += 1

          if (charCounts(A(i)) > 1) i = A.length
          else i += 1
        }

        charCounts.exists(_._2 > 1)
      } else {
        var i = 0
        var diff = 0
        var i1 = -1
        var i2 = -1

        while (i < A.length) {
          if (A(i) != B(i)) {
            if (i1 == -1) i1 = i
            else if (i2 == -1) i2 = i
            diff += 1
          }

          if (diff > 2) i = A.length
          else i += 1
        }

        diff == 2 && A(i1) == B(i2) && A(i2) == B(i1)
      }
  }
  object Solution1 {
    def buddyStrings(A: String, B: String): Boolean =
      if (A.length < 2 || A.length != B.length) false
      else {
        var i = 0
        var diff = 0
        val charCountsA = collection.mutable.Map[Char, Int]().withDefaultValue(0)
        val charCountsB = collection.mutable.Map[Char, Int]().withDefaultValue(0)

        while (i < A.length) {
          charCountsA(A(i)) += 1
          charCountsB(B(i)) += 1
          if (A(i) != B(i))
            diff += 1

          if (diff > 2) i = A.length
          else i += 1
        }

        (diff == 2 && charCountsA == charCountsB) || (diff == 0 && charCountsA.exists(_._2 > 1))
      }
  }

  "Example 1" in {
    Solution.buddyStrings("ab", "ba") shouldBe true
    //Explanation: You can swap A[0] = 'a' and A[1] = 'b' to get "ba", which is equal to B.
  }
  "Example 2" in {
    Solution.buddyStrings("ab", "ab") shouldBe false
    //Explanation: The only letters you can swap are A[0] = 'a' and A[1] = 'b', which results in "ba" != B.
  }
  "Example 3" in {
    Solution.buddyStrings("aa", "aa") shouldBe true
    //Explanation: You can swap A[0] = 'a' and A[1] = 'a' to get "aa", which is equal to B.
  }
  "Example 4" in {
    Solution.buddyStrings("aaaaaaabc", "aaaaaaacb") shouldBe true
  }
  "Example 5" in {
    Solution.buddyStrings("", "aa") shouldBe false
  }

  "Test 27" in {
    Solution.buddyStrings("abcaa", "abcbb") shouldBe false
  }

  "My test: different chars" in {
    Solution.buddyStrings("ab", "aa") shouldBe false
  }
  "My test: max size" in {
    import util.Random

    val length = 20000
    val sb = new StringBuilder
    (1 to length).foreach(_ => sb.addOne((Random.nextInt('z' - 'a' + 1) + 'a').toChar))
    val str = sb.toString()

    Solution.buddyStrings(str, sb.drop(1).toString()) shouldBe false
    Solution.buddyStrings(str, str) shouldBe true
  }
}
