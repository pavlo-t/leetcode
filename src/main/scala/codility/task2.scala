package codility

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class task2 extends AnyWordSpec with Matchers {
  /**
   *
   */

  object Solution {
    def solution(a: Array[Int]): Int = {
      val offered = a.toSet

      @inline def moveLeft(l: Int, visits: Map[Int, Int]): Map[Int, Int] = visits(a(l)) match {
        case 1 => visits - a(l)
        case c => visits.updated(a(l), c - 1)
      }

      @inline def moveRight(r: Int, visits: Map[Int, Int]): Map[Int, Int] =
        visits.updated(a(r + 1), visits.getOrElse(a(r + 1), 0) + 1)

      @scala.annotation.tailrec
      def solve(l: Int, r: Int, visits: Map[Int, Int], rsf: Int): Int =
        if (visits.size == offered.size)
          solve(l + 1, r, moveLeft(l, visits), rsf.min(r + 1 - l))
        else if (r < a.length - 1)
          solve(l, r + 1, moveRight(r, visits), rsf)
        else rsf

      solve(0, 0, Map(a(0) -> 1), a.length)
    }

    def solution_scala2_13(a: Array[Int]): Int = {
      val offered = a.toSet

      @inline def moveLeft(l: Int, visits: Map[Int, Int]): Map[Int, Int] =
        visits.updatedWith(a(l))(_.flatMap(i => if (i > 1) Some(i - 1) else None))
      @inline def moveRight(r: Int, visits: Map[Int, Int]): Map[Int, Int] =
        visits.updatedWith(a(r + 1))(_.map(_ + 1).orElse(Some(1)))

      @scala.annotation.tailrec
      def solve(l: Int, r: Int, visits: Map[Int, Int], rsf: Int): Int =
        if (visits.size == offered.size)
          solve(l + 1, r, moveLeft(l, visits), rsf.min(r + 1 - l))
        else if (r < a.length - 1)
          solve(l, r + 1, moveRight(r, visits), rsf)
        else rsf

      solve(0, 0, Map(a(0) -> 1), a.length)
    }
  }

  "test 1" in (Solution.solution(Array(7, 3, 7, 3, 1, 3, 4, 1)) shouldBe 5)
  "test 2" in (Solution.solution(Array(2, 1, 1, 3, 2, 1, 1, 3)) shouldBe 3)
  "test 3" in (Solution.solution(Array(7, 5, 2, 7, 2, 7, 4, 7)) shouldBe 6)

  "test 4" in (Solution.solution(Array(7)) shouldBe 1)
  "test 5" in (Solution.solution(Array(7, 7)) shouldBe 1)
  "test 6" in (Solution.solution(Array(6, 7)) shouldBe 2)

  "perf 1" in (Solution.solution((0 to 99_999).toArray) shouldBe 100_000)
  "perf 2" in (Solution.solution((0 to 99_999).map(_ => 1).toArray) shouldBe 1)
}
