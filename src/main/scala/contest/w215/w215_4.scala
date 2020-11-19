package contest.w215

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/contest/weekly-contest-215/problems/maximize-grid-happiness/]]
 */
//noinspection DuplicatedCode
class w215_4 extends AnyWordSpec with Matchers {
  /**
   * === 1659. Maximize Grid Happiness ===
   *
   * You are given four integers, `m`, `n`, `introvertsCount`, and `extrovertsCount`.
   * You have an `m x n` grid, and there are two types of people: introverts and extroverts.
   * There are `introvertsCount` introverts and `extrovertsCount` extroverts.
   *
   * You should decide how many people you want to live in the grid and assign each of them one grid cell.
   * Note that you '''do not''' have to have all the people living in the grid.
   *
   * The '''happiness''' of each person is calculated as follows:
   *  - Introverts '''start''' with `120` happiness and '''lose''' `30` happiness for each neighbor (introvert or extrovert).
   *  - Extroverts '''start''' with `40` happiness and '''gain''' `20` happiness for each neighbor (introvert or extrovert).
   *
   * Neighbors live in the directly adjacent cells north, east, south, and west of a person's cell.
   *
   * The '''grid happiness''' is the '''sum''' of each person's happiness.
   * Return ''the '''maximum possible grid happiness'''''.
   *
   * '''Constraints:'''
   *  - `1 <= m, n <= 5`
   *  - `0 <= introvertsCount, extrovertsCount <= min(m * n, 6)`
   */
  object Solution {
    def getMaxGridHappiness(m: Int, n: Int, introvertsCount: Int, extrovertsCount: Int): Int = {
      /**
       * `mem` a ternary flag container of size `n`.
       *
       * `memSize` is a limit on `mem`, we only need to keep track of `n` positions to get upper and left neighbor.
       *
       * ''Trite meaning:'' 0: empty, 1: introvert, 2: extrovert
       *
       * ''Example:''
       * {{{
       * 0d -> 00t
       * 1d -> 01t
       * 2d -> 02t
       * 3d -> 10t
       * 4d -> 11t
       * 5d -> 12t
       * }}}
       */
      val memSize = math.pow(3, n).toInt

      def addToMem(mem: Int, t: Int): Int = (mem * 3 + t) % memSize
      def getLeft(mem: Int): Int = mem % 3
      def getUp(mem: Int): Int = mem / (memSize / 3)

      def intH(n: Int): Int = if (n == 0) 0 else if (n == 1) -60 else -10
      def extH(n: Int): Int = if (n == 0) 0 else if (n == 1) -10 else 40

      val cache = Array.ofDim[Int](m, n, introvertsCount + 1, extrovertsCount + 1, memSize)

      def getMaxHappiness(row: Int, col: Int, is: Int, es: Int, mem: Int): Int = {
        if (is == 0 && es == 0) 0
        else if (col == n) getMaxHappiness(row + 1, 0, is, es, mem)
        else if (row == m) 0
        else if (cache(row)(col)(is)(es)(mem) != 0) cache(row)(col)(is)(es)(mem)
        else {
          val nCol = col + 1
          val up = if (row > 0) getUp(mem) else 0
          val left = if (col > 0) getLeft(mem) else 0

          val r0 =
            getMaxHappiness(row, nCol, is, es, addToMem(mem, 0))
          val r1 = if (is == 0) 0 else {
            val localHappiness = 120 + intH(left) + intH(up)
            getMaxHappiness(row, nCol, is - 1, es, addToMem(mem, 1)) + localHappiness
          }
          val r2 = if (es == 0) 0 else {
            val localHappiness = 40 + extH(left) + extH(up)
            getMaxHappiness(row, nCol, is, es - 1, addToMem(mem, 2)) + localHappiness
          }
          val res = r0 max r1 max r2
          cache(row)(col)(is)(es)(mem) = res
          res
        }
      }

      getMaxHappiness(0, 0, introvertsCount, extrovertsCount, 0)
    }
  }

  object Solution1 {
    def getMaxGridHappiness(m: Int, n: Int, introvertsCount: Int, extrovertsCount: Int): Int = {
      /**
       * `mem` is a sequence of last `n` trites, each takes 2 bits.
       *
       * `memSize` is a limit on `mem`, we only need to keep track of `n` positions to get upper and left neighbor
       *
       * ''Trite meaning:'' 0: empty, 1: introvert, 2: extrovert
       *
       * ''Example:''
       * {{{
       * 0000b -> 00t
       * 0001b -> 01t
       * 0010b -> 02t
       * 0011b -> error
       * 0100b -> 10t
       * 0101b -> 11t
       * }}}
       */
      val memSize = (1 << (n << 1)) - 1

      def addToMem(mem: Int, t: Int): Int = ((mem << 2) | t) & memSize
      def getLeft(mem: Int): Int = mem & 3
      def getUp(mem: Int): Int = mem >> ((n - 1) << 1)

      def intH(n: Int): Int = if (n == 0) 0 else if (n == 1) -60 else -10
      def extH(n: Int): Int = if (n == 0) 0 else if (n == 1) -10 else 40

      val cache = Array.ofDim[Int](m, n, introvertsCount + 1, extrovertsCount + 1, memSize)

      def getMaxHappiness(row: Int, col: Int, is: Int, es: Int, mem: Int): Int = {
        if (is == 0 && es == 0) 0
        else if (col == n) getMaxHappiness(row + 1, 0, is, es, mem)
        else if (row == m) 0
        else if (cache(row)(col)(is)(es)(mem) != 0) cache(row)(col)(is)(es)(mem)
        else {
          val nCol = col + 1
          val up = if (row > 0) getUp(mem) else 0
          val left = if (col > 0) getLeft(mem) else 0

          val r0 =
            getMaxHappiness(row, nCol, is, es, addToMem(mem, 0))
          val r1 = if (is == 0) 0 else {
            val localHappiness = 120 + intH(left) + intH(up)
            getMaxHappiness(row, nCol, is - 1, es, addToMem(mem, 1)) + localHappiness
          }
          val r2 = if (es == 0) 0 else {
            val localHappiness = 40 + extH(left) + extH(up)
            getMaxHappiness(row, nCol, is, es - 1, addToMem(mem, 2)) + localHappiness
          }
          val res = r0 max r1 max r2
          cache(row)(col)(is)(es)(mem) = res
          res
        }
      }

      getMaxHappiness(0, 0, introvertsCount, extrovertsCount, 0)
    }
  }

  import Solution.getMaxGridHappiness

  "Example 1: (m = 2, n = 3, introvertsCount = 1, extrovertsCount = 2) -> 240" in {
    getMaxGridHappiness(2, 3, 1, 2) shouldBe 240
    //Explanation: Assume the grid is 1-indexed with coordinates (row, column).
    //We can put the introvert in cell (1,1) and put the extroverts in cells (1,3) and (2,3).
    //- Introvert at (1,1) happiness: 120 (starting happiness) - (0 * 30) (0 neighbors) = 120
    //- Extrovert at (1,3) happiness: 40 (starting happiness) + (1 * 20) (1 neighbor) = 60
    //- Extrovert at (2,3) happiness: 40 (starting happiness) + (1 * 20) (1 neighbor) = 60
    //The grid happiness is 120 + 60 + 60 = 240.
  }
  "Example 2: (m = 3, n = 1, introvertsCount = 2, extrovertsCount = 1) -> 260" in {
    getMaxGridHappiness(3, 1, 2, 1) shouldBe 260
    //Explanation: Place the two introverts in (1,1) and (3,1) and the extrovert at (2,1).
    //- Introvert at (1,1) happiness: 120 (starting happiness) - (1 * 30) (1 neighbor) = 90
    //- Extrovert at (2,1) happiness: 40 (starting happiness) + (2 * 20) (2 neighbors) = 80
    //- Introvert at (3,1) happiness: 120 (starting happiness) - (1 * 30) (1 neighbor) = 90
    //The grid happiness is 90 + 80 + 90 = 260.
  }
  "Example 3: (m = 2, n = 2, introvertsCount = 4, extrovertsCount = 0) -> 240" in {
    getMaxGridHappiness(2, 2, 4, 0) shouldBe 240
  }

  "simple grids" should {
    "(m = 1, n = 1, introvertsCount = 1, extrovertsCount = 0) -> 120" in {
      getMaxGridHappiness(1, 1, 1, 0) shouldBe 120
    }
    "(m = 1, n = 1, introvertsCount = 0, extrovertsCount = 1) -> 40" in {
      getMaxGridHappiness(1, 1, 0, 1) shouldBe 40
    }
    "(m = 1, n = 1, introvertsCount = 1, extrovertsCount = 1) -> 120" in {
      getMaxGridHappiness(1, 1, 1, 1) shouldBe 120
    }

    "(m = 2, n = 1, introvertsCount = 1, extrovertsCount = 1) -> 150" in {
      getMaxGridHappiness(2, 1, 1, 1) shouldBe 150
    }
    "(m = 1, n = 2, introvertsCount = 1, extrovertsCount = 1) -> 150" in {
      getMaxGridHappiness(1, 2, 1, 1) shouldBe 150
    }
    "(m = 2, n = 1, introvertsCount = 2, extrovertsCount = 0) -> 180" in {
      getMaxGridHappiness(2, 1, 2, 0) shouldBe 180
    }
    "(m = 1, n = 2, introvertsCount = 2, extrovertsCount = 0) -> 180" in {
      getMaxGridHappiness(1, 2, 2, 0) shouldBe 180
    }
    "(m = 2, n = 1, introvertsCount = 0, extrovertsCount = 2) -> 120" in {
      getMaxGridHappiness(2, 1, 0, 2) shouldBe 120
    }
    "(m = 1, n = 2, introvertsCount = 0, extrovertsCount = 2) -> 120" in {
      getMaxGridHappiness(1, 2, 0, 2) shouldBe 120
    }
  }
  "(m = 5, n = 5, introvertsCount = 6, extrovertsCount = 6) -> 1240" in {
    getMaxGridHappiness(5, 5, 6, 6) shouldBe 1240
  }
}
