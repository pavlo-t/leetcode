package contest.w214

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class w214_4 extends AnyWordSpec with Matchers {

  /**
   * <h3>5564. Create Sorted Array through Instructions</h3>
   *
   * Given an integer array `instructions`, you are asked to create a sorted array from the elements in `instructions`.
   * You start with an empty container `nums`.
   * For each element from <b>left to right</b> in `instructions`, insert it into `nums`.
   * The <b>cost</b> of each insertion is the <b>minimum</b> of the following:<ul>
   * <li> The number of elements currently in `nums` that are <b>strictly less than</b> `instructions[i]`.
   * <li> The number of elements currently in `nums` that are <b>strictly greater than</b> `instructions[i]`.
   * </ul>
   *
   * <p>For example, if inserting element `3` into `nums = [1,2,3,5]`, the <b>cost</b> of insertion is `min(2, 1)`
   * (elements `1` and `2` are less than `3`, element `5` is greater than `3`) and `nums` will become `[1,2,3,3,5]`.
   *
   * <p>Return <em>the <b>total cost</b> to insert all elements from `instructions` into `nums`</em>.
   * Since the answer may be large, return it <b>modulo</b> `1_000_000_007`
   *
   * <p><b>Constraints:</b><ul>
   * <li> `1 <= instructions.length <= 100_000`
   * <li> `1 <= instructions[i] <= 100_000`
   * </ul>
   */
  object Solution {
    val Modulo = 1_000_000_007
    def createSortedArray(instructions: Array[Int]): Int = {
      instructions
        .zipWithIndex
        .foldLeft((0, new BinaryIndexedTree(100_000))) { case ((acc, bit), (v, i)) =>
          ((acc + (bit.sum(v - 1) min (i - bit.sum(v)))) % Modulo, bit.add(v))
        }._1
    }

    /**
     * Binary Indexed Tree aka Fenwick Tree
     *
     * @see [[https://en.wikipedia.org/wiki/Fenwick_tree]]
     * @see [[https://www.hackerearth.com/practice/notes/binary-indexed-tree-or-fenwick-tree/]]
     */
    class BinaryIndexedTree(size: Int) {
      private val A = Array.ofDim[Int](size + 1)
      private def lsb(i: Int): Int = i & -i

      @scala.annotation.tailrec
      final def sum(i: Int, rsf: Int = 0): Int =
        if (i > 0) sum(i - lsb(i), rsf + A(i))
        else rsf

      @scala.annotation.tailrec
      final def add(i: Int): BinaryIndexedTree =
        if (i <= size) {
          A(i) += 1
          add(i + lsb(i))
        } else this
    }
  }
  object SolutionMyBIT {
    val Modulo = 1_000_000_007
    def createSortedArray(instructions: Array[Int]): Int = {
      def lsb(i: Int): Int = i & -i

      val size = 100_000
      val bit = Array.ofDim[Int](size + 1)

      @scala.annotation.tailrec
      def sum(i: Int, rsf: Int = 0): Int =
        if (i > 0) sum(i - lsb(i), rsf + bit(i))
        else rsf

      @scala.annotation.tailrec
      def add(i: Int): Unit =
        if (i <= size) {
          bit(i) += 1
          add(i + lsb(i))
        }

      instructions
        .indices
        .fold(0) { (acc, i) =>
          val v = instructions(i)
          val newRes = (acc + (sum(v - 1) min (i - sum(v)))) % Modulo
          add(v)
          newRes
        }
    }
  }
  /**
   * [[https://leetcode.com/problems/create-sorted-array-through-instructions/discuss/927531/JavaC%2B%2BPython-Binary-Indexed-Tree]]
   */
  object Solution1 {
    val Modulo = 1_000_000_007
    def createSortedArray(instructions: Array[Int]): Int = {
      val max = 100_000
      val c = Array.ofDim[Int](max + 1)

      def update(x: Int): Unit = {
        var i = x
        while (i <= max) {
          c(i) += 1
          i += i & -i
        }
      }

      def get(x: Int): Int = {
        var i = x
        var res = 0
        while (i > 0) {
          res += c(i)
          i -= i & -i
        }
        res
      }

      var res = 0
      for (i <- instructions.indices) {
        res += Math.min(get(instructions(i) - 1), i - get(instructions(i)))
        res %= Modulo
        update(instructions(i))
      }
      res
    }

  }
  object SolutionMyWithCache {
    val Modulo = 1_000_000_007
    def createSortedArray(instructions: Array[Int]): Int = {
      var cost = 0
      val cache = collection.mutable.Map[Int, Int](instructions.head -> 1).withDefaultValue(0)
      for (i <- instructions.tail) {
        val (costL, costR) = cache.foldLeft((0, 0)) { case (acc@(costL, costR), k -> count) =>
          if (k < i) (costL + count, costR)
          else if (k > i) (costL, costR + count)
          else acc
        }
        cost += (costL min costR) % Modulo
        cache(i) += 1
      }
      cost
    }
  }
  object SolutionMyBruteForce {
    val Modulo = 1_000_000_007
    def createSortedArray(instructions: Array[Int]): Int = {
      var cost = 0
      for (i <- 1 until instructions.length) {
        var costL = 0
        var costR = 0
        for (j <- i to 0 by -1) {
          if (instructions(j) < instructions(i)) costL += 1
          else if (instructions(j) > instructions(i)) costR += 1
        }
        cost += (costL min costR) % Modulo
      }
      cost
    }
  }

  import Solution.createSortedArray

  "Example 1: (instructions = [1,5,6,2]) -> 1" in {
    createSortedArray(Array(1, 5, 6, 2)) shouldBe 1
    //Explanation: Begin with nums = [].
    //Insert 1 with cost min(0, 0) = 0, now nums = [1].
    //Insert 5 with cost min(1, 0) = 0, now nums = [1,5].
    //Insert 6 with cost min(2, 0) = 0, now nums = [1,5,6].
    //Insert 2 with cost min(1, 2) = 1, now nums = [1,2,5,6].
    //The total cost is 0 + 0 + 0 + 1 = 1.
  }
  "Example 2: (instructions = [1,2,3,6,5,4]) -> 3" in {
    createSortedArray(Array(1, 2, 3, 6, 5, 4)) shouldBe 3
    //Explanation: Begin with nums = [].
    //Insert 1 with cost min(0, 0) = 0, now nums = [1].
    //Insert 2 with cost min(1, 0) = 0, now nums = [1,2].
    //Insert 3 with cost min(2, 0) = 0, now nums = [1,2,3].
    //Insert 6 with cost min(3, 0) = 0, now nums = [1,2,3,6].
    //Insert 5 with cost min(3, 1) = 1, now nums = [1,2,3,5,6].
    //Insert 4 with cost min(3, 2) = 2, now nums = [1,2,3,4,5,6].
    //The total cost is 0 + 0 + 0 + 0 + 1 + 2 = 3.
  }
  "Example 3: (instructions = [1,3,3,3,2,4,2,1,2]) -> 4" in {
    createSortedArray(Array(1, 3, 3, 3, 2, 4, 2, 1, 2)) shouldBe 4
    //Explanation: Begin with nums = [].
    //Insert 1 with cost min(0, 0) = 0, now nums = [1].
    //Insert 3 with cost min(1, 0) = 0, now nums = [1,3].
    //Insert 3 with cost min(1, 0) = 0, now nums = [1,3,3].
    //Insert 3 with cost min(1, 0) = 0, now nums = [1,3,3,3].
    //Insert 2 with cost min(1, 3) = 1, now nums = [1,2,3,3,3].
    //Insert 4 with cost min(5, 0) = 0, now nums = [1,2,3,3,3,4].
    //Insert 2 with cost min(1, 4) = 1, now nums = [1,2,2,3,3,3,4].
    //Insert 1 with cost min(0, 6) = 0, now nums = [1,1,2,2,3,3,3,4].
    //Insert 2 with cost min(2, 4) = 2, now nums = [1,1,2,2,2,3,3,3,4].
    //The total cost is 0 + 0 + 0 + 0 + 1 + 0 + 1 + 0 + 2 = 4.
  }

  "(instructions = [1]) -> 0" in {
    createSortedArray(Array(1)) shouldBe 0
  }
  "(instructions = [1 repeat 100_000]) -> 0" in {
    val instructions = Array.fill(100_000)(1)
    createSortedArray(instructions) shouldBe 0
  }
  "(instructions = [random repeat 100_000]) -> >= 0" in {
    val instructions = Array.fill(100_000)(util.Random.nextInt(100_000) + 1)
    createSortedArray(instructions) should be >= 0
  }
}
