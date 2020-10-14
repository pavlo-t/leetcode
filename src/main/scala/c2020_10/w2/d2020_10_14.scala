package c2020_10.w2

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


//noinspection DuplicatedCode
class d2020_10_14 extends AnyWordSpec with Matchers {

  /**
   * <h3>House Robber II</h3>
   *
   * You are a professional robber planning to rob houses along a street.
   * Each house has a certain amount of money stashed.
   * All houses at this place are <b>arranged in a circle</b>.
   * That means the first house is the neighbor of the last one.
   * Meanwhile, adjacent houses have a security system connected,
   * and <b>it will automatically contact the police if two adjacent houses were broken into on the same night</b>.
   *
   * Given a list of non-negative integers `nums` representing the amount of money of each house,
   * return <em>the maximum amount of money you can rob tonight <b>without alerting the police</b></em>.
   *
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= nums.length <= 100`
   * <li> `0 <= nums[i] <= 1000`
   * </ul>
   */
  object Solution {
    def rob(nums: Array[Int]): Int = {
      if (nums.length <= 3)
        nums.max
      else
        math.max(robL(nums.drop(1)), robL(nums.dropRight(1)))
    }

    private def robL(nums: Array[Int]): Int = {
      var prev = 0
      var max = 0
      for (num <- nums) {
        val next = math.max(max, prev + num)
        prev = max
        max = next
      }
      max
    }
  }

  object SolutionRecursiveWithCache {
    def rob(nums: Array[Int]): Int = {
      if (nums.length <= 3) nums.max
      else {
        val size = nums.length - 1
        math.max(loop(nums.take(size)), loop(nums.takeRight(size)))
      }
    }

    private def loop(nums: Array[Int]): Int = {
      val cache = Array.fill[Int](nums.length)(-1)

      def doLoop(i: Int): Int = {
        if (i < 0) 0
        else if (cache(i) >= 0) cache(i)
        else {
          cache(i) = math.max(doLoop(i - 2) + nums(i), doLoop(i - 1))
          cache(i)
        }
      }
      doLoop(nums.length - 1)
    }
  }
  object SolutionRecursive {
    def rob(nums: Array[Int]): Int = {
      if (nums.length <= 3) nums.max
      else {
        val size = nums.length - 1
        math.max(rob(nums.take(size), size - 1), rob(nums.takeRight(size), size - 1))
      }
    }

    private def rob(nums: Array[Int], i: Int): Int = {
      if (i < 0) 0
      else math.max(rob(nums, i - 2) + nums(i), rob(nums, i - 1))
    }
  }
  object SolutionBruteForceOptimized {
    import scala.annotation.tailrec

    def rob(nums: Array[Int]): Int = {
      if (nums.length <= 3) nums.max
      else {
        def getSum(in: Set[Int]): Int = in.toSeq.map(nums(_)).sum

        def getNext(in: Set[Int]): Set[Set[Int]] = {
          val lastIdx = nums.length - (if (in.contains(0)) 2 else 1)
          val min = in.max + 2
          Set(min, min + 1) filter (_ <= lastIdx) map (in + _)
        }

        @tailrec
        def loop(todo: Set[Set[Int]], rsf: Int): Int = {
          if (todo.isEmpty) rsf
          else {
            val next = getNext(todo.head)
            if (next.isEmpty)
              loop(todo.tail, math.max(rsf, getSum(todo.head)))
            else
              loop(next ++ todo.tail, rsf)
          }
        }

        loop(Set(Set(0), Set(1), Set(2)), 0)
      }
    }
  }
  object SolutionBruteForce {
    import scala.annotation.tailrec

    def rob(nums: Array[Int]): Int = {
      if (nums.length < 2) nums(0)
      else {
        def getSum(in: Set[Int]): Int = in.toSeq.map(nums(_)).sum

        def getNext(in: Set[Int]): Set[Set[Int]] = {
          val lastIdx = nums.length - 1
          nums.indices.filter { i =>
            !in.contains(i) && !in.contains(i + 1) && !in.contains(i - 1) &&
              !(in.contains(0) && i == lastIdx) &&
              !(in.contains(lastIdx) && i == 0) &&
              in.forall(_ < i)
          }.map(in + _).toSet
        }

        @tailrec
        def loop(todo: Set[Set[Int]], rsf: Set[Set[Int]]): Set[Set[Int]] =
          if (todo.isEmpty) rsf
          else {
            val nexts = getNext(todo.head)
            if (nexts.isEmpty)
              loop(todo.tail, rsf + todo.head)
            else
              loop(nexts ++ todo.tail, rsf)
          }

        loop(Set(Set()), Set()).map(getSum).max
      }
    }
  }

  "Example 1: [2,3,2] -> 3" in {
    val nums = Array(2, 3, 2)
    Solution.rob(nums) shouldBe 3
    // Explanation:
    //   You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
  }
  "Example 2: [1,2,3,1] -> 4" in {
    val nums = Array(1, 2, 3, 1)
    Solution.rob(nums) shouldBe 4
    // Explanation:
    //   Rob house 1 (money = 1) and then rob house 3 (money = 3).
    //   Total amount you can rob = 1 + 3 = 4.
  }
  "Example 3: [0] -> 0" in {
    val nums = Array(0)
    Solution.rob(nums) shouldBe 0
  }

  "Test 46: [1,1,1,1] -> 2" in {
    val nums = Array(1, 1, 1, 1)
    Solution.rob(nums) shouldBe 2
  }
  "Test 52: [2,4,8,9,9,3]" in {
    val nums = Array(2, 4, 8, 9, 9, 3)
    Solution.rob(nums) shouldBe 19
  }
  "Test 53: 40 elements" in {
    val nums = Array(94, 40, 49, 65, 21, 21, 106, 80, 92, 81, 679, 4, 61, 6, 237, 12, 72, 74, 29, 95, 265, 35, 47, 1,
      61, 397, 52, 72, 37, 51, 1, 81, 45, 435, 7, 36, 57, 86, 81, 72)
    Solution.rob(nums) shouldBe 2926
  }
  "Test 62: 48 elements" in {
    val nums = Array(114, 117, 207, 117, 235, 82, 90, 67, 143, 146, 53, 108, 200, 91, 80, 223, 58, 170, 110, 236, 81,
      90, 222, 160, 165, 195, 187, 199, 114, 235, 197, 187, 69, 129, 64, 214, 228, 78, 188, 67, 205, 94, 205, 169, 241,
      202, 144, 240)
    Solution.rob(nums) shouldBe 4077
  }
  "Test 74: 100 elements" in {
    val nums = Array(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    Solution.rob(nums) shouldBe 0
  }

  "My test: [1] -> 1" in {
    val nums = Array(1)
    Solution.rob(nums) shouldBe 1
  }
  "My test: [1,2] -> 2" in {
    val nums = Array(1, 2)
    Solution.rob(nums) shouldBe 2
  }
  "My test: [1,2,3] -> 3" in {
    val nums = Array(1, 2, 3)
    Solution.rob(nums) shouldBe 3
  }
  "My test: [1,2,3,4] -> 6" in {
    val nums = Array(1, 2, 3, 4)
    Solution.rob(nums) shouldBe 6
  }
  "My test: [1,2,3,4,5] -> 8" in {
    val nums = Array(1, 2, 3, 4, 5)
    Solution.rob(nums) shouldBe 8
  }
  "My test: 15 elements" in {
    val length = 15
    val nums = Array.ofDim[Int](length)
    for (i <- 0 until length) {
      nums(i) = i + 1
    }
    Solution.rob(nums) shouldBe 63
  }
  "My test: 30 elements" in {
    val length = 30
    val nums = Array.ofDim[Int](length)
    for (i <- 0 until length) {
      nums(i) = i + 1
    }
    Solution.rob(nums) shouldBe 240
  }
  "My test: Max size" in {
    val length = 100
    val nums = Array.ofDim[Int](length)
    for (i <- 0 until length) {
      nums(i) = i + 1
    }
    Solution.rob(nums) shouldBe 2550
  }
}
