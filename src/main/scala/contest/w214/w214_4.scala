package contest.w214

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class w214_4 extends AnyWordSpec with Matchers {

  /**
   * <h3>5564. Create Sorted Array through Instructions</h3>
   */
  object Solution {
    val Modulo = 1_000_000_007
    def createSortedArray(instructions: Array[Int]): Int = {
      var cost = 0
      //val cache = collection.mutable.Map[Int, Int]()
      for (i <- (1 until instructions.length)) {
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

  "(instructions = [random of length 100_000]) -> >= 0" in {
    val instructions = Array.fill(100_000)(util.Random.nextInt(100_000) + 1)
    createSortedArray(instructions) should be >= 0
  }
}
