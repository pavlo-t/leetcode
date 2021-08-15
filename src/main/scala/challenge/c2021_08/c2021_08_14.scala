package challenge.c2021_08

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/614/week-2-august-8th-august-14th/3889/]]
 */
class c2021_08_14 extends AnyWordSpec with Matchers {
  /**
   * == Remove Boxes ==
   *
   * You are given several `boxes` with different colors represented by different positive numbers.
   *
   * You may experience several rounds to remove boxes until there is no box left.
   * Each time you can choose some continuous boxes with the same color (i.e., composed of `k` boxes, `k >= 1`),
   * remove them and get `k * k` points.
   *
   * Return _the maximum points you can get_.
   *
   * '''Constraints:'''
   *  - `1 <= boxes.length <= 100`
   *  - `1 <= boxes[i] <= 100`
   */
  object Solution {
    /** [[https://leetcode.com/problems/remove-boxes/solution/]] */
    def removeBoxes(boxes: Array[Int]): Int = {
      val n = boxes.length
      val dp = Array.ofDim[Int](n, n, n)

      @scala.annotation.tailrec
      def leftMostSame(l: Int, r: Int, k: Int): (Int, Int) =
        if (l >= r || boxes(r - 1) != boxes(r)) (r, k)
        else leftMostSame(l, r - 1, k + 1)

      def calculatePoints(l: Int, ri: Int, ki: Int): Int = {
        if (l > ri) 0
        else {
          val (r, k) = leftMostSame(l, ri, ki)
          if (dp(l)(r)(k) != 0) dp(l)(r)(k)
          else {
            dp(l)(r)(k) = calculatePoints(l, r - 1, 0) + (k + 1) * (k + 1)
            for (i <- l until r if boxes(i) == boxes(r))
              dp(l)(r)(k) = dp(l)(r)(k).max(calculatePoints(l, i, k + 1) + calculatePoints(i + 1, r - 1, 0))

            dp(l)(r)(k)
          }
        }
      }

      calculatePoints(0, n - 1, 0)
    }
    /** [[https://massivealgorithms.blogspot.com/2017/04/leetcode-546-remove-boxes.html]] */
    def removeBoxes_working(boxes: Array[Int]): Int = {
      val n = boxes.length
      val dp = Array.ofDim[Int](n, n, n)

      for (i <- 0 until n; k <- 0 to i)
        dp(i)(i)(k) = (k + 1) * (k + 1)

      for {
        t <- 1 until n
        j <- t until n
        i = j - t
        k <- 0 to i
      } {
        dp(i)(j)(k) = (k + 1) * (k + 1) + dp(i + 1)(j)(0)
        for (m <- i + 1 to j if boxes(m) == boxes(i))
          dp(i)(j)(k) = dp(i)(j)(k).max(dp(i + 1)(m - 1)(0) + dp(m)(j)(k + 1))
      }

      dp(0)(n - 1)(0)
    }

    /** [[https://xiaoguan.gitbooks.io/leetcode/content/LeetCode/546-remove-boxes-hard.html]] */
    def removeBoxes_MemoryLimitExceeded_TooSlow(boxes: Array[Int]): Int = {
      val n = boxes.length
      val dp = Array.ofDim[Int](n, n, n)

      @scala.annotation.tailrec
      def leftMostSame(l: Int, r: Int): Int =
        if (l >= r || boxes(r - 1) != boxes(r)) r
        else leftMostSame(l, r - 1)

      def rec(l: Int, rs: Int, lenS: Int): Int =
        if (l > rs) 0
        else if (dp(l)(rs)(lenS) != 0) dp(l)(rs)(lenS)
        else {
          val r = leftMostSame(l, rs)
          val len = lenS + rs - r
          dp(l)(r)(len) = rec(l, r - 1, 0) + (len + 1) * (len + 1)
          for (i <- l until r if boxes(i) == boxes(r))
            dp(l)(r)(len) = dp(l)(r)(len) max (rec(l, i, len + 1) + rec(i + 1, r - 1, 0))
          dp(l)(r)(len)
        }

      rec(0, n - 1, 0)
    }

    def removeBoxesTailRec(boxes: Array[Int]): Int = {
      val n = boxes.length
      @scala.annotation.tailrec
      def group(i: Int, rsf: Seq[(Int, Int)]): Seq[(Int, Int)] =
        if (i == n) rsf.reverse
        else rsf match {
          case (n, c) +: rest if n == boxes(i) => group(i + 1, (n, c + 1) +: rest)
          case _                               => group(i + 1, (boxes(i), 1) +: rsf)
        }

      def sameColorLR(i: Int, bs: Seq[(Int, Int)]): Boolean =
        bs(i - 1)._1 == bs(i + 1)._1

      def mergeLR(i: Int, bs: Seq[(Int, Int)]): Seq[(Int, Int)] =
        bs.patch(i - 1, Iterator.single((bs(i - 1)._1, bs(i - 1)._2 + bs(i + 1)._2)), 3)

      @scala.annotation.tailrec
      def removeRec(todo: Seq[(Seq[(Int, Int)], Int)], gr: Int): Int = todo match {
        case Nil              => gr
        case (bs, lr) +: rest =>
          if (bs.isEmpty) removeRec(rest, gr max lr)
          else if (bs.size < 3) removeRec(rest, gr.max(lr + bs.map { case (_, i) => i * i }.sum))
          else removeRec(
            rest ++ bs.indices.map(i => (i match {
              case 0                       => bs.tail
              case i if i == bs.size - 1   => bs.init
              case i if sameColorLR(i, bs) => mergeLR(i, bs)
              case i                       => bs.patch(i, Nil, 1)
            }, lr + bs(i)._2 * bs(i)._2)),
            gr)
      }

      removeRec(Seq((group(1, Seq((boxes(0), 1))), 0)), 0)
    }
    def removeBoxesRecursionBruteForce(boxes: Array[Int]): Int = {
      val n = boxes.length
      @scala.annotation.tailrec
      def group(i: Int, rsf: Seq[(Int, Int)]): Seq[(Int, Int)] =
        if (i == n) rsf.reverse
        else rsf match {
          case (n, c) +: rest if n == boxes(i) => group(i + 1, (n, c + 1) +: rest)
          case _                               => group(i + 1, (boxes(i), 1) +: rsf)
        }

      def sameColorLR(i: Int, bs: Seq[(Int, Int)]): Boolean =
        bs(i - 1)._1 == bs(i + 1)._1

      def mergeLR(i: Int, bs: Seq[(Int, Int)]): Seq[(Int, Int)] =
        bs.patch(i - 1, Iterator.single((bs(i - 1)._1, bs(i - 1)._2 + bs(i + 1)._2)), 3)

      def removeRec(bs: Seq[(Int, Int)]): Int =
        if (bs.isEmpty) 0
        else if (bs.size < 3) bs.map { case (_, i) => i * i }.sum
        else bs.indices.map(i => (i match {
          case 0                       => removeRec(bs.tail)
          case i if i == bs.size - 1   => removeRec(bs.init)
          case i if sameColorLR(i, bs) => removeRec(mergeLR(i, bs))
          case i                       => removeRec(bs.patch(i, Nil, 1))
        }) + bs(i)._2 * bs(i)._2).max

      removeRec(group(1, Seq((boxes(0), 1))))
    }
  }

  "Example 1: (boxes = [1,3,2,2,2,3,4,3,1]) -> 23" in {
    Solution.removeBoxes(Array(1, 3, 2, 2, 2, 3, 4, 3, 1)) shouldBe 23
    // Explanation:
    // [1, 3, 2, 2, 2, 3, 4, 3, 1]
    // ----> [1, 3, 3, 4, 3, 1] (3*3=9 points)
    // ----> [1, 3, 3, 3, 1] (1*1=1 points)
    // ----> [1, 1] (3*3=9 points)
    // ----> [] (2*2=4 points)
  }
  "Example 2: (boxes = [1,1,1]) -> 9" in (Solution.removeBoxes(Array(1, 1, 1)) shouldBe 9)
  "Example 3: (boxes = [1]) -> 1" in (Solution.removeBoxes(Array(1)) shouldBe 1)

  "(boxes = [3]) -> 1" in (Solution.removeBoxes(Array(3)) shouldBe 1)
  "(boxes = [3,3,3]) -> 9" in (Solution.removeBoxes(Array(3, 3, 3)) shouldBe 9)

  "(boxes = [1,2,2,1,1,1]) -> 20" in (Solution.removeBoxes(Array(1, 2, 2, 1, 1, 1)) shouldBe 20)
  "(boxes = [1..=100]) -> 100" in (Solution.removeBoxes((1 to 100).toArray) shouldBe 100)

  "test 21: -> 2758" in {
    val boxes = Array(1, 2, 2, 1, 1, 1, 2, 1, 1, 2, 1, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 1, 2, 2, 2, 2, 1, 2, 1, 1, 2, 2,
      1, 2, 1, 2, 2, 2, 2, 2, 1, 2, 1, 2, 2, 1, 1, 1, 2, 2, 1, 2, 1, 2, 2, 1, 2, 1, 1, 1, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2,
      2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1)
    Solution.removeBoxes(boxes) shouldBe 2758
  }
}
