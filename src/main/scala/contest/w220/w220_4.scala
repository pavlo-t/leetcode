package contest.w220

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/contest/weekly-contest-220/problems/checking-existence-of-edge-length-limited-paths/]]
 */
//noinspection DuplicatedCode
class w220_4 extends AnyWordSpec with Matchers {
  object Solution {
    def distanceLimitedPathsExist(n: Int, edgeList: Array[Array[Int]], queries: Array[Array[Int]]): Array[Boolean] = {
      val qs = queries.zipWithIndex.sortBy(_._1(2))
      val es = edgeList.sortBy(_ (2))

      val ds = new DisjointSet(n)
      val result = Array.fill(queries.length)(false)

      var i = 0
      for ((Array(p, q, lim), j) <- qs) {
        while (i < es.length && es(i)(2) < lim) {
          val Array(u, v, _) = es(i)
          ds.union(u, v)
          i += 1
        }
        result(j) = ds.find(p) == ds.find(q)
      }

      result
    }

    class DisjointSet(n: Int) {
      private val parents: Array[Int] = (0 until n).toArray
      private val ranks: Array[Int] = Array.fill(n)(0)

      def find(x: Int): Int = {
        if (parents(x) != x)
          parents(x) = find(parents(x))

        parents(x)
      }

      def union(x: Int, y: Int): Unit = {
        def makeNewRoot(x: Int, y: Int): Unit = {
          parents(y) = x
          if (ranks(x) == ranks(y))
            ranks(x) += 1
        }

        val xr = find(x)
        val yr = find(y)

        if (xr != yr)
          if (ranks(xr) < ranks(yr))
            makeNewRoot(yr, xr)
          else
            makeNewRoot(xr, yr)
      }
    }
  }

  object SolutionWithLogging {
    def distanceLimitedPathsExist(n: Int, edgeList: Array[Array[Int]], queries: Array[Array[Int]]): Array[Boolean] = {
      val qs = queries.zipWithIndex.sortInPlaceBy(_._1(2))
      val es = edgeList.sortBy(_ (2))

      Thread.sleep(20)
      println(s"distanceLimitedPathsExist(n=$n," +
        s"  edgeList=${edgeList.map(_.mkString("[", ",", "]")).mkString("[", ",", "]")}," +
        s"  queries=${queries.map(_.mkString("[", ",", "]")).mkString("[", ",", "]")})")

      println(s"es=${es.map(_.mkString("[", ",", "]")).mkString("[", ",", "]")})")
      println(s"qs=${qs.map(e => (e._1.mkString("[", ",", "]"), e._2)).mkString("[", ",", "]")}")

      val ds = new DisjointSet(n)
      val result = Array.fill(queries.length)(false)

      var i = 0
      for ((Array(p, q, lim), j) <- qs) {
        println(s"   i=$i,j=$j,ds=$ds")
        while (i < es.length && es(i)(2) < lim) {
          val Array(u, v, _) = es(i)
          ds.union(u, v)
          println(s"   i=$i,j=$j,ds=$ds")
          i += 1
        }
        println(s"   i=$i,j=$j,ds=$ds")
        result(j) = ds.find(p) == ds.find(q)
      }
      println(s"           ds=$ds")

      result
    }

    class DisjointSet(n: Int) {
      private val parents: Array[Int] = (0 until n).toArray
      private val ranks: Array[Int] = Array.fill(n)(0)

      def find(x: Int): Int = {
        if (parents(x) != x)
          parents(x) = find(parents(x))

        parents(x)
      }

      def union(x: Int, y: Int): Unit = {
        def makeNewRoot(x: Int, y: Int): Unit = {
          parents(y) = x
          if (ranks(x) == ranks(y))
            ranks(x) += 1
        }

        val xr = find(x)
        val yr = find(y)

        if (xr != yr)
          if (ranks(xr) < ranks(yr))
            makeNewRoot(yr, xr)
          else
            makeNewRoot(xr, yr)
      }

      override def toString: String = s"DisjointSet(" +
        s"parents=${parents.mkString("[", ",", "]")}," +
        s"ranks=${ranks.mkString("[", ",", "]")})"
    }
  }

  import Solution.distanceLimitedPathsExist

  "Example 1: (n = 3, edgeList = [[0,1,2],[1,2,4],[2,0,8],[1,0,16]], queries = [[0,1,2],[0,2,5]]) -> [false,true]" in {
    val n = 3
    val edgeList = Array(Array(0, 1, 2), Array(1, 2, 4), Array(2, 0, 8), Array(1, 0, 16))
    val queries = Array(Array(0, 1, 2), Array(0, 2, 5))

    val expected = Array(false, true)

    distanceLimitedPathsExist(n, edgeList, queries) shouldBe expected
    // Explanation:
    // Note that there are two overlapping edges between 0 and 1 with distances 2 and 16.
    //
    // For the first query,
    // between 0 and 1 there is no path where each distance is less than 2,
    // thus we return false for this query.
    //
    // For the second query,
    // there is a path (0 -> 1 -> 2) of two edges with distances less than 5,
    // thus we return true for this query.
  }

  "Example 2: (n = 5, edgeList = [[0,1,10],[1,2,5],[2,3,9],[3,4,13]], queries = [[0,4,14],[1,4,13]]) -> [true,false]" in {
    val n = 5
    val edgeList = Array(Array(0, 1, 10), Array(1, 2, 5), Array(2, 3, 9), Array(3, 4, 13))
    val queries = Array(Array(0, 4, 14), Array(1, 4, 13))

    val expected = Array(true, false)

    distanceLimitedPathsExist(n, edgeList, queries) shouldBe expected
    // Explanation:
    // 1. There is a path (0 -> 1 -> 2 -> 3 -> 4) where all distances are less than 14.
    // 2. Edge [3,4,13] has distance of 13 which is not strictly less than 13.
  }

}
