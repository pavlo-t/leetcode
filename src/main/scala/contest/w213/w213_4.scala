package contest.w213

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/contest/weekly-contest-213/problems/kth-smallest-instructions]]
 */
//noinspection DuplicatedCode
class w213_4 extends AnyWordSpec with Matchers {

  /**
   * <h3>1643. Kth Smallest Instructions</h3>
   *
   * Bob is standing at cell `(0, 0)`, and he wants to reach `destination`: `(row, column)`.
   * He can only travel <b>right</b> and <b>down</b>.
   * You are going to help Bob by providing <b>instructions</b> for him to reach `destination`.
   *
   * The instructions are represented as a string, where each character is either:<ul>
   * <li> `'H'`, meaning move horizontally (go <b>right</b>), or
   * <li> `'V'`, meaning move vertically (go <b>down</b>).
   * </ul>
   *
   * Multiple <b>instructions</b> will lead Bob to `destination`.
   * For example, if destination is `(2, 3)`, both `"HHHVV"` and `"HVHVH"` are valid <b>instructions</b>.
   *
   * However, Bob is very picky.
   * Bob has a lucky number `k`, and he wants the `k`th <b>lexicographically smallest instructions</b> that will
   * lead him to `destination`.
   * `k` is <b>1-indexed</b>.
   *
   * Given an integer array `destination` and an integer `k`,
   * return <em>the `k`th <b>lexicographically smallest instructions</b> that will take Bob to</em> `destination`.
   *
   * <b>Constraints:</b><ul>
   * <li> `destination.length == 2`
   * <li> `1 <= row, column <= 15`
   * <li> `1 <= k <= nCr(row + column, row)`, where `nCr(a, b)` denotes `a` choose `b`.
   * </ul>
   */
  object Solution {
    // Build Pascal's triangle:
    val cit: Array[Long] => Array[Long] =
      (_: Array[Long])
        .scanLeft((0L, 0L)) { case ((p, _), n) => (n, p + n) }
        .map(_._2)
        .tail :+ 1L
    val C: Array[Array[Long]] = Iterator.iterate(Array(0L, 1L))(cit).take(31).toArray

    def kth(h: Int, v: Int, k: Long): String =
      if (h + v == 0) ""
      else if (k < C(h + v - 1)(h)) 'H' +: kth(h - 1, v, k)
      else 'V' +: kth(h, v - 1, k - C(h + v - 1)(h))

    def kthSmallestPath(destination: Array[Int], k: Int): String = {
      val Array(v, h) = destination
      val res = kth(h, v, k - 1)
      res
    }
  }

  object SolutionCombinatorics {
    def kthSmallestPath(destination: Array[Int], k: Int): String = {
      @scala.annotation.tailrec
      def build(hs: Int, vs: Int, remK: BigInt = k, rsf: String = ""): String = {
        if (hs <= 0 && vs <= 0) rsf
        else if (hs == 0) rsf + "V" * vs
        else if (vs == 0) rsf + "H" * hs
        else {
          val combs = nCr(hs + vs - 1, vs)
          if (combs >= remK) build(hs - 1, vs, remK, rsf + 'H')
          else build(hs, vs - 1, remK - combs, rsf + 'V')
        }
      }

      build(destination(1), destination(0))
    }

    @scala.annotation.tailrec
    private def factorial(i: BigInt, rsf: BigInt = 1): BigInt =
      if (i <= 1) rsf
      else factorial(i - 1, rsf * i)

    private def nCr(n: BigInt, k: BigInt): BigInt = factorial(n) / (factorial(k) * factorial(n - k))
  }

  import Solution.kthSmallestPath

  """Example 1: (destination = [2,3], k = 1) -> "HHHVV"""" in {
    kthSmallestPath(Array(2, 3), 1) shouldBe "HHHVV"
    // Explanation: All the instructions that reach (2, 3) in lexicographic order are as follows:
    //   ["HHHVV", "HHVHV", "HHVVH", "HVHHV", "HVHVH", "HVVHH", "VHHHV", "VHHVH", "VHVHH", "VVHHH"].
  }
  """Example 2: (destination = [2,3], k = 2) -> "HHVHV"""" in {
    kthSmallestPath(Array(2, 3), 2) shouldBe "HHVHV"
  }
  """Example 3: (destination = [2,3], k = 3) -> "HHVVH"""" in {
    kthSmallestPath(Array(2, 3), 3) shouldBe "HHVVH"
  }

  """(destination = [2,3], k = 4) -> "HVHHV"""" in {
    kthSmallestPath(Array(2, 3), 4) shouldBe "HVHHV"
  }
  """(destination = [2,3], k = 5) -> "HVHVH"""" in {
    kthSmallestPath(Array(2, 3), 5) shouldBe "HVHVH"
  }
  """(destination = [2,3], k = 6) -> "HVVHH"""" in {
    kthSmallestPath(Array(2, 3), 6) shouldBe "HVVHH"
  }
  """(destination = [2,3], k = 7) -> "VHHHV"""" in {
    kthSmallestPath(Array(2, 3), 7) shouldBe "VHHHV"
  }
  """(destination = [2,3], k = 8) -> "VHHVH"""" in {
    kthSmallestPath(Array(2, 3), 8) shouldBe "VHHVH"
  }
  """(destination = [2,3], k = 9) -> "VHVHH"""" in {
    kthSmallestPath(Array(2, 3), 9) shouldBe "VHVHH"
  }
  """(destination = [2,3], k = 10) -> "VVHHH"""" in {
    kthSmallestPath(Array(2, 3), 10) shouldBe "VVHHH"
  }

  """(destination = [15,15], k = 1) -> "HHHHHHHHHHHHHHHVVVVVVVVVVVVVVV"""" in {
    kthSmallestPath(Array(15, 15), 1) shouldBe "HHHHHHHHHHHHHHHVVVVVVVVVVVVVVV"
  }
  """(destination = [15,15], k = 2) -> "HHHHHHHHHHHHHHVHVVVVVVVVVVVVVV"""" in {
    kthSmallestPath(Array(15, 15), 2) shouldBe "HHHHHHHHHHHHHHVHVVVVVVVVVVVVVV"
  }
  """(destination = [15,15], k = max) -> "VVVVVVVVVVVVVVVHHHHHHHHHHHHHHH"""" in {
    kthSmallestPath(Array(15, 15), 155_117_520) shouldBe "VVVVVVVVVVVVVVVHHHHHHHHHHHHHHH"
  }
}
