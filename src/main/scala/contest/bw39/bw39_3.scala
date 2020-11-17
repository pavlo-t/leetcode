package contest.bw39

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/contest/biweekly-contest-39/problems/minimum-jumps-to-reach-home/]]
 */
//noinspection DuplicatedCode,NameBooleanParameters
class bw39_3 extends AnyWordSpec with Matchers {
  /**
   * === 1654. Minimum Jumps to Reach Home ===
   *
   * A certain bug's home is on the x-axis at position `x`.
   * Help them get there from position `0`.
   *
   * The bug jumps according to the following rules:
   *  - It can jump exactly `a` positions '''forward''' (to the right).
   *  - It can jump exactly `b` positions '''backward''' (to the left).
   *  - It cannot jump backward twice in a row.
   *  - It cannot jump to any `forbidden` positions.
   *
   * The bug may jump forward '''beyond''' its home,
   * but it '''cannot jump''' to positions numbered with '''negative''' integers.
   *
   * Given an array of integers `forbidden`,
   * where `forbidden[i]` means that the bug cannot jump to the position `forbidden[i]`,
   * and integers `a`, `b`, and `x`,
   * return ''the minimum number of jumps needed for the bug to reach its home''.
   * If there is no possible sequence of jumps that lands the bug on position `x`, return `-1`.
   *
   * '''Constraints:'''
   *  - `1 <= forbidden.length <= 1000`
   *  - `1 <= a, b, forbidden[i] <= 2000`
   *  - `0 <= x <= 2000`
   *  - All the elements in `forbidden` are distinct.
   *  - Position `x` is not forbidden.
   */
  object Solution {
    import collection.mutable

    def minimumJumps(forbidden: Array[Int], a: Int, b: Int, x: Int): Int = {
      val max = forbidden.maxOption.getOrElse(0).max(x) + a + b

      val todo = mutable.Queue((0, 0))
      val fs = mutable.Set(forbidden: _*)
      fs.addOne(0)

      def isValid(i: Int): Boolean = i > 0 && i <= max && !fs.contains(i)

      @scala.annotation.tailrec
      def findWay(): Int = {
        if (todo.isEmpty) -1
        else {
          val (i, s) = todo.dequeue

          if (i == x) s
          else if (i + a == x || i - b == x) s + 1
          else {
            if (isValid(i + a)) todo.enqueue((i + a, s + 1))
            if (isValid(i - b) && isValid(i - b + a)) todo.enqueue((i - b + a, s + 2))
            fs += i

            findWay()
          }
        }
      }

      findWay()
    }
  }
  object SolutionTailrecImmutableQueue {
    import collection.immutable.Queue

    def minimumJumps(forbidden: Array[Int], a: Int, b: Int, x: Int): Int = {
      val max = forbidden.maxOption.getOrElse(0).max(x) + a + b

      @scala.annotation.tailrec
      def findWay(todo: Queue[(Int, Int)], fs: Set[Int]): Int = {
        if (todo.isEmpty) -1
        else {
          val ((i, s), rest) = todo.dequeue

          if (i == x) s
          else if (i + a == x || i - b == x) s + 1
          else {
            def isValid(i: Int): Boolean = i > 0 && i <= max && !fs.contains(i)

            val nexts =
              Seq(
                (i + a, s + 1),
                (if (isValid(i - b)) i - b + a else -1, s + 2)
              ).filter(nt => isValid(nt._1))

            findWay(rest.enqueueAll(nexts), fs + i)
          }
        }
      }

      findWay(Queue((0, 0)), forbidden.toSet + 0)
    }
  }
  object SolutionImperative {
    import collection.mutable

    def minimumJumps(forbidden: Array[Int], a: Int, b: Int, t: Int): Int = {
      if (t == 0) return 0

      val threshold = forbidden.maxOption.getOrElse(0).max(t) + a + b
      val forb = forbidden.toSet
      val seen = mutable.Set(0)
      val q = mutable.Queue((0, 0))


      while (q.nonEmpty) {
        val (pos, steps) = q.dequeue()

        if (!forb.contains(pos + a) && !seen.contains(pos + a) && pos + a <= threshold) {
          // Termination condition
          if (pos + a == t) return steps + 1

          q.enqueue((pos + a, steps + 1))
          seen.addOne(pos + a)
        }

        if (pos - b > 0 && !forb.contains(pos - b) && !seen.contains(pos - b)) {
          // Termination condition
          if (pos - b == t) return steps + 1

          seen.add(pos - b)
          if (!forb.contains(pos - b + a) && !seen.contains(pos - b + a) && pos - b + a <= threshold) {
            // Termination condition
            if (pos - b + a == t) return steps + 2

            q.enqueue((pos - b + a, steps + 2))
            seen.add(pos - b + a)
          }
        }
      }
      -1
    }
  }

  import Solution.minimumJumps

  "Example 1: (forbidden = [14,4,18,1,15], a = 3, b = 15, x = 9) -> 3" in {
    minimumJumps(Array(14, 4, 18, 1, 15), 3, 15, 9) shouldBe 3
    //Explanation: 3 jumps forward (0 -> 3 -> 6 -> 9) will get the bug home.
  }
  "Example 2: (forbidden = [8,3,16,6,12,20], a = 15, b = 13, x = 11) -> -1" in {
    minimumJumps(Array(8, 3, 16, 6, 12, 20), 15, 13, 11) shouldBe -1
  }
  "Example 3: (forbidden = [1,6,2,14,5,17,4], a = 16, b = 9, x = 7) -> 2" in {
    minimumJumps(Array(1, 6, 2, 14, 5, 17, 4), 16, 9, 7) shouldBe 2
    //Explanation: One jump forward (0 -> 16) then one jump backward (16 -> 7) will get the bug home.
  }

  "test 14: (forbidden = [...], a = 29, b = 98, x = 80) -> 121" in {
    val forbidden = Array(162, 118, 178, 152, 167, 100, 40, 74, 199, 186, 26, 73, 200, 127, 30, 124, 193, 84, 184, 36,
      103, 149, 153, 9, 54, 154, 133, 95, 45, 198, 79, 157, 64, 122, 59, 71, 48, 177, 82, 35, 14, 176, 16, 108, 111, 6,
      168, 31, 134, 164, 136, 72, 98)
    minimumJumps(forbidden, 29, 98, 80) shouldBe 121
  }

  "(forbidden = [], a = 1, b = 1, x = 0) -> 0" in {
    minimumJumps(Array(), 1, 1, 0) shouldBe 0
  }
  "(forbidden = [], a = 1, b = 1, x = 1) -> 1" in {
    minimumJumps(Array(), 1, 1, 1) shouldBe 1
  }

  "(forbidden = [], a = 1, b = 1, x = 2000) -> 2000" in {
    minimumJumps(Array(), 1, 1, 2000) shouldBe 2000
  }
  "(forbidden = [], a = 1998, b = 1999, x = 2000) -> 2000" in {
    minimumJumps(Array(), 1998, 1999, 2000) shouldBe 3994
  }
}
