package challenge.c2022.c2022_03

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/problems/linked-list-cycle/]] */
//noinspection DuplicatedCode
class c2022_03_08 extends AnyWordSpec with Matchers {
  /**
   * =141. Linked List Cycle=
   *
   * Given `head`, the head of a linked list, determine if the linked list has a cycle in it.
   *
   * There is a cycle in a linked list if there is some node in the list that can be reached again
   * by continuously following the `next` pointer.
   * Internally, `pos` is used to denote the index of the node that tail's `next` pointer is connected to.
   * '''Note that''' `pos` '''is not passed as a parameter.'''
   *
   * Return `true` ''if there is a cycle in the linked list''. Otherwise, return `false`.
   *
   * '''Constraints:'''
   *  - The number of the nodes in the list is in the range `[0, 10_000]`.
   *  - `-100_000 <= Node.val <= 100_000`
   *  - `pos` is `-1` or a '''valid index''' in the linked-list.
   *
   *
   * '''Follow up:''' Can you solve it using `O(1)` (i.e. constant) memory?
   */
  object Solution {
    def hasCycle(head: ListNode): Boolean = {
      var slow = head
      var fast = head
      while (fast != null && fast.next != null) {
        slow = slow.next
        fast = fast.next.next
        if (slow == fast) {
          return true
        }
      }
      false
    }
  }

  class ListNode(var _x: Int = 0) {
    //noinspection ConvertNullInitializerToUnderscore
    var next: ListNode = null
    var x: Int = _x
  }

  private def l(xs: Int*): ListNode = {
    var head: ListNode = null
    for (x <- xs.reverseIterator) {
      val n = new ListNode(x)
      n.next = head
      head = n
    }
    head
  }

  "[3,2,0,-4], pos = 1" in {
    val h = l(3, 2, 0, -4)
    h.next.next.next.next = h.next
    Solution.hasCycle(h) shouldBe true
    // Explanation: There is a cycle in the linked list, where the tail connects to the 1st node (0-indexed).
  }
  "[1,2], pos = 0" in {
    val h = l(1, 2)
    h.next.next = h
    Solution.hasCycle(h) shouldBe true
    // Explanation: There is a cycle in the linked list, where the tail connects to the 0th node.
  }
  "[1], pos = -1" in {
    Solution.hasCycle(l(1)) shouldBe false
    // Explanation: There is no cycle in the linked list.
  }
  "[], pos = -1" in {
    Solution.hasCycle(null) shouldBe false
  }
}
