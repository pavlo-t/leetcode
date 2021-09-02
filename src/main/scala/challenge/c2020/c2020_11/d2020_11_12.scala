package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/565/week-2-november-8th-november-14th/3528/]]
 */
//noinspection DuplicatedCode
class d2020_11_12 extends AnyWordSpec with Matchers {

  /**
   * <h3>Permutations II</h3>
   *
   * Given a collection of numbers, `nums`, that might contain duplicates,
   * return <em>all possible unique permutations <b>in any order</b></em>.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= nums.length <= 8`
   * <li> `-10 <= nums[i] <= 10`
   * </ul>
   */
  object Solution {
    def permuteUnique(nums: Array[Int]): List[List[Int]] = {
      def backtrack(comb: List[Int], counts: Map[Int, Int]): List[List[Int]] = {
        if (comb.size == nums.length) List(comb)
        else
          counts.map { case (num, cnt) =>
            backtrack(num :: comb, if (cnt == 1) counts.removed(num) else counts.updated(num, cnt - 1))
          }.reduce(_ ++ _)
      }

      backtrack(List(), nums.groupMapReduce(identity)(_ => 1)((a, b) => a + b))
    }
  }

  object SolutionMutable {
    import collection.mutable

    def permuteUnique(nums: Array[Int]): List[List[Int]] = {
      val counts = mutable.Map[Int, Int]().withDefaultValue(0)
      nums.foreach(n => counts(n) += 1)
      val results = mutable.ListBuffer[List[Int]]()
      val comb = mutable.ListBuffer[Int]()

      def backtrack(): Unit = {
        if (comb.size == nums.length) results += comb.toList
        else for ((num, cnt) <- counts if cnt > 0) {
          comb += num
          counts(num) -= 1

          backtrack()

          comb.dropRightInPlace(1)
          counts(num) += 1
        }
      }

      backtrack()

      results.toList
    }
  }

  object SolutionScalaBuiltins {
    def permuteUnique(nums: Array[Int]): List[List[Int]] =
      nums.foldLeft(Set(List[Int]())) { (acc, i) =>
        acc.flatMap { list => list.insertAt(list.size, i) +: list.indices.map(list.insertAt(_, i)) }
      }.toList

    implicit class ListInsertAt[T](l: List[T]) {
      def insertAt(index: Int, elem: T): List[T] = {
        val (car, cdr) = l.splitAt(index)
        car ++ (elem :: cdr)
      }
    }
  }

  import Solution.permuteUnique

  "Example 1: (nums = [1,1,2]) -> [[1,1,2],[1,2,1],[2,1,1]]" in {
    permuteUnique(Array(1, 1, 2)) should contain allElementsOf List(List(1, 1, 2), List(1, 2, 1), List(2, 1, 1))
  }
  "Example 2: (nums = [1,2,3]) -> [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]" in {
    val expected = List(List(1, 2, 3), List(1, 3, 2), List(2, 1, 3), List(2, 3, 1), List(3, 1, 2), List(3, 2, 1))
    permuteUnique(Array(1, 2, 3)) should contain allElementsOf expected
  }

  "(nums = [1]) -> [[1]]" in {
    val expected = List(List(1))
    permuteUnique(Array(1)) shouldBe expected
  }
  "(nums = [1,1,1,1,1,1,1,1]) -> [[1,1,1,1,1,1,1,1]]" in {
    val expected = List(List(1, 1, 1, 1, 1, 1, 1, 1))
    permuteUnique(Array(1, 1, 1, 1, 1, 1, 1, 1)) shouldBe expected
  }
  "(nums = [1,2,3,4,5,6,7,8]) -> List of size 8!" in {
    permuteUnique(Array(1, 2, 3, 4, 5, 6, 7, 8)).size shouldBe 40320
  }
}
