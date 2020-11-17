package contest.bw39

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/contest/biweekly-contest-39/problems/distribute-repeating-integers/]]
 */
//noinspection DuplicatedCode
class bw39_4 extends AnyWordSpec with Matchers {
  /**
   * === 1655. Distribute Repeating Integers ===
   *
   * You are given an array of `n` integers, `nums`, where there are at most `50` unique values in the array.
   * You are also given an array of `m` customer order quantities, `quantity`,
   * where `quantity[i]` is the amount of integers the `i`th customer ordered.
   * Determine if it is possible to distribute `nums` such that:
   *  - The `i`th customer gets '''exactly''' `quantity[i]` integers,
   *  - The integers the `i`th customer gets are '''all equal''', and
   *  - Every customer is satisfied.
   *
   * Return `true` ''if it is possible to distribute ''`nums`'' according to the above conditions''.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 100_000`
   *  - `1 <= nums[i] <= 1000`
   *  - `1 <= quantity.length <= 10`
   *  - `1 <= quantity[i] <= 100_000`
   *  - There are at most `50` unique values in `nums`.
   */
  object Solution {
    import collection.mutable

    def canDistribute(nums: Array[Int], quantity: Array[Int]): Boolean = {
      val counts = mutable.Map[Int, Int]().withDefaultValue(0)
      for (i <- nums) counts(i) += 1

      def canI(qs: Seq[Int], cts: Seq[Int]): Boolean =
        if (qs.isEmpty) true
        else {
          val ci = cts.indexWhere(_ >= qs.head)
          if (ci == -1) false
          else (ci until cts.size).exists(i => canI(qs.tail, cts.updated(i, cts(i) - qs.head).sorted))
        }

      canI(quantity.toSeq.sorted(Ordering[Int].reverse), counts.values.toSeq.sorted)
    }
  }
  object SolutionTailrecTodoFailing {
    import collection.mutable

    def canDistribute(nums: Array[Int], quantity: Array[Int]): Boolean = {
      val counts = mutable.Map[Int, Int]().withDefaultValue(0)
      for (i <- nums) counts(i) += 1

      @scala.annotation.tailrec
      def canI(todo: Seq[(Seq[Int], Seq[Int])]): Boolean = todo match {
        case Nil                                => false
        case (Nil, _) +: _                      => true
        case (q +: restQs, counts) +: restTodos =>
          val ci = counts.indexWhere(_ >= q)
          if (ci == -1) canI(restTodos)
          else {
            val nextTodos = (ci until counts.size).map(i => (restQs, counts.updated(i, counts(i) - q).sorted))
            canI(nextTodos ++ restTodos)
          }
      }

      canI(Seq((quantity.toSeq, counts.values.toList.sorted)))
    }
  }

  import Solution.canDistribute

  "Example 1: (nums = [1,2,3,4], quantity = [2]) -> false" in {
    canDistribute(Array(1, 2, 3, 4), Array(2)) shouldBe false
    //Explanation: The 0th customer cannot be given two different integers.
  }
  "Example 2: (nums = [1,2,3,3], quantity = [2]) -> true" in {
    canDistribute(Array(1, 2, 3, 3), Array(2)) shouldBe true
    //Explanation: The 0th customer is given [3,3]. The integers [1,2] are not used.
  }
  "Example 3: (nums = [1,1,2,2], quantity = [2,2]) -> true" in {
    canDistribute(Array(1, 1, 2, 2), Array(2, 2)) shouldBe true
    //Explanation: The 0th customer is given [1,1], and the 1st customer is given [2,2].
  }
  "Example 4: (nums = [1,1,2,3], quantity = [2,2]) -> false" in {
    canDistribute(Array(1, 1, 2, 3), Array(2, 2)) shouldBe false
    //Explanation: Although the 0th customer could be given [1,1], the 1st customer cannot be satisfied.
  }
  "Example 5: (nums = [1,1,1,1,1], quantity = [2,3]) -> true" in {
    canDistribute(Array(1, 1, 1, 1, 1), Array(2, 3)) shouldBe true
    //Explanation: The 0th customer is given [1,1], and the 1st customer is given [1,1,1].
  }

  "Test 16: (nums = [1..50 times 2], quantity = [2,2,2,2,2,2,2,2,2,3]) -> false" in {
    val nums = Array(1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14,
      15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 26, 26, 27, 27, 28, 28,
      29, 29, 30, 30, 31, 31, 32, 32, 33, 33, 34, 34, 35, 35, 36, 36, 37, 37, 38, 38, 39, 39, 40, 40, 41, 41, 42, 42,
      43, 43, 44, 44, 45, 45, 46, 46, 47, 47, 48, 48, 49, 49, 50, 50)
    val quantity = Array(2, 2, 2, 2, 2, 2, 2, 2, 2, 3)
    canDistribute(nums, quantity) shouldBe false
  }
  "Test 97: (nums = [1,1,1,1,2,2,2], quantity = [3,2,2]) -> true" in {
    canDistribute(Array(1, 1, 1, 1, 2, 2, 2), Array(3, 2, 2)) shouldBe true
  }
  "Test 98: (nums = [1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2], quantity = [4,3,4,3,3]) -> true" in {
    val nums = Array(1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2)
    val quantity = Array(4, 3, 4, 3, 3)
    canDistribute(nums, quantity) shouldBe true
  }
  "Test 99: (nums = [154,533,533,533,154,154,533,154,154], quantity = [3,2,2,2]) -> true" in {
    val nums = Array(154, 533, 533, 533, 154, 154, 533, 154, 154)
    val quantity = Array(3, 2, 2, 2)
    canDistribute(nums, quantity) shouldBe true
  }

  "(nums = [1 times 100_000], quantity = [100_000]) -> true" in {
    canDistribute(Array.fill(100_000)(1), Array(100_000)) shouldBe true
  }
  "(nums = [1 times 100_000], quantity = [1,100_000]) -> false" in {
    canDistribute(Array.fill(100_000)(1), Array(1, 100_000)) shouldBe false
  }
  "(nums = [1..50 up to length 100_000], quantity = [2000 repeated 10]) -> true" in {
    val nums = (1 to 100_000).map(_ % 50 + 1).toArray
    val quantity = Array.fill(10)(2000)
    canDistribute(nums, quantity) shouldBe true
  }
  "(nums = [100_000 times Random.nextInt(50) + 1], quantity = [10 times Random.nextInt(100_000) + 1]) -> ???" in {
    val nums = (1 to 100_000).map(_ => util.Random.nextInt(50) + 1).toArray
    val quantity = Array.fill(10)(util.Random.nextInt(100_000) + 1)

    val result = canDistribute(nums, quantity)

    println(s"canDistribute(${nums.length}, ${quantity.mkString("quantity: [", ",", "]")}) -> $result")
  }
}
