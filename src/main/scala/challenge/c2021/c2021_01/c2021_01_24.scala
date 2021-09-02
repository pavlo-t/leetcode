package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3615/]]
 */
//noinspection DuplicatedCode
class c2021_01_24 extends AnyWordSpec with Matchers {
  /**
   * === Merge k Sorted Lists ===
   *
   * You are given an array of `k` linked-lists `lists`, each linked-list is sorted in ascending order.
   *
   * '''Merge all the linked-lists into one sorted linked-list and return it'''.
   *
   * '''Constraints:'''
   *  - `0 <= lists.length <= 10_000`
   *  - `0 <= lists[i].length <= 500`
   *  - `-10_000 <= lists[i][j] <= 10_000`
   *  - `lists[i]` is sorted in '''ascending order'''.
   *  - The sum of `lists[i].length` won't exceed `10_000`.
   */
  object Solution {
    def mergeKLists(lists: Array[ListNode]): ListNode = {
      val root = new ListNode()

      @scala.annotation.tailrec
      def rec(ls: Seq[ListNode], curr: ListNode): Unit =
        if (ls.nonEmpty) {
          val i = ls.indices.foldLeft(0)((r, i) => if (ls(i).x < ls(r).x) i else r)
          curr.next = new ListNode(ls(i).x)
          if (ls(i).next == null) rec(ls.patch(i, Nil, 1), curr.next)
          else rec(ls.updated(i, ls(i).next), curr.next)
        }

      rec(lists.filter(_ != null).toSeq, root)
      root.next
    }
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    //override def toString: String = s"$x,$next"
    override def toString: String = {
      val lb = collection.mutable.ListBuffer[Int]()
      var curr = this
      while (curr != null) {
        lb.addOne(curr.x)
        curr = curr.next
      }
      lb.mkString("[", ",", "]")
    }
  }

  import Solution.mergeKLists

  private def l(xs: Int*): ListNode = {
    val root = new ListNode(0)
    var curr = root
    for (x <- xs) {
      val n = new ListNode(x)
      curr.next = n
      curr = n
    }
    root.next
  }

  "Example 1: (lists = [[1,4,5],[1,3,4],[2,6]]) -> [1,1,2,3,4,4,5,6]" in {
    val lists = Array(l(1, 4, 5), l(1, 3, 4), l(2, 6))
    val e = l(1, 1, 2, 3, 4, 4, 5, 6)
    mergeKLists(lists).toString shouldBe e.toString
    //Explanation: The linked-lists are:
    //[ 1->4->5, 1->3->4, 2->6 ]
    //merging them into one sorted list:
    //1->1->2->3->4->4->5->6
  }
  "Example 2: (lists = []) -> []" in {
    mergeKLists(Array()) shouldBe null
  }
  "Example 3: (lists = [[]]) -> []" in {
    mergeKLists(Array(null)) shouldBe null
  }

  "(lists = [[1],...,[10_000]]) -> 1 to 10000" in {
    val lists = (1 to 10000).map(i => l(i)).toArray
    val e = l(1 to 10000: _*)
    mergeKLists(lists).toString shouldBe e.toString
  }
  "(lists = [[10_000],...,[1]]) -> 1 to 10000" in {
    val lists = (10000 to 1 by -1).map(i => l(i)).toArray
    val e = l(1 to 10000: _*)
    mergeKLists(lists).toString shouldBe e.toString
  }
  "(lists = [[1 to 10_000]]) -> 1 to 10000" in {
    val lists = Array(l(1 to 10000: _*))
    val e = l(1 to 10000: _*)
    mergeKLists(lists).toString shouldBe e.toString
  }

}
