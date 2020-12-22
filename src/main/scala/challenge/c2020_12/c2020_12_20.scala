package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/571/week-3-december-15th-december-21st/3572/]]
 */
//noinspection DuplicatedCode
class c2020_12_20 extends AnyWordSpec with Matchers {

  /**
   * === Decoded String at Index ===
   *
   * An encoded string `S` is given.
   * To find and write the ''decoded'' string to a tape,
   * the encoded string is read '''one character at a time''' and the following steps are taken:
   *  - If the character read is a letter, that letter is written onto the tape.
   *  - If the character read is a digit (say `d`),
   *    the entire current tape is repeatedly written `d-1` more times in total.
   *
   * Now for some encoded string `S`, and an index `K`,
   * find and return the `K`-th letter (1 indexed) in the decoded string.
   *
   * '''Constraints:'''
   *  - `2 <= S.length <= 100`
   *  - `S` will only contain lowercase letters and digits `2` through `9`.
   *  - `S` starts with a letter.
   *  - `1 <= K <= 1_000_000_000`
   *  - It's guaranteed that `K` is less than or equal to the length of the decoded string.
   *  - The decoded string is guaranteed to have less than `2^63` letters.
   */
  object Solution {
    def decodeAtIndex(S: String, K: Int): String = {
      @scala.annotation.tailrec
      def recBack(i: Int, l: Long, k: Int): Int = {
        val nk = (k % l).toInt
        if (S(i).isDigit)
          recBack(i - 1, l / (S(i) - '0'), nk)
        else if (nk == 0)
          i
        else
          recBack(i - 1, l - 1, nk)
      }

      @scala.annotation.tailrec
      def rec(i: Int, l: Long, k: Int): String =
        if (S(i).isDigit) {
          val nl = l * (S(i) - '0')
          if (nl < k)
            rec(i + 1, nl, k)
          else
            S(recBack(i - 1, l, k)).toString
        } else if (k == l + 1)
          S(i).toString
        else
          rec(i + 1, l + 1, k)

      rec(0, 0, K)
    }
  }

  object SolutionIterative {
    // https://leetcode.com/problems/decoded-string-at-index/solution/
    def decodeAtIndex(S: String, K: Int): String = {
      var size = 0L

      for (c <- S)
        if (c.isDigit) size *= c - '0'
        else size += 1

      var k = K

      for (c <- S.reverseIterator) {
        k = (k % size).toInt

        if (c.isDigit)
          size /= c - '0'
        else if (k == 0)
          return c.toString
        else
          size -= 1
      }

      throw new IllegalArgumentException()
    }
  }

  object SolutionMy {
    def decodeAtIndex(S: String, K: Int): String = {
      @scala.annotation.tailrec
      def rec(i: Int, l: Long, k: Int): String =
        S(i) match {
          case c if c.isDigit  =>
            val nl = l * (c - '0')
            if (nl < k) rec(i + 1, nl, k)
            else {
              val nk = k % l
              if (nk == 0) S(S.lastIndexWhere(!_.isDigit, i)).toString
              else rec(0, 0, nk.toInt)
            }
          case c if k == l + 1 => c.toString
          case _               => rec(i + 1, l + 1, k)
        }

      rec(0, 0, K)
    }
  }

  object SolutionBruteForce2 {
    def decodeAtIndex(S: String, K: Int): String = {
      @scala.annotation.tailrec
      def rec(i: Int, rsf: String): String = {
        if (rsf.length >= K) rsf(K - 1).toString
        else S(i) match {
          case c if c.isDigit =>
            val d = c - '0'
            if (rsf.length.toLong * d >= K) rsf((K - 1) % rsf.length).toString
            else rec(i + 1, rsf * d)
          case c              => rec(i + 1, rsf + c)
        }
      }

      rec(0, "")
    }
  }
  object SolutionBruteForce {
    def decodeAtIndex(S: String, K: Int): String = {
      @scala.annotation.tailrec
      def rec(i: Int, rsf: String): String = {
        if (rsf.length >= K) rsf(K - 1).toString
        else S(i) match {
          case d if d.isDigit => rec(i + 1, rsf * (d - '0'))
          case c              => rec(i + 1, rsf + c)
        }
      }

      rec(0, "")
    }
  }

  import Solution.decodeAtIndex

  """Example 1: (S = "leet2code3", K = 10) -> "o"""" in {
    decodeAtIndex("leet2code3", 10) shouldBe "o"
    //Explanation:
    //The decoded string is "leetleetcodeleetleetcodeleetleetcode".
    //The 10th letter in the string is "o".
  }
  """Example 2: (S = "ha22", K = 5) -> "h"""" in {
    decodeAtIndex("ha22", 5) shouldBe "h"
    //Explanation:
    //The decoded string is "hahahaha". The 5th letter is "h".
  }
  """Example 3: (S = "a2345678999999999999999", K = 1) -> "a"""" in {
    decodeAtIndex("a2345678999999999999999", 1) shouldBe "a"
    //Explanation:
    //The decoded string is "a" repeated 8301530446056247680 times. The 1st letter is "a".
  }

  """test 8: (S="a23",K=6) -> "a"""" in (decodeAtIndex("a23", 6) shouldBe "a")
  """test 34: (S="y959q969u3hb22odq595",K=222280369) -> "y"""" in {
    decodeAtIndex("y959q969u3hb22odq595", 222280369) shouldBe "y"
  }

  """(S = "abc3", K = 1) -> "a"""" in (decodeAtIndex("abc3", 1) shouldBe "a")
  """(S = "abc3", K = 2) -> "b"""" in (decodeAtIndex("abc3", 2) shouldBe "b")
  """(S = "abc3", K = 3) -> "c"""" in (decodeAtIndex("abc3", 3) shouldBe "c")
  """(S = "abc3", K = 4) -> "a"""" in (decodeAtIndex("abc3", 4) shouldBe "a")
  """(S = "abc3", K = 5) -> "b"""" in (decodeAtIndex("abc3", 5) shouldBe "b")
  """(S = "abc3", K = 6) -> "c"""" in (decodeAtIndex("abc3", 6) shouldBe "c")
  """(S = "abc3", K = 7) -> "a"""" in (decodeAtIndex("abc3", 7) shouldBe "a")
  """(S = "abc3", K = 8) -> "b"""" in (decodeAtIndex("abc3", 8) shouldBe "b")
  """(S = "abc3", K = 9) -> "c"""" in (decodeAtIndex("abc3", 9) shouldBe "c")

  """(S = "a2345678999999999999999", K = 100_000_000) -> "a"""" in {
    decodeAtIndex("a2345678999999999999999", 100_000_000) shouldBe "a"
  }
  """(S = "a2345678999999999999999", K = 1_000_000_000) -> "a"""" in {
    decodeAtIndex("a2345678999999999999999", 1_000_000_000) shouldBe "a"
  }
}
