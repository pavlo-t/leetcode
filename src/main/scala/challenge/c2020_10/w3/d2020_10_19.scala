package challenge.c2020_10.w3

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_10_19 extends AnyWordSpec with Matchers {

  /**
   * <h3>Minimum Domino Rotations For Equal Row</h3>
   *
   * In a row of dominoes, `A[i]` and `B[i]` represent the top and bottom halves of the `i`th domino.
   * (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)
   *
   * We may rotate the `i`th domino, so that `A[i]` and `B[i]` swap values.
   *
   * Return the minimum number of rotations so that all the values in `A` are the same,
   * or all the values in `B` are the same.
   *
   * If it cannot be done, return `-1`.
   *
   * <b>Constraints:</b><ul>
   * <li> `2 <= A.length == B.length <= 20_000`
   * <li> `1 <= A[i], B[i] <= 6`
   * </ul>
   */
  object Solution {
    import scala.annotation.tailrec

    def minDominoRotations(A: Array[Int], B: Array[Int]): Int = {
      val AS = A.toList
      val BS = B.toList
      (1 to 6).map(n => count(n, AS, BS)).filter(_ >= 0) match {
        case r if r.isEmpty => -1
        case r              => r.min
      }
    }

    @tailrec
    def count(n: Int, xss: Seq[Int], yss: Seq[Int], acc: (Int, Int) = (0, 0)): Int = (xss, yss) match {
      case (Nil, Nil)                           => acc._1 min acc._2
      case (x :: _, y :: _) if x != n && y != n => -1
      case (x :: xs, y :: ys) if x == y         => count(n, xs, ys, acc)
      case (x :: xs, _ :: ys) if x == n         => count(n, xs, ys, (acc._1, acc._2 + 1))
      case (_ :: xs, y :: ys) if y == n         => count(n, xs, ys, (acc._1 + 1, acc._2))
    }

    @tailrec
    def cnt2(n: Int, xss: Seq[Int], yss: Seq[Int], acc: Int = 0): Int = (xss, yss) match {
      case (Nil, Nil)                   => acc
      case (x :: xs, _ :: ys) if x == n => cnt2(n, xs, ys, acc)
      case (_ :: xs, y :: ys) if y == n => cnt2(n, xs, ys, acc + 1)
      case _                            => -1
    }

    def cnt1(n: Int, xss: Seq[Int], yss: Seq[Int]): Int = (xss, yss) match {
      case (Nil, Nil)         => 0
      case (x :: xs, y :: ys) =>
        if (x == n) cnt1(n, xs, ys)
        else if (y == n) 1 + cnt1(n, xs, ys)
        else -1
    }
  }

  object SolutionIdxRecursionMutable {
    import collection.mutable
    import scala.annotation.tailrec

    def minDominoRotations(A: Array[Int], B: Array[Int]): Int = {
      def rf(a: Boolean, old: Option[(Int, Int)]): Option[(Int, Int)] = old match {
        case None           => None
        case Some((oa, ob)) => Some(if (a) (oa, ob + 1) else (oa + 1, ob))
      }

      val rs = mutable.Map[Int, (Int, Int)]()
      (1 to 6).foreach(i => rs(i) = (0, 0))

      @tailrec
      def count(i: Int): Unit = {
        if (rs.nonEmpty && i < A.length) {
          val a = A(i)
          val b = B(i)
          rs.filterInPlace { case (k, _) => k == a || k == b }
          if (a != b) {
            rs.updateWith(a)(rf(true, _))
            rs.updateWith(b)(rf(false, _))
          }

          count(i + 1)
        }
      }
      count(0)

      if (rs.isEmpty) -1
      else rs.map { case (_, (a, b)) => a min b }.min
    }
  }

  object SolutionIdxRecursionImmutable {
    import scala.annotation.tailrec

    def minDominoRotations(A: Array[Int], B: Array[Int]): Int = {
      def rf(a: Boolean, old: Option[(Int, Int)]): Option[(Int, Int)] = old match {
        case None           => None
        case Some((oa, ob)) => Some(if (a) (oa, ob + 1) else (oa + 1, ob))
      }
      @tailrec
      def count(i: Int, rsf: Map[Int, (Int, Int)]): Int = {
        if (rsf.isEmpty) -1
        else if (i >= A.length) rsf.map { case (_, (a, b)) => a min b }.min
        else {
          val a = A(i)
          val b = B(i)
          val next = rsf.filter { case (k, _) => k == a || k == b }
          count(
            i + 1,
            if (a == b) next else next.updatedWith(a)(rf(true, _)).updatedWith(b)(rf(false, _))
          )
        }
      }
      count(0, (1 to 6).map(_ -> (0, 0)).toMap)
    }
  }

  object SolutionImperative {
    import collection.mutable

    def minDominoRotations(A: Array[Int], B: Array[Int]): Int = {
      val rs = mutable.Map[Int, Array[Int]]()
      for (n <- 1 to 6) rs(n) = Array.fill(2)(0)
      var i = 0

      while (i < A.length && rs.nonEmpty) {
        val a = A(i)
        val b = B(i)
        if (a != b) {
          if (rs.contains(a)) rs(a)(1) += 1
          if (rs.contains(b)) rs(b)(0) += 1
        }

        rs.filterInPlace { case (k, _) => k == a || k == b }
        i += 1
      }

      if (rs.isEmpty) -1
      else rs.flatMap(_._2.toSeq).min
    }
  }

  import Solution.minDominoRotations

  "Example 1: ([2,1,2,4,2,2], [5,2,6,2,3,2]) -> 2" in {
    val A = Array(2, 1, 2, 4, 2, 2)
    val B = Array(5, 2, 6, 2, 3, 2)

    minDominoRotations(A, B) shouldBe 2
    // Explanation:
    //   If we rotate the second and fourth dominoes, we can make every value in the top row equal to 2
  }
  "Example 2: ([3,5,1,2,3], [3,6,3,3,4]) -> -1" in {
    val A = Array(3, 5, 1, 2, 3)
    val B = Array(3, 6, 3, 3, 4)

    minDominoRotations(A, B) shouldBe -1
    // Explanation:
    //   In this case, it is not possible to rotate the dominoes to make one row of values equal.
  }

  "([1,2,1,2,1], [2,1,2,1,2]) -> 2" in {
    val A = Array(1, 2, 1, 2, 1)
    val B = Array(2, 1, 2, 1, 2)

    minDominoRotations(A, B) shouldBe 2
  }
  "(20_000 elements, each has 1) -> 10_000" in {
    val length = 20_000
    val A = Array.ofDim[Int](length)
    val B = Array.ofDim[Int](length)
    for (i <- A.indices) {
      val other = util.Random.nextInt(5) + 2
      if (i % 2 == 0) {
        A(i) = 1
        B(i) = other
      } else {
        A(i) = other
        B(i) = 1
      }
    }

    minDominoRotations(A, B) shouldBe 10_000
  }
  "(20_000 elements, each has 1 and 2) -> 10_000" in {
    val length = 20_000
    val A = Array.ofDim[Int](length)
    val B = Array.ofDim[Int](length)
    for (i <- A.indices) {
      if (i % 2 == 0) {
        A(i) = 1
        B(i) = 2
      } else {
        A(i) = 2
        B(i) = 1
      }
    }

    minDominoRotations(A, B) shouldBe 10_000
  }
  "(20_000 elements, all different) -> -1" in {
    val length = 20_000
    val A = Array.ofDim[Int](length)
    val B = Array.ofDim[Int](length)
    for (i <- A.indices) {
      val v = i % 6 + 1
      A(i) = v
      B(i) = v
    }

    minDominoRotations(A, B) shouldBe -1
  }
}
