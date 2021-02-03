package challenge.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3627/]]
 */
//noinspection ConvertNullInitializerToUnderscore
class c2021_02_03 extends AnyWordSpec with Matchers {
  /**
   * === Linked List Cycle ===
   *
   * Given head, the head of a linked list, determine if the linked list has a cycle in it.
   *
   * There is a cycle in a linked list if there is some node in the list that can be reached again
   * by continuously following the `next` pointer.
   * Internally, `pos` is used to denote the index of the node that tail's `next` pointer is connected to.
   * '''Note that''' `pos` '''is not passed as a parameter.'''
   *
   * Return `true` ''if there is a cycle in the linked list''.
   * Otherwise, return `false`.
   *
   * '''Constraints:'''
   *  - The number of the nodes in the list is in the range `[0, 10_000]`.
   *  - `-100_000 <= Node.val <= 100_000`
   *  - `pos` is `-1` or a '''valid index''' in the linked-list.
   *
   * '''Follow up:''' Can you solve it using `O(1)` (i.e. constant) memory?
   */
  object Solution {
    def hasCycle(head: ListNode): Boolean =
      if (head == null) false
      else {
        @scala.annotation.tailrec
        def rec(l: ListNode, r: ListNode): Boolean = {
          if (l == r) true
          else if (r == null || r.next == null) false
          else rec(l.next, r.next.next)
        }
        rec(head, head.next)
      }
  }

  object SolutionMy {
    def hasCycle(head: ListNode): Boolean = {
      @scala.annotation.tailrec
      def rec(n: ListNode, seen: Set[ListNode]): Boolean =
        if (n == null) false
        else if (seen.contains(n)) true
        else rec(n.next, seen + n)
      rec(head, Set())
    }
  }

  class ListNode(var _x: Int = 0) {
    var next: ListNode = null
    var x: Int = _x
  }

  import Solution.hasCycle

  private def l(x: Int, n: ListNode = null): ListNode = {
    val r = new ListNode(x)
    r.next = n
    r
  }

  "Example 1: (head = [3,2,0,-4], pos = 1) -> true" in {
    val ch = l(2)
    ch.next = l(0, l(-4, ch))
    val head = l(3, ch)
    hasCycle(head) shouldBe true
    //Explanation: There is a cycle in the linked list, where the tail connects to the 1st node (0-indexed).
  }
  "Example 2: (head = [1,2], pos = 0) -> true" in {
    val head = l(1)
    head.next = l(2, head)
    hasCycle(head) shouldBe true
    //Explanation: There is a cycle in the linked list, where the tail connects to the 0th node.
  }
  "Example 3: (head = [1], pos = -1) -> false" in {
    val head = l(1)
    hasCycle(head) shouldBe false
    //Explanation: There is no cycle in the linked list.
  }

}
