package challenge.c2022.c2022_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/problems/linked-list-cycle-ii/]] */
//noinspection ConvertNullInitializerToUnderscore, DuplicatedCode
class c2022_01_19 extends AnyWordSpec with Matchers {
  /**
   * = 142. Linked List Cycle II =
   *
   * Given the `head` of a linked list, return
   * ''the node where the cycle begins. If there is no cycle, return ''`null`.
   *
   * There is a cycle in a linked list if there is some node in the list that can be reached again
   * by continuously following the `next` pointer.
   * Internally, `pos` is used to denote the index of the node
   * that tail's `next` pointer is connected to ('''0-indexed''').
   * It is `-1` if there is no cycle.
   * '''Note that '''`pos`''' is not passed as a parameter'''.
   *
   * '''Do not modify''' the linked list.
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
    def detectCycle(head: ListNode): ListNode = {
      @scala.annotation.tailrec
      def tailRec(head: ListNode, seen: Set[ListNode]): ListNode =
        if (head == null) null
        else if (seen.contains(head)) head
        else tailRec(head.next, seen + head)

      tailRec(head, Set())
    }

    def detectCycle_Set_On_memory(head: ListNode): ListNode = {
      @scala.annotation.tailrec
      def tailRec(head: ListNode, seen: Set[ListNode]): ListNode =
        if (head == null) null
        else if (seen.contains(head)) head
        else tailRec(head.next, seen + head)

      tailRec(head, Set())
    }
  }

  class ListNode(var _x: Int = 0) {
    var next: ListNode = null
    var x: Int = _x
  }

  private def l(x: Int, n: ListNode = null): ListNode = {
    val result = new ListNode(x)
    result.next = n
    result
  }

  private def c(xs: Int*): ListNode = {
    val head = l(xs.head)
    var curr = head
    for (x <- xs.tail) {
      curr.next = l(x)
      curr = curr.next
    }
    curr.next = head
    head
  }

  "head = [3,2,0,-4], pos = 1" in {
    val head = l(3, c(2, 0, 4))
    Solution.detectCycle(head) shouldBe head.next
    // Explanation: There is a cycle in the linked list, where tail connects to the second node.
  }
  "head = [1,2], pos = 0" in {
    val head = c(1, 2)
    Solution.detectCycle(head) shouldBe head
    // Explanation: There is a cycle in the linked list, where tail connects to the first node.
  }
  "head = [1], pos = 0" in {
    val head = c(1)
    Solution.detectCycle(head) shouldBe head
  }
  "head = [1], pos = -1" in {
    Solution.detectCycle(l(1)) shouldBe null
    // Explanation: There is no cycle in the linked list.
  }
}
