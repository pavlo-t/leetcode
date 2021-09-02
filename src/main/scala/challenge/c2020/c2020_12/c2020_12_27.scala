package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3582/]]
 */
//noinspection DuplicatedCode
class c2020_12_27 extends AnyWordSpec with Matchers {
  /**
   * === Jump Game IV ===
   *
   * Given an array of integers `arr`, you are initially positioned at the first index of the array.
   *
   * In one step you can jump from index `i` to index:
   *  - `i + 1` where: `i + 1 < arr.length`.
   *  - `i - 1` where: `i - 1 >= 0`.
   *  - `j` where: `arr[i] == arr[j]` and `i != j`.
   *
   * Return ''the minimum number of steps'' to reach the '''last index''' of the array.
   *
   * Notice that you can not jump outside of the array at any time.
   *
   * '''Constraints:'''
   *  - `1 <= arr.length <= 50_000`
   *  - `-10^8 <= arr[i] <= 10^8`
   */
  object Solution {
    def minJumps(A: Array[Int]): Int = {
      import collection.mutable

      val map: mutable.Map[Int, Seq[Int]] =
        A.zipWithIndex
          .foldLeft(mutable.Map[Int, Seq[Int]]().withDefaultValue(Seq())) {
            case (acc, (x, i)) =>
              acc(x) = i +: acc(x)
              acc
          }

      val seen = Array.fill(A.length)(false)
      seen(0) = true

      val T = A.length - 1

      @scala.annotation.tailrec
      def rec(todo: Seq[Int], step: Int): Int = todo match {
        case T +: _     => step
        case -1 +: rest => rec(rest :+ -1, step + 1)
        case i +: rest  =>
          val njs = (map(A(i)) :+ (i - 1) :+ (i + 1)).filter(j => j > 0 && j <= T && !seen(j))
          map(A(i)) = Seq()
          njs.foreach(j => seen(j) = true)
          rec(rest ++ njs, step)

        case Nil => throw new IllegalArgumentException()
      }

      rec(Seq(0, -1), 0)
    }
  }

  import Solution.minJumps

  "Example 1: (arr = [100,-23,-23,404,100,23,23,23,3,404]) -> 3" in {
    minJumps(Array(100, -23, -23, 404, 100, 23, 23, 23, 3, 404)) shouldBe 3
    //Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9. Note that index 9 is the last index of the array.
  }
  "Example 2: (arr = [7]) -> 0" in {
    minJumps(Array(7)) shouldBe 0
    //Explanation: Start index is the last index. You don't need to jump.
  }
  "Example 3: (arr = [7,6,9,6,9,6,9,7]) -> 1" in {
    minJumps(Array(7, 6, 9, 6, 9, 6, 9, 7)) shouldBe 1
    //Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
  }
  "Example 4: (arr = [6,1,9]) -> 2" in {
    minJumps(Array(6, 1, 9)) shouldBe 2
  }
  "Example 5: (arr = [11,22,7,7,7,7,7,7,7,22,13]) -> 3" in {
    minJumps(Array(11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13)) shouldBe 3
  }

  "(arr = [1..=50000]) -> 49999" in {
    minJumps((1 to 50000).toArray) shouldBe 49999
  }

  "(arr = [1 * 50000]) -> 1" in {
    minJumps(Array.fill(50000)(1)) shouldBe 1
  }


}
