package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/december-leetcoding-challenge/573/week-5-december-29th-december-31st/3584/]]
 */
//noinspection DuplicatedCode
class c2020_12_w5 extends AnyWordSpec with Matchers {
  /**
   * === Longest Substring with At Most K Distinct Characters ===
   *
   * Given a string `s` and an integer `k`, return
   * ''the length of the longest substring of ''`s`'' that contains at most ''`k`'' '''distinct''' characters''.
   *
   * '''Constraints:'''
   *  - `1 <= s.length <= 50_000`
   *  - `0 <= k <= 50`
   */
  object Solution {
    def lengthOfLongestSubstringKDistinct(s: String, k: Int): Int = {
      if (k == 0) 0 else {
        @scala.annotation.tailrec
        def rec(l: Int, r: Int, chars: Map[Char, Int], rsf: Int): Int = {
          if (l == s.length || r == s.length) rsf
          else if (chars.size <= k)
            chars.updated(s(r), chars.getOrElse(s(r), 0) + 1) match {
              case chars if chars.size > k => rec(l, r + 1, chars, rsf)
              case chars                   => rec(l, r + 1, chars, rsf max (r - l + 1))
            }
          else
            rec(l + 1, r,
              if (chars(s(l)) == 1) chars.removed(s(l))
              else chars.updated(s(l), chars(s(l)) - 1),
              rsf)
        }

        rec(0, 0, Map(), 0)
      }
    }
  }

  object SolutionMutableLinkedHashMapCharLastIdx {
    import scala.collection.mutable

    def lengthOfLongestSubstringKDistinct(s: String, k: Int): Int = {
      if (k == 0) 0 else {
        val chars = mutable.LinkedHashMap[Char, Int]()

        @scala.annotation.tailrec
        def rec(l: Int, r: Int, rsf: Int): Int = {
          if (l == s.length || r == s.length) rsf
          else {
            if (chars.contains(s(r)))
              chars.remove(s(r))
            chars.update(s(r), r)

            if (chars.size > k) {
              val (c, i) = chars.head
              chars.remove(c)
              rec(i + 1, r + 1, rsf)
            } else {
              rec(l, r + 1, rsf max (r - l + 1))
            }
          }
        }

        rec(0, 0, 0)
      }
    }
  }
  object SolutionMapCharLastIdx {
    def lengthOfLongestSubstringKDistinct(s: String, k: Int): Int = {
      if (k == 0) 0 else {
        @scala.annotation.tailrec
        def rec(l: Int, r: Int, chars: Map[Char, Int], rsf: Int): Int = {
          if (l == s.length || r == s.length) rsf
          else chars.updated(s(r), r) match {
            case chars if chars.size <= k => rec(l, r + 1, chars, rsf max (r - l + 1))
            case chars                    =>
              val (dropChar, dropI) = chars.minBy(_._2)
              rec(dropI + 1, r + 1, chars.removed(dropChar), rsf)
          }
        }

        rec(0, 0, Map(), 0)
      }
    }
  }

  object SolutionSetOfChars {
    def lengthOfLongestSubstringKDistinct(s: String, k: Int): Int = {
      if (k == 0) 0 else {
        @scala.annotation.tailrec
        def rec(l: Int, r: Int, chars: Set[Char], rsf: Int): Int = {
          if (l == s.length || r == s.length) rsf
          else chars + s(r) match {
            case chars if chars.size <= k => rec(l, r + 1, chars, rsf max (r - l + 1))
            case chars                    =>
              val l = chars.map(s.lastIndexOf(_, r)).min + 1
              rec(l, r + 1, chars - s(l - 1), rsf)
          }
        }

        rec(0, 0, Set(), 0)
      }
    }
  }
  object SolutionBruteForce {
    def lengthOfLongestSubstringKDistinct(s: String, k: Int): Int = {
      if (k == 0) {
        0
      } else {
        def distinctChars(l: Int, r: Int): Int =
          (l to r).foldLeft(Set[Char]())((acc, i) => acc + s(i)).size

        @scala.annotation.tailrec
        def rec(l: Int, r: Int, rsf: Int): Int = {
          if (l == s.length || r == s.length) rsf
          else if (distinctChars(l, r) > k) rec(l + 1, r max (l + 1), rsf)
          else rec(l, r + 1, rsf max (r - l + 1))
        }

        rec(0, 0, 0)
      }
    }
  }

  import Solution.lengthOfLongestSubstringKDistinct

  """Example 1: (s = "eceba", k = 2) -> 3""" in {
    lengthOfLongestSubstringKDistinct("eceba", 2) shouldBe 3
    //Explanation: The substring is "ece" with length 3.
  }
  """Example 2: (s = "aa", k = 1) -> 2""" in {
    lengthOfLongestSubstringKDistinct("aa", 2) shouldBe 2
    //Explanation: The substring is "aa" with length 2.
  }

  """(s = "a", k = 2) -> 1""" in {
    lengthOfLongestSubstringKDistinct("a", 2) shouldBe 1
  }
  """(s = "a", k = 0) -> 0""" in {
    lengthOfLongestSubstringKDistinct("a", 0) shouldBe 0
  }
  """(s = "a" * 50000, k = 2) -> 50000""" in {
    lengthOfLongestSubstringKDistinct("a" * 50000, 2) shouldBe 50000
  }
  """(s = "abcde" * 10000, k = 2) -> 2""" in {
    lengthOfLongestSubstringKDistinct("abcde" * 10000, 2) shouldBe 2
  }
  """(s = "abcde" * 10000, k = 10) -> 50000""" in {
    lengthOfLongestSubstringKDistinct("abcde" * 10000, 10) shouldBe 50000
  }
  """(s = "A to Z ++ a to z" * 962 = 50024, k = 50) -> 50""" in {
    val s = (('A' to 'Z') ++ ('a' to 'z')).mkString("") * 962
    lengthOfLongestSubstringKDistinct(s, 50) shouldBe 50
  }
  """(s = A + "B to Z ++ a to y" * 999 + z = 49952, k = 50) -> 49950""" in {
    val s = "A" + ((('B' to 'Z') ++ ('a' to 'y')).mkString("") * 999) + "z"
    lengthOfLongestSubstringKDistinct(s, 50) shouldBe 49950
  }

}
