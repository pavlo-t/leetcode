package c2020_10.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d2020_10_02 extends AnyWordSpec with Matchers {

  /**
   * Given an array of <b>distinct</b> integers `candidates` and a target integer `target`,
   * return <em>a list of all <b>unique combinations</b> of `candidates` where the chosen numbers sum to `target`</em>.
   * You may return the combinations in <b>any order</b>.
   *
   * The <b>same</b> number may be chosen from `candidates` an <b>unlimited number of times</b>.
   * Two combinations are unique if the frequency of at least one of the chosen numbers is different.
   *
   * Constraints:
   * <li> 1 <= candidates.length <= 30
   * <li> 1 <= candidates[i] <= 200
   * <li> All elements of candidates are distinct.
   * <li> 1 <= target <= 500
   */
  object Solution {
    def combinationSum(candidates: Array[Int], target: Int): List[List[Int]] = {
      def loop(xss: List[Int], rsf: List[Int]): List[List[Int]] =
        xss match {
          case Nil     => Nil
          case x :: xs =>
            val sum = rsf.sum + x

            if (sum > target)
              loop(xs, rsf)
            else if (sum == target)
              List(x :: rsf) ++ loop(xs, rsf)
            else
              loop(xss, x :: rsf) ++ loop(xs, rsf)
        }

      loop(candidates.toList, List())
    }

    def combinationSum2(candidates: Array[Int], target: Int): List[List[Int]] = {
      import collection.mutable.ListBuffer
      val results = ListBuffer[List[Int]]()

      def loop(xss: List[Int], rsf: List[Int]): Unit =
        xss match {
          case Nil     =>
          case x :: xs =>
            val sum = rsf.sum + x

            if (sum > target)
              loop(xs, rsf)
            else if (sum == target) {
              results.addOne(x :: rsf)
              loop(xs, rsf)
            } else {
              loop(xss, x :: rsf)
              loop(xs, rsf)
            }
        }

      loop(candidates.toList, List())

      results.toList
    }

    def combinationSum3(candidates: Array[Int], target: Int): List[List[Int]] = {
      import collection.mutable.ListBuffer

      val results = ListBuffer[List[Int]]()

      def setResults(idx: Int, rsf: List[Int]): Unit = {
        if (idx < candidates.length) {
          val sum = rsf.sum + candidates(idx)

          if (sum > target)
            setResults(idx + 1, rsf)
          else if (sum == target) {
            results.addOne(candidates(idx) :: rsf)
            setResults(idx + 1, rsf)
          } else {
            setResults(idx, candidates(idx) :: rsf)
            setResults(idx + 1, rsf)
          }
        }
      }
      setResults(0, List())

      results.toList
    }
  }

  "CombinationSum" should {
    "Example 1" in {
      Solution.combinationSum(Array(2, 3, 6, 7), 7) shouldBe List(List(3, 2, 2), List(7))
      // Explanation:
      //   2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
      //   7 is a candidate, and 7 = 7.
      //   These are the only two combinations.
    }
    "Example 2" in {
      Solution.combinationSum(Array(2, 3, 5), 8) shouldBe List(List(2, 2, 2, 2), List(3, 3, 2), List(5, 3))
    }
    "Example 3" in {
      Solution.combinationSum(Array(2), 1) shouldBe List()
    }
    "Example 4" in {
      Solution.combinationSum(Array(1), 1) shouldBe List(List(1))
    }
    "Example 5" in {
      Solution.combinationSum(Array(1), 2) shouldBe List(List(1, 1))
    }

    "test 121" in {
      Solution.combinationSum(Array(4, 2, 8), 8) shouldBe List(List(4, 4), List(2, 2, 4), List(2, 2, 2, 2), List(8))
    }
  }
}
