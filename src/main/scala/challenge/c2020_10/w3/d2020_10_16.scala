package challenge.c2020_10.w3

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_10_16 extends AnyWordSpec with Matchers {

  /**
   * <h3>Search a 2D Matrix</h3>
   *
   * Write an efficient algorithm that searches for a value in an `m x n` matrix.
   * This matrix has the following properties:<ul>
   * <li> Integers in each row are sorted from left to right.
   * <li> The first integer of each row is greater than the last integer of the previous row.
   * </ul>
   *
   * <b>Constraints:</b><ul>
   * <li> `m == matrix.length`
   * <li> `n == matrix[i].length`
   * <li> `0 <= m, n <= 100`
   * <li> `-10000 <= matrix[i][j], target <= 10000`
   * </ul>
   */
  // O(log n * log m)
  object Solution {
    def searchMatrix(matrix: Array[Array[Int]], target: Int): Boolean = {
      if (matrix.length == 0) false
      else if (matrix.length == 1) binarySearch(matrix(0), (v: Int) => v - target).isDefined
      else {
        def rowComparator(r: Array[Int]): Int =
          if (r.head > target) 1
          else if (r.last < target) -1
          else 0

        binarySearch(matrix, rowComparator)
          .flatMap(binarySearch(_, (v: Int) => v - target))
          .isDefined
      }
    }

    private def binarySearch[T](arr: Array[T], c: T => Int): Option[T] = {
      var start = 0
      var end = arr.length - 1
      var i = start + (end - start) / 2

      while (start <= end) {
        val r = c(arr(i))
        if (r == 0) return Some(arr(i))
        else if (r > 0)
          end = i - 1
        else
          start = i + 1
        i = start + (end - start) / 2
      }
      None
    }
  }
  // O(log n * log m)
  object SolutionBinarySearchWhile {
    def searchMatrix(matrix: Array[Array[Int]], target: Int): Boolean = {
      if (matrix.length == 0) false
      else if (matrix.length == 1) searchRow(matrix(0), target)
      else {
        findRow(matrix, target) match {
          case None    => false
          case Some(r) => searchRow(r, target)
        }
      }
    }

    private def findRow(m: Array[Array[Int]], t: Int): Option[Array[Int]] = {
      var start = 0
      var end = m.length - 1
      var ri = start + (end - start) / 2

      while (start <= end) {
        val r = m(ri)
        if (r.length > 0 && r.head <= t && r.last >= t)
          return Some(r)
        else if (r(0) > t)
          end = ri - 1
        else
          start = ri + 1
        ri = start + (end - start) / 2
      }
      None
    }

    private def searchRow(r: Array[Int], t: Int): Boolean = {
      var start = 0
      var end = r.length - 1
      var mid = start + (end - start) / 2

      while (start <= end) {
        if (r(mid) == t) return true
        else if (r(mid) > t)
          end = mid - 1
        else
          start = mid + 1
        mid = start + (end - start) / 2
      }
      false
    }
  }
  // O(n*m)
  object SolutionWithScalaBuiltIns {
    def searchMatrix(matrix: Array[Array[Int]], target: Int): Boolean =
      matrix.exists(_.contains(target))
  }

  import Solution.searchMatrix

  private def R(vs: Int*): Array[Int] = Array(vs: _*)
  private def M(rows: Array[Int]*): Array[Array[Int]] = Array(rows: _*)

  //private def printMatrix(m: Array[Array[Int]]): Unit =
  //  println(m.map(_.mkString("[", ",", "]")).mkString("[", ",\n", "]"))

  "Example 1: ([[1,3,5,7],[10,11,16,20],[23,30,34,50]], 3) -> true" in {
    val matrix = M(R(1, 3, 5, 7), R(10, 11, 16, 20), R(23, 30, 34, 50))
    searchMatrix(matrix, 3) shouldBe true
  }
  "Example 1: ([[1,3,5,7],[10,11,16,20],[23,30,34,50]], 11) -> true" in {
    val matrix = M(R(1, 3, 5, 7), R(10, 11, 16, 20), R(23, 30, 34, 50))
    searchMatrix(matrix, 11) shouldBe true
  }
  "Example 1: ([[1,3,5,7],[10,11,16,20],[23,30,34,50]], 50) -> true" in {
    val matrix = M(R(1, 3, 5, 7), R(10, 11, 16, 20), R(23, 30, 34, 50))
    searchMatrix(matrix, 50) shouldBe true
  }
  "Example 2: ([[1,3,5,7],[10,11,16,20],[23,30,34,50]], 13) -> false" in {
    val matrix = M(R(1, 3, 5, 7), R(10, 11, 16, 20), R(23, 30, 34, 50))
    searchMatrix(matrix, 13) shouldBe false
  }
  "Example 3: ([], 0) -> false" in {
    val matrix = Array[Array[Int]]()
    searchMatrix(matrix, 0) shouldBe false
  }

  "My test: ([[]], 0) -> false" in {
    val matrix = M(R())
    searchMatrix(matrix, 0) shouldBe false
  }
  "My test: ([[1]], 0) -> false" in {
    val matrix = M(R(1))
    searchMatrix(matrix, 0) shouldBe false
  }
  "My test: ([[0]], 0) -> true" in {
    val matrix = M(R(0))
    searchMatrix(matrix, 0) shouldBe true
  }
  "My test: ([[0,1],[2,3]], 1) -> true" in {
    val matrix = M(R(0, 1), R(2, 3))
    searchMatrix(matrix, 1) shouldBe true
  }
  "My test: ([[0,1],[2,3]], 3) -> true" in {
    val matrix = M(R(0, 1), R(2, 3))
    searchMatrix(matrix, 3) shouldBe true
  }
  "My test: ([[0,1],[2,3]], 4) -> false" in {
    val matrix = M(R(0, 1), R(2, 3))
    searchMatrix(matrix, 4) shouldBe false
  }
  "My test: ([[0,1],[2,3]], -1) -> false" in {
    val matrix = M(R(0, 1), R(2, 3))
    searchMatrix(matrix, -1) shouldBe false
  }
  "My test: ([[1..100]..[9_901..10_000]], 42) -> true" in {
    val length = 100
    val matrix = Array.fill[Array[Int]](length)(Array.ofDim[Int](length))
    for (i <- matrix.indices; j <- matrix(i).indices) {
      matrix(i)(j) = (i * 100) + (j + 1)
    }

    searchMatrix(matrix, 42) shouldBe true
  }
  "My test: ([[1..100]..[9_901..10_000]], 0) -> false" in {
    val length = 100
    val matrix = Array.fill[Array[Int]](length)(Array.ofDim[Int](length))
    for (i <- matrix.indices; j <- matrix(i).indices) {
      matrix(i)(j) = (i * 100) + (j + 1)
    }

    //searchMatrix(matrix, 0) shouldBe false
  }

}
