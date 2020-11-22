package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/566/week-3-november-15th-november-21st/3538/]]
 */
//noinspection DuplicatedCode
class d2020_11_21 extends AnyWordSpec with Matchers {

  /**
   * === Numbers At Most N Given Digit Set ===
   *
   * Given an array of `digits`, you can write numbers using each `digits[i]` as many times as we want.
   * For example, if `digits = ['1','3','5']`, we may write numbers such as `'13'`, `'551'`, and `'1351315'`.
   *
   * Return ''the number of positive integers that can be generated'' that are less than or equal to a given integer `n`.
   *
   * '''Constraints:'''
   *  - `1 <= digits.length <= 9`
   *  - `digits[i].length == 1`
   *  - `digits[i]` is a digit from `'1'` to `'9'`.
   *  - All the values in `digits` are '''unique'''.
   *  - `1 <= n <= 1_000_000_000`
   */
  object Solution {
    def atMostNGivenDigitSet(digits: Array[String], n: Int): Int = {
      def otherCount(s: String): Int =
        if (s.length == 1) digits.count(_.head <= s.head)
        else {
          val currentCount = math.pow(digits.length, s.length - 1).toInt
          val lowerCount = if (digits.exists(_.head == s.head)) otherCount(s.tail) else 0
          digits.count(_.head < s.head) * currentCount + lowerCount
        }

      val ns = n.toString

      val leading0count = (1 until ns.length).fold(0)((acc, p) => acc + math.pow(digits.length, p).toInt)
      leading0count + otherCount(ns)
    }
  }

  object SolutionMathRec {
    def atMostNGivenDigitSet(digits: Array[String], n: Int): Int = {
      val bb = digits.length // bijectiveBase
      val dChars = digits.map(_.head).sorted

      @scala.annotation.tailrec
      def getLargestValid(s: String, rsf: Seq[Int]): Seq[Int] = {
        if (s.isEmpty)
          rsf
        else {
          val (cIndex, matched) = dChars.lastIndexWhere(_ <= s.head) match {
            case -1 => (0, false)
            case v  => (v + 1, dChars(v) == s.head)
          }
          if (matched)
            getLargestValid(s.tail, rsf :+ cIndex)
          else
            (rsf :+ cIndex) ++ s.tail.map(_ => bb)
        }
      }

      val lv = getLargestValid(n.toString, Seq())

      lv.fold(0)((acc, x) => acc * bb + x)
    }
  }

  object SolutionMath {
    import scala.util.control.Breaks._

    def atMostNGivenDigitSet(D: Array[String], n: Int): Int = {
      D.sortInPlace()

      val B = D.length // bijective-base B
      val ca = n.toString
      val K = ca.length
      val A = Array.ofDim[Int](K)
      var t = 0

      breakable(for (c <- ca) {
        var cIndex = 0 // Largest such that c >= D(cIndex - 1)
        var matched = false
        for (i <- D.indices) {
          if (c >= D(i).head) {
            cIndex = i + 1
            if (c == D(i).head)
              matched = true
          }
        }

        A(t) = cIndex
        t += 1

        if (!matched) {
          if (cIndex == 0) { // subtract 1
            var j = t - 1
            while (j > 0) {
              if (A(j) > 0)
                j = 0
              else {
                A(j) += B
                A(j - 1) -= 1

                j -= 1
              }
            }
          }

          while (t < K) {
            A(t) = B
            t += 1
          }

          break()
        }
      })

      var ans = 0
      for (x <- A) ans = ans * B + x
      ans
    }
  }
  object SolutionDp {
    def atMostNGivenDigitSet(digits: Array[String], n: Int): Int = {
      val S = n.toString
      val K = S.length()
      val dp = Array.ofDim[Int](K + 1)
      dp(K) = 1

      for (i <- K - 1 to 0 by -1) {
        // compute dp(i)
        val Si = S(i) - '0'
        for (d <- digits) {
          if (d.toInt < Si)
            dp(i) += math.pow(digits.length, K - i - 1).toInt
          else if (d.toInt == Si)
            dp(i) += dp(i + 1)
        }
      }

      for (i <- 1 until K) {
        dp(0) += math.pow(digits.length, i).toInt
      }
      dp(0)
    }
  }

  import Solution.atMostNGivenDigitSet

  """Example 1: (digits = ["1","3","5","7"], n = 100) -> 20""" in {
    atMostNGivenDigitSet(Array("1", "3", "5", "7"), 100) shouldBe 20
    //The 20 numbers that can be written are:
    //1, 3, 5, 7, 11, 13, 15, 17, 31, 33, 35, 37, 51, 53, 55, 57, 71, 73, 75, 77.
  }
  """Example 2: (digits = ["1","4","9"], n = 1000000000) -> 29523""" in {
    atMostNGivenDigitSet(Array("1", "4", "9"), 1_000_000_000) shouldBe 29523
    //We can write 3 one digit numbers, 9 two digit numbers, 27 three digit numbers,
    //81 four digit numbers, 243 five digit numbers, 729 six digit numbers,
    //2187 seven digit numbers, 6561 eight digit numbers, and 19683 nine digit numbers.
    //In total, this is 29523 integers that can be written using the digits array.
  }
  """Example 3: (digits = ["7"], n = 8) -> 1""" in {
    atMostNGivenDigitSet(Array("7"), 8) shouldBe 1
  }

  """(digits = ["7"], n = 6) -> 0""" in {
    atMostNGivenDigitSet(Array("7"), 6) shouldBe 0
  }
  """(digits = ["7"], n = 7) -> 1""" in {
    atMostNGivenDigitSet(Array("7"), 7) shouldBe 1
  }
  """(digits = ["7"], n = 10) -> 1""" in {
    atMostNGivenDigitSet(Array("7"), 10) shouldBe 1
  }
  """(digits = ["7"], n = 76) -> 1""" in {
    atMostNGivenDigitSet(Array("7"), 76) shouldBe 1
  }
  """(digits = ["7"], n = 77) -> 2""" in {
    atMostNGivenDigitSet(Array("7"), 77) shouldBe 2
  }
  """(digits = ["7"], n = 78) -> 2""" in {
    atMostNGivenDigitSet(Array("7"), 78) shouldBe 2
  }
  """(digits = ["7"], n = 80) -> 2""" in {
    atMostNGivenDigitSet(Array("7"), 80) shouldBe 2
  }
  """(digits = ["7"], n = 100) -> 2""" in {
    atMostNGivenDigitSet(Array("7"), 100) shouldBe 2
  }

  """Test 37: (digits = ["5","6"], n = 19) -> 2""" in {
    atMostNGivenDigitSet(Array("5", "6"), 19) shouldBe 2
  }

  """Test ???: (digits = ["1","9","4"], n = 42) -> 7""" in {
    atMostNGivenDigitSet(Array("1", "9", "4"), 42) shouldBe 7
  }

  """(digits = ["1","3","5","7"], n = 77) -> 20""" in {
    atMostNGivenDigitSet(Array("1", "3", "5", "7"), 77) shouldBe 20
  }
  """(digits = ["1","3","5","7"], n = 15) -> 7""" in {
    atMostNGivenDigitSet(Array("1", "3", "5", "7"), 15) shouldBe 7
  }
  """(digits = ["1","3","5","7"], n = 20) -> 8""" in {
    atMostNGivenDigitSet(Array("1", "3", "5", "8"), 20) shouldBe 8
  }

  """(digits = ["1","4","9"], n = 999999999) -> 29523""" in {
    atMostNGivenDigitSet(Array("1", "4", "9"), 999999999) shouldBe 29523
  }
}
