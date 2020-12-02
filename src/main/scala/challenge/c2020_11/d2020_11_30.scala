package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/568/week-5-november-29th-november-30th/3549/]]
 */
//noinspection DuplicatedCode
class d2020_11_30 extends AnyWordSpec with Matchers {

  /**
   * === The Skyline Problem ===
   *
   * A city's skyline is the outer contour of the silhouette formed by all the buildings in that
   * city when viewed from a distance.
   * Now suppose you are '''given the locations and height of all the buildings''',
   * write a program to '''output the skyline''' formed by these buildings collectively.
   *
   * The geometric information of each building is represented by a triplet of integers `[Li, Ri, Hi]`,
   * where `Li` and `Ri` are the x coordinates of the left and right edge of the ith building, respectively,
   * and `Hi` is its height.
   * It is guaranteed that `0 ≤ Li, Ri ≤ INT_MAX`, `0 < Hi ≤ INT_MAX`, and `Ri - Li > 0`.
   * You may assume all buildings are perfect rectangles grounded on an absolutely flat surface at height 0.
   *
   * The output is a list of '''"key points"''' in the format of `[ [x1,y1], [x2, y2], [x3, y3], ... ]`
   * that uniquely defines a skyline.
   * '''A key point is the left endpoint of a horizontal line segment'''.
   * Note that the last key point, where the rightmost building ends, is merely used to mark
   * the termination of the skyline, and always has zero height.
   * Also, the ground in between any two adjacent buildings should be considered part of the skyline contour.
   *
   * '''Notes:'''
   *  - The number of buildings in any input list is guaranteed to be in the range `[0, 10000]`.
   *  - The input list is already sorted in ascending order by the left x position `Li`.
   *  - The output list must be sorted by the x position.
   *  - There must be no consecutive horizontal lines of equal height in the output skyline.
   *    For instance, `[...[2 3], [4 5], [7 5], [11 5], [12 7]...]` is not acceptable;
   *    the three lines of height 5 should be merged into one in the final output as such:
   *    `[...[2 3], [4 5], [12 7], ...]`
   */
  object Solution {
    private type Result = List[List[Int]]

    def getSkyline(buildings: Array[Array[Int]]): List[List[Int]] =
      if (buildings.isEmpty) List()
      else if (buildings.length == 1) {
        val Array(l, r, h) = buildings(0)
        List(List(l, h), List(r, 0))
      } else {
        val (left, right) = buildings.splitAt(buildings.length / 2)
        merge(getSkyline(left), getSkyline(right))
      }

    private def merge(left: Result, right: Result): Result = {
      def nextRsf(rsf: Result, x: Int, y: Int): Result =
        rsf match {
          case Nil                         => List(x, y) :: Nil
          case List(_, ly) :: _ if y == ly => rsf
          case List(lx, _) :: _ if x != lx => List(x, y) :: rsf
          case _ :: rest                   => List(x, y) :: rest
        }

      @scala.annotation.tailrec
      def loop(l: Result, r: Result, rsf: Result, cly: Int = 0, cry: Int = 0): Result =
        (l, r) match {
          case (Nil, Nil)                => rsf.reverse
          case (List(x, y) :: rest, Nil) => loop(rest, r, nextRsf(rsf, x, y))
          case (Nil, right)              => loop(right, Nil, rsf)

          case (List(lx, ly) :: ls, List(rx, ry) :: rs) =>
            if (lx < rx)
              loop(ls, r, nextRsf(rsf, lx, ly max cry), ly, cry)
            else
              loop(l, rs, nextRsf(rsf, rx, cly max ry), cly, ry)
        }

      loop(left, right, List())
    }
  }

  object SolutionTreeMapImmutable {
    def getSkyline(buildings: Array[Array[Int]]): List[List[Int]] = {
      val walls = buildings
        .foldLeft(List[(Int, Int)]()) { case (acc, Array(li, ri, hi)) => (ri, hi) :: (li, -hi) :: acc }
        .sorted

      import collection.immutable.TreeMap

      @scala.annotation.tailrec
      def buildSkyline(walls: List[(Int, Int)],
                       chs: TreeMap[Int, Int],
                       rsf: List[List[Int]]): List[List[Int]] =
        walls match {
          case Nil            => rsf.reverse
          case (x, y) :: rest =>
            val nChs =
              if (y < 0) chs.updatedWith(-y)(o => Some(o.getOrElse(0) + 1))
              else chs.updatedWith(y)(o => if (o.get == 1) None else Some(o.get - 1))
            val newRsf =
              if (nChs.lastKey != chs.lastKey) List(x, nChs.lastKey) :: rsf
              else rsf

            buildSkyline(rest, nChs, newRsf)
        }

      buildSkyline(walls, TreeMap(0 -> 1), Nil)
    }
  }
  object SolutionMutable {
    def getSkyline(buildings: Array[Array[Int]]): List[List[Int]] = {
      val walls = buildings
        .foldLeft(List[(Int, Int)]()) { case (acc, Array(li, ri, hi)) => (ri, hi) :: (li, -hi) :: acc }
        .sorted

      val chs = collection.mutable.TreeMap[Int, Int](0 -> 1)
      val result = collection.mutable.ListBuffer[List[Int]]()

      @scala.annotation.tailrec
      def buildSkyline(walls: List[(Int, Int)]): Unit =
        if (walls.nonEmpty) {
          val (x, y) :: rest = walls
          val lastHeight = chs.lastKey

          if (y < 0) chs.updateWith(-y)(o => Some(o.getOrElse(0) + 1))
          else chs.updateWith(y)(o => if (o.get == 1) None else Some(o.get - 1))

          if (chs.lastKey != lastHeight)
            result.addOne(List(x, chs.lastKey))

          buildSkyline(rest)
        }

      buildSkyline(walls)
      result.toList
    }
  }

  object SolutionImproved {
    def getSkyline(buildings: Array[Array[Int]]): List[List[Int]] = {
      val walls = buildings
        .foldLeft(List[(Int, Int)]()) { case (acc, Array(li, ri, hi)) => (ri, hi) :: (li, -hi) :: acc }
        .sorted

      @scala.annotation.tailrec
      def buildSkyline(walls: List[(Int, Int)], chs: List[Int], rsf: List[List[Int]]): List[List[Int]] = walls match {
        case Nil            => rsf.reverse
        case (x, y) :: rest =>
          // Keep current heights list sorted in desc order
          val nChs =
            if (y < 0) {
              // add a wall
              val (front, back) = chs.splitAt(chs.indexWhere(-y >= _))
              front ::: -y :: back
            } else {
              // remove a wall
              val (front, back) = chs.splitAt(chs.indexWhere(y >= _))
              front ::: back.tail
            }
          val newRsf =
            if (chs.head != nChs.head) List(x, nChs.head) :: rsf
            else rsf

          buildSkyline(rest, nChs, newRsf)
      }

      buildSkyline(walls, List(0), Nil)
    }
  }
  object SolutionMy {
    def getSkyline(buildings: Array[Array[Int]]): List[List[Int]] = {
      Thread.sleep(20)
      if (buildings.length <= 30)
        println(s"getSkyline(${buildings.map(_.mkString("(", ",", ")")).take(30).mkString("[", ",", "]")}")
      else
        println(s"getSkyline(${buildings.map(_.mkString("(", ",", ")")).take(30).mkString("[", ",", ",...]")}")

      val walls = buildings
        .foldLeft(List[(Int, Int)]()) { case (acc, Array(li, ri, hi)) => (ri, -hi) :: (li, hi) :: acc }
        .sorted

      def getNewCurrentWalls(y: Int, currentWalls: List[Int]): List[Int] =
        if (y > 0) {
          // add a wall
          val (front, back) = currentWalls.splitAt(currentWalls.indexWhere(y >= _))
          front ::: y :: back
        } else {
          // remove a wall
          val (front, back) = currentWalls.splitAt(currentWalls.indexWhere(-y >= _))
          front ::: back.tail
        }


      @scala.annotation.tailrec
      def buildSkyline(walls: List[(Int, Int)],
                       currentWalls: List[Int],
                       rsf: List[List[Int]]): List[List[Int]] = {
        println(s"  buildSkyline(\n    " +
          s"walls=${walls.take(30)},\n    " +
          s"currentWalls=$currentWalls,\n    " +
          s"rsf=${rsf.reverse})")
        walls match {
          case Nil => rsf.reverse
          //case (x0, y0) :: (rest@(x1, y1) :: _) if x0 == x1 =>
          //  buildSkyline(rest, getNewCurrentWalls(y0, currentWalls), rsf)
          case (x, y) :: rest =>
            val newCurrentWalls = getNewCurrentWalls(y, currentWalls)
            val newRsf =
              if (currentWalls.head != newCurrentWalls.head) rsf match {
                case List(x0, y0) :: Nil if x0 == x                    =>
                  List(x, newCurrentWalls.head) :: Nil
                case List(x1, y1) :: (p0@List(_, y0)) :: rr if x1 == x =>
                  if (y1 > newCurrentWalls.head)
                    List(x, newCurrentWalls.head) :: p0 :: rr
                  else if (y0 == newCurrentWalls.head)
                    p0 :: rr
                  else List(x, newCurrentWalls.head) :: p0 :: rr
                case _                                                 =>
                  List(x, newCurrentWalls.head) :: rsf
              } else rsf

            buildSkyline(rest, newCurrentWalls, newRsf)
        }
      }

      buildSkyline(walls, List(0), Nil)
    }
  }

  import Solution.getSkyline

  "Example 1: ([[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]]) -> " +
    "[[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]" in {
    val buildings = Array(Array(2, 9, 10), Array(3, 7, 15), Array(5, 12, 12), Array(15, 20, 10), Array(19, 24, 8))
    val expected = List(List(2, 10), List(3, 15), List(7, 12), List(12, 0), List(15, 10), List(20, 8), List(24, 0))
    getSkyline(buildings) shouldBe expected
  }

  "Example 2: ([(1,5,11),(2,7,6),(3,9,13),(12,16,7),(14,25,3),(19,22,18),(23,29,13),(24,28,4)]) -> " +
    "[(1,11),(3,13),(9,0),(12,7),(16,3),(19,18),(22,3),(23,13),(29,0)]" in {
    val buildings = Array(Array(1, 5, 11), Array(2, 7, 6), Array(3, 9, 13), Array(12, 16, 7), Array(14, 25, 3),
      Array(19, 22, 18), Array(23, 29, 13), Array(24, 28, 4))
    val expected = List(List(1, 11), List(3, 13), List(9, 0), List(12, 7), List(16, 3),
      List(19, 18), List(22, 3), List(23, 13), List(29, 0))
    getSkyline(buildings) shouldBe expected
  }

  "Test 27: ([[0,2,3],[2,5,3]]) -> [[0,3],[5,0]]" in {
    val buildings = Array(Array(0, 2, 3), Array(2, 5, 3))
    val expected = List(List(0, 3), List(5, 0))
    getSkyline(buildings) shouldBe expected
  }

  "([]) -> []" in {
    getSkyline(Array()) shouldBe List()
  }
  "([1,2,3]) -> [[1,3],[2,0]]" in {
    getSkyline(Array(Array(1, 2, 3))) shouldBe List(List(1, 3), List(2, 0))
  }
  "([[1,5,2],[2,4,3]]) -> [[1,2],[2,3],[4,2],[5,0]]" in {
    val buildings = Array(Array(1, 5, 2), Array(2, 4, 3))
    val expected = List(List(1, 2), List(2, 3), List(4, 2), List(5, 0))
    getSkyline(buildings) shouldBe expected
  }
  "([[1,3,1],[2,4,1]]) -> [[1,1],[4,0]]" in {
    val buildings = Array(Array(1, 3, 1), Array(2, 4, 1))
    val expected = List(List(1, 1), List(4, 0))
    getSkyline(buildings) shouldBe expected
  }
  "([[1,2,1],[2,3,1]]) -> [[1,1],[3,0]]" in {
    val buildings = Array(Array(1, 2, 1), Array(2, 3, 1))
    val expected = List(List(1, 1), List(3, 0))
    getSkyline(buildings) shouldBe expected
  }
  "([[1,2,1],[3,4,1]]) -> [[1,1],[2,0],[3,1],[4,0]]" in {
    val buildings = Array(Array(1, 2, 1), Array(3, 4, 1))
    val expected = List(List(1, 1), List(2, 0), List(3, 1), List(4, 0))
    getSkyline(buildings) shouldBe expected
  }
  "([[1,3,1],[1,2,2]]) -> [[1,2],[2,1],[3,0]]" in {
    val buildings = Array(Array(1, 3, 1), Array(1, 2, 2))
    val expected = List(List(1, 2), List(2, 1), List(3, 0))
    getSkyline(buildings) shouldBe expected
  }
  "([[1,3,1],[2,3,2]]) -> [[1,1],[2,2],[3,0]]" in {
    val buildings = Array(Array(1, 3, 1), Array(2, 3, 2))
    val expected = List(List(1, 1), List(2, 2), List(3, 0))
    getSkyline(buildings) shouldBe expected
  }
  "([[1,4,2],[2,3,1]]) -> [[1,2],[4,0]]" in {
    val buildings = Array(Array(1, 4, 2), Array(2, 3, 1))
    val expected = List(List(1, 2), List(4, 0))
    getSkyline(buildings) shouldBe expected
  }
  "([[1,2,1],[1,2,1]]) -> [[1,1],[2,0]]" in {
    val buildings = Array(Array(1, 2, 1), Array(1, 2, 1))
    val expected = List(List(1, 1), List(2, 0))
    getSkyline(buildings) shouldBe expected
  }

  "([[1,2,5],[1,3,4],[1,4,3],[1,5,2],[1,6,1]]) -> [[1,5],[2,4],[3,3],[4,2],[5,1],[6,0]]" in {
    val buildings = Array(Array(1, 2, 5), Array(1, 3, 4), Array(1, 4, 3), Array(1, 5, 2), Array(1, 6, 1))
    val expected = List(List(1, 5), List(2, 4), List(3, 3), List(4, 2), List(5, 1), List(6, 0))
    getSkyline(buildings) shouldBe expected
  }
  "([[1,6,1],[1,5,2],[1,4,3],[1,3,4],[1,2,5]]) -> [[1,5],[2,4],[3,3],[4,2],[5,1],[6,0]]" in {
    val buildings = Array(Array(1, 6, 1), Array(1, 5, 2), Array(1, 4, 3), Array(1, 3, 4), Array(1, 2, 5))
    val expected = List(List(1, 5), List(2, 4), List(3, 3), List(4, 2), List(5, 1), List(6, 0))
    getSkyline(buildings) shouldBe expected
  }

  "(10000 buildings) -> ???" in {
    import util.Random.nextInt
    val buildings =
      (0 until 10000)
        .map(li => Array(li, nextInt(10000) + 1 + li, nextInt(10000) + 1))
        .toArray
    val result = getSkyline(buildings)
    result.size should be >= 0
  }
}
