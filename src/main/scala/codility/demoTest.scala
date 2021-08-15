package codility

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://app.codility.com/public-link/AgileEngine-Scala/]] */
class demoTest extends AnyWordSpec with Matchers {
  /**
   * Task 1
   *
   * Task description
   * This is a demo task.
   *
   * Write a function:
   *
   * object Solution { def solution(a: Array[Int]): Int }
   *
   * that, given an array A of N integers, returns the smallest positive integer (greater than 0) that does not occur in A.
   *
   * For example, given A = [1, 3, 6, 4, 1, 2], the function should return 5.
   *
   * Given A = [1, 2, 3], the function should return 4.
   *
   * Given A = [−1, −3], the function should return 1.
   *
   * Write an efficient algorithm for the following assumptions:
   *
   * N is an integer within the range [1..100,000];
   * each element of array A is an integer within the range [−1,000,000..1,000,000].
   */
  object Solution {
    def solution(a: Array[Int]): Int = {
      // write your code in Scala 2.12
      val set = a.toSet
      (1 to Int.MaxValue).find(!set.contains(_)).get
    }
  }

  "([-3, -1]) -> 1" in (Solution.solution(Array(-3, -1)) shouldBe 1)
  "([1, 5, 2, 4, 6]) -> 3" in (Solution.solution(Array(1, 5, 2, 4, 6)) shouldBe 3)
  "([0..100_000]) -> 100_000" in (Solution.solution((0 to 99_999).toArray) shouldBe 100_000)
}
