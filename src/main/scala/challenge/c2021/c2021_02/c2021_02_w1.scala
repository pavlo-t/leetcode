package challenge.c2021.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3624/]]
 */
//noinspection DuplicatedCode,ScalaUnusedSymbol
class c2021_02_w1 extends AnyWordSpec with Matchers {
  /**
   * === Squirrel Simulation ===
   *
   * There's a tree, a squirrel, and several nuts.
   * Positions are represented by the cells in a 2D grid.
   * Your goal is to find the '''minimal''' distance for the squirrel to collect all the nuts
   * and put them under the tree one by one.
   * The squirrel can only take at most one nut at '''one time''' and can move in four directions -
   * up, down, left and right, to the adjacent cell.
   * The distance is represented by the number of moves.
   *
   * '''Note:'''
   *  - All given positions won't overlap.
   *  - The squirrel can take at most one nut at one time.
   *  - The given positions of nuts have no order.
   *  - Height and width are positive integers. 3 <= height * width <= 10,000.
   *  - The given positions contain at least one nut, only one tree and one squirrel.
   */
  object Solution {
    def minDistance(height: Int, width: Int, tree: Array[Int], squirrel: Array[Int], nuts: Array[Array[Int]]): Int = {
      def distance(p1: Array[Int], p2: Array[Int]): Int = (p1(0) - p2(0)).abs + (p1(1) - p2(1)).abs

      val (total, maxSaving) = nuts.foldLeft((0, Int.MinValue)) { case ((total, maxSaving), nut) =>
        val sd = distance(squirrel, nut)
        val td = distance(tree, nut)
        (total + td * 2, maxSaving max (td - sd))
      }

      total - maxSaving
    }
  }

  object SolutionMy {
    def minDistance(height: Int, width: Int, tree: Array[Int], squirrel: Array[Int], nuts: Array[Array[Int]]): Int = {
      def distance(p1: Array[Int], p2: Array[Int]): Int = (p1(0) - p2(0)).abs + (p1(1) - p2(1)).abs

      val sds = nuts.map(distance(squirrel, _))
      val tds = nuts.map(distance(tree, _))
      val ttd = tds.sum * 2

      @scala.annotation.tailrec
      def rec(i: Int, rsf: Int): Int = {
        if (i == nuts.length) rsf
        else rec(i + 1, rsf min (ttd - tds(i) + sds(i)))
      }

      rec(0, Int.MaxValue)
    }
  }

  import Solution.minDistance

  "Example: (Height:5,Width:7,Tree position:[2,2],Squirrel:[4,4],Nuts:[[3,0],[2,5]]) -> 12" in {
    minDistance(5, 7, Array(2, 2), Array(4, 4), Array(Array(3, 0), Array(2, 5))) shouldBe 12
    //Explanation:
    //   0123456
    // 0|       |
    // 1|       |
    // 2|  T  N |
    // 3|N      |
    // 4|    S  |
    //
    // Optimal moves:
    // 1. collect nut at [2,5]: [4,5], [3,5], [2,5]
    // 2. get it to the tree:   [2,4], [2,3], [2,2]
    // 3. collect nut at [3,0]: [2,1], [2,0], [3,0]
    // 4. get it to the tree:   [3,1], [3,2], [2,2]
  }

  "(Height:1,Width:3,Tree position:[0,0],Squirrel:[0,1],Nuts:[[0,2]]) -> 3" in {
    minDistance(1, 3, Array(0, 0), Array(0, 1), Array(Array(0, 2))) shouldBe 3
  }
  "(Height:1,Width:3,Tree position:[0,1],Squirrel:[0,0],Nuts:[[0,2]]) -> 3" in {
    minDistance(1, 3, Array(0, 1), Array(0, 0), Array(Array(0, 2))) shouldBe 3
  }
  "(Height:1,Width:3,Tree position:[0,2],Squirrel:[0,0],Nuts:[[0,1]]) -> 2" in {
    minDistance(1, 3, Array(0, 2), Array(0, 0), Array(Array(0, 1))) shouldBe 2
  }

}
