package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/571/week-3-december-15th-december-21st/3569/]]
 */
//noinspection DuplicatedCode
class c2020_12_17 extends AnyWordSpec with Matchers {

  /**
   * === 4Sum II ===
   *
   * Given four lists `A`, `B`, `C`, `D` of integer values,
   * compute how many tuples `(i, j, k, l)` there are such that `A[i] + B[j] + C[k] + D[l]` is zero.
   *
   * To make problem a bit easier, all `A`, `B`, `C`, `D` have same length of `N` where `0 ≤ N ≤ 500`.
   * All integers are in the range of `-2^28` to `2^28 - 1` and the result is guaranteed to be at most `2^31 - 1`.
   */
  object Solution {
    import collection.mutable

    def fourSumCount(A: Array[Int], B: Array[Int], C: Array[Int], D: Array[Int]): Int = {
      var result = 0

      val AB = mutable.Map[Int, Int]().withDefaultValue(0)

      for (i <- A.indices; j <- A.indices)
        AB(A(i) + B(j)) += 1

      for (k <- A.indices; l <- A.indices)
        result += AB(-(C(k) + D(l)))

      result
    }
  }

  object SolutionMy2Maps {
    import collection.mutable

    def fourSumCount(A: Array[Int], B: Array[Int], C: Array[Int], D: Array[Int]): Int = {
      var result = 0

      val AB = mutable.Map[Int, Int]().withDefaultValue(0)
      val CD = mutable.Map[Int, Int]().withDefaultValue(0)

      for (i <- A.indices; j <- A.indices) {
        AB(A(i) + B(j)) += 1
        CD(C(i) + D(j)) += 1
      }

      for ((ab, abCnt) <- AB)
        result += abCnt * CD(-ab)

      result
    }
  }

  import Solution.fourSumCount

  "Example: (A=[1,2],B=[-2,-1],C=[-1,2],D=[0,2]) -> 2" in {
    val A = Array(1, 2)
    val B = Array(-2, -1)
    val C = Array(-1, 2)
    val D = Array(0, 2)

    fourSumCount(A, B, C, D) shouldBe 2
    //Explanation:
    //The two tuples are:
    //1. (0, 0, 0, 1) -> A[0] + B[0] + C[0] + D[1] = 1 + (-2) + (-1) + 2 = 0
    //2. (1, 1, 0, 0) -> A[1] + B[1] + C[0] + D[0] = 2 + (-1) + (-1) + 0 = 0
  }

  "Test 6: ([-1,-1],[-1,1],[-1,1],[1,-1]) -> 6" in {
    val A = Array(-1, -1)
    val B = Array(-1, 1)
    val C = Array(-1, 1)
    val D = Array(1, -1)

    fourSumCount(A, B, C, D) shouldBe 6
  }

  "(A=[0],B=[0],C=[0],D=[0]) -> 1" in {
    val a = Array(0)
    fourSumCount(a, a, a, a) shouldBe 1
  }

  "(A=[1 to 500],B=[1 to 500],C=[1 to 500],D=[1 to 500]) -> 0" in {
    val a = (1 to 500).toArray
    fourSumCount(a, a, a, a) shouldBe 0
  }

}
