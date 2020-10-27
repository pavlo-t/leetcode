package c2020_10.w4

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection ConvertNullInitializerToUnderscore,DuplicatedCode
class d2020_10_27 extends AnyWordSpec with Matchers {

  /**
   * <h3>Linked List Cycle II</h3>
   *
   * Given a linked list, return the node where the cycle begins.
   * If there is no cycle, return `null`.
   *
   * There is a cycle in a linked list if there is some node in the list that can be reached again by continuously
   * following the `next` pointer.
   * Internally, `pos` is used to denote the index of the node that tail's `next` pointer is connected to.
   * <b>Note that `pos` is not passed as a parameter</b>.
   *
   * <b>Notice</b> that you <b>should not modify</b> the linked list.
   *
   * <b>Follow up</b>:
   *
   * Can you solve it using `O(1)` (i.e. constant) memory?
   *
   * <b>Constraints:</b><ul>
   * <li> The number of the nodes in the list is in the range `[0, 10_000]`.
   * <li> `-100_000 <= Node.val <= 100_000`
   * <li> `pos` is `-1` or a <b>valid index</b> in the linked-list.
   * </ul>
   */
  object Solution {
    import scala.annotation.tailrec

    def detectCycle(head: ListNode): ListNode = {
      if (head == null || head.next == null) null
      else {
        @tailrec
        def loop(h: ListNode, seen: Seq[ListNode] = Seq()): ListNode = {
          if (h == null) null
          else if (seen.contains(h)) h
          else loop(h.next, h +: seen)
        }

        loop(head)
      }
    }
  }

  class ListNode(var _x: Int = 0) {
    var next: ListNode = null
    var x: Int = _x
  }
  def ln(x: Int, next: ListNode = null): ListNode = {
    val n = new ListNode(x)
    n.next = next
    n
  }

  import Solution.detectCycle

  "Example 1: ([3,2,0,-4]) -> 1" in {
    val cycleHead = ln(2)
    val cycleTail = ln(0, ln(-4, cycleHead))
    cycleHead.next = cycleTail
    val head = ln(3, cycleHead)

    detectCycle(head) shouldBe cycleHead
    // Explanation: There is a cycle in the linked list, where tail connects to the second node.
  }
  "Example 2: ([1,2]) -> 0" in {
    val cycleHead = ln(1)
    val cycleTail = ln(2, cycleHead)
    cycleHead.next = cycleTail
    val head = cycleHead

    detectCycle(head) shouldBe cycleHead
    // Explanation: There is a cycle in the linked list, where tail connects to the first node.
  }
  "Example 3: ([1]) -> null" in {
    val head = ln(1)

    detectCycle(head) shouldBe null
    // Explanation: There is no cycle in the linked list.
  }

  "(10_000 nodes) -> first" in {
    val cycleHead = ln(0)
    var current = cycleHead
    for (i <- 1 to 10_000) {
      val next = ln(i)
      current.next = next
      current = next
    }
    current.next = cycleHead
    val head = cycleHead

    detectCycle(head) shouldBe cycleHead
  }
}
