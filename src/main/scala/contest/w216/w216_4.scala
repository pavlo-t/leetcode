package contest.w216

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


/**
 * [[https://leetcode.com/contest/weekly-contest-216/problems/minimum-initial-energy-to-finish-tasks/]]
 */
//noinspection DuplicatedCode
class w216_4 extends AnyWordSpec with Matchers {

  /**
   * === 1665. Minimum Initial Energy to Finish Tasks ===
   *
   * You are given an array `tasks` where `tasks[i] = [actual_i, minimum_i]`:
   *  - `actual_i` is the actual amount of energy you '''spend to finish''' the `i`th task.
   *  - `minimum_i` is the minimum amount of energy you '''require to begin''' the `i`th task.
   *
   * For example, if the task is `[10, 12]` and your current energy is `11`, you cannot start this task.
   * However, if your current energy is `13`, you can complete this task, and your energy will be `3` after finishing it.
   *
   * You can finish the tasks in '''any order''' you like.
   *
   * Return ''the '''minimum''' initial amount of energy you will need to finish all the tasks''.
   *
   * ''Constraints:''
   *  - `1 <= tasks.length <= 100_000`
   *  - `1 <= actual_i <= minimum_i <= 10_000`
   */
  object Solution {
    def minimumEffort(tasks: Array[Array[Int]]): Int = {
      var totalEnergy = 0

      for (Array(actual, initial) <- tasks.sortInPlaceBy(t => t(1) - t(0))) {
        totalEnergy += actual
        if (totalEnergy < initial)
          totalEnergy = initial
      }

      totalEnergy
    }
  }

  import Solution.minimumEffort

  "Example 1: (tasks = [[1,2],[2,4],[4,8]]) -> 8" in {
    val tasks = Array(Array(1, 2), Array(2, 4), Array(4, 8))
    minimumEffort(tasks) shouldBe 8
    //Explanation:
    //Starting with 8 energy, we finish the tasks in the following order:
    //    - 3rd task. Now energy = 8 - 4 = 4.
    //    - 2nd task. Now energy = 4 - 2 = 2.
    //    - 1st task. Now energy = 2 - 1 = 1.
    //Notice that even though we have leftover energy, starting with 7 energy does not work because we cannot do the 3rd task.
  }
  "Example 2: (tasks = [[1,3],[2,4],[10,11],[10,12],[8,9]]) -> 32" in {
    val tasks = Array(Array(1, 3), Array(2, 4), Array(10, 11), Array(10, 12), Array(8, 9))
    minimumEffort(tasks) shouldBe 32
    //Explanation:
    //Starting with 32 energy, we finish the tasks in the following order:
    //    - 1st task. Now energy = 32 - 1 = 31.
    //    - 2nd task. Now energy = 31 - 2 = 29.
    //    - 3rd task. Now energy = 29 - 10 = 19.
    //    - 4th task. Now energy = 19 - 10 = 9.
    //    - 5th task. Now energy = 9 - 8 = 1.
  }
  "Example 3: (tasks = [[1,7],[2,8],[3,9],[4,10],[5,11],[6,12]]) -> 27" in {
    val tasks = Array(Array(1, 7), Array(2, 8), Array(3, 9), Array(4, 10), Array(5, 11), Array(6, 12))
    minimumEffort(tasks) shouldBe 27
    //Explanation:
    //Starting with 27 energy, we finish the tasks in the following order:
    //    - 5th task. Now energy = 27 - 5 = 22.
    //    - 2nd task. Now energy = 22 - 2 = 20.
    //    - 3rd task. Now energy = 20 - 3 = 17.
    //    - 1st task. Now energy = 17 - 1 = 16.
    //    - 4th task. Now energy = 16 - 4 = 12.
    //    - 6th task. Now energy = 12 - 6 = 6.
  }
}
