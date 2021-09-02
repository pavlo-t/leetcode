package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/568/week-5-november-29th-november-30th/3548/]]
 */
//noinspection DuplicatedCode
class d2020_11_29 extends AnyWordSpec with Matchers {

  /**
   * === Jump Game III ===
   *
   * Given an array of non-negative integers `arr`, you are initially positioned at `start` index of the array.
   * When you are at index `i`, you can jump to `i + arr[i]` or `i - arr[i]`,
   * check if you can reach to '''any''' index with value 0.
   *
   * Notice that you can not jump outside of the array at any time.
   *
   * '''Constraints:'''
   *  - `1 <= arr.length <= 50_000`
   *  - `0 <= arr[i] < arr.length`
   *  - `0 <= start < arr.length`
   */
  object Solution {
    def canReach(arr: Array[Int], start: Int): Boolean = {
      val seen = collection.mutable.Set[Int]()
      val todo = collection.mutable.Queue[Int](start)

      @scala.annotation.tailrec
      def search(): Boolean =
        if (todo.isEmpty) false
        else {
          val i = todo.dequeue()
          if (i < 0 || i >= arr.length || seen.contains(i)) search()
          else if (arr(i) == 0) true
          else {
            seen.add(i)
            todo.enqueue(i + arr(i))
            todo.enqueue(i - arr(i))
            search()
          }
        }

      search()
    }
  }
  object SolutionMutSeen {
    def canReach(arr: Array[Int], start: Int): Boolean = {
      val seen = collection.mutable.Set[Int]()

      @scala.annotation.tailrec
      def search(todo: List[Int]): Boolean =
        todo match {
          case Nil                                                     => false
          case i :: is if i < 0 || i >= arr.length || seen.contains(i) => search(is)
          case i :: _ if arr(i) == 0                                   => true
          case i :: is                                                 =>
            seen.add(i)
            search(i + arr(i) :: i - arr(i) :: is)
        }

      search(List(start))
    }
  }
  object SolutionImm {
    def canReach(arr: Array[Int], start: Int): Boolean = {
      @scala.annotation.tailrec
      def search(todo: List[Int], seen: Set[Int]): Boolean = {
        todo match {
          case Nil                                                     => false
          case i :: is if i < 0 || i >= arr.length || seen.contains(i) => search(is, seen)
          case i :: is if arr(i) != 0                                  => search(i + arr(i) :: i - arr(i) :: is, seen + i)
          case _                                                       => true
        }
      }

      search(List(start), Set())
    }
  }
  object SolutionBruteForce {
    def canReach(arr: Array[Int], start: Int): Boolean = {
      import collection.mutable

      //Thread.sleep(30)
      //println(s"canReach(${arr.take(30).mkString("[", ",", "]")},start=$start)")

      val seen = mutable.Set[Int]()

      def dfs(i: Int): Boolean = {
        //println(s"  dfs($i) seen=${seen.take(30).mkString("[", ",", "]")}")
        if (i < 0 || i >= arr.length || seen.contains(i)) false
        else if (arr(i) == 0) true
        else {
          seen.add(i)
          dfs(i + arr(i)) || dfs(i - arr(i))
        }
      }

      dfs(start)
    }
  }

  import Solution.canReach

  "Example 1: (arr = [4,2,3,0,3,1,2], start = 5) -> true" in {
    canReach(Array(4, 2, 3, 0, 3, 1, 2), 5) shouldBe true
    //Explanation:
    //All possible ways to reach at index 3 with value 0 are:
    //  index 5 -> index 4 -> index 1 -> index 3
    //  index 5 -> index 6 -> index 4 -> index 1 -> index 3
  }
  "Example 2: (arr = [4,2,3,0,3,1,2], start = 0) -> true" in {
    canReach(Array(4, 2, 3, 0, 3, 1, 2), 0) shouldBe true
    //Explanation:
    //One possible way to reach at index 3 with value 0 is:
    //  index 0 -> index 4 -> index 1 -> index 3
  }
  "Example 3: (arr = [3,0,2,1,2], start = 2) -> false" in {
    canReach(Array(3, 0, 2, 1, 2), 2) shouldBe false
    //Explanation: There is no way to reach at index 1 with value 0.
  }

  "(arr = [1,1,1..1] with len = 50000, start = 49999) -> false" in {
    canReach(Array.fill(50000)(1), 49999) shouldBe false
  }
  "(arr = [0,1,1..1,1] with len = 50000, start = 49999) -> true" in {
    val arr = Array.fill(50000)(1)
    arr(0) = 0
    canReach(arr, 49999) shouldBe true
  }
  "(arr = [1,1..1,1,0] with len = 50000, start = 0) -> true" in {
    val arr = Array.fill(50000)(1)
    arr(49999) = 0
    canReach(arr, 0) shouldBe true
  }

  "(arr = [50000 random numbers], start = random) -> ???" in {
    canReach(Array.fill(50000)(util.Random.nextInt(50000)), util.Random.nextInt(50000))
  }

}
