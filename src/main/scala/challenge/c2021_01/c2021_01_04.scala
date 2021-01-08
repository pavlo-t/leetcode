package challenge.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3592/]]
 */
class c2021_01_04 extends AnyWordSpec with Matchers {
  /**
   * === Merge Two Sorted Lists ===
   *
   * Merge two sorted linked lists and return it as a '''sorted''' list.
   * The list should be made by splicing together the nodes of the first two lists.
   *
   * '''Constraints:'''
   *  - The number of nodes in both lists is in the range `[0, 50]`.
   *  - `-100 <= Node.val <= 100`
   *  - Both `l1` and `l2` are sorted in '''non-decreasing''' order.
   */
  object Solution {
    def mergeTwoLists(l1: ListNode, l2: ListNode): ListNode = {
      @scala.annotation.tailrec
      def tailRec(l1: ListNode, l2: ListNode, guard: ListNode, prev: ListNode): ListNode = {
        if (l1 == null && l2 == null) guard.next
        else if (l1 == null || (l2 != null && l1.x > l2.x)) tailRec(l2, l1, guard, prev)
        else {
          val curr = new ListNode(l1.x)
          prev.next = curr
          tailRec(l1.next, l2, guard, curr)
        }
      }

      val guard = new ListNode(0)
      tailRec(l1, l2, guard, guard)
    }
  }

  object SolutionRec {
    def mergeTwoLists(l1: ListNode, l2: ListNode): ListNode =
      if (l1 == null) l2
      else if (l2 == null) l1
      else if (l1.x <= l2.x) new ListNode(l1.x, mergeTwoLists(l1.next, l2))
      else mergeTwoLists(l2, l1)
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = s"$x,$next"
  }

  import Solution.mergeTwoLists

  private def L(x: Int, n: ListNode = null) = new ListNode(x, n)

  "Example 1: (l1 = [1,2,4], l2 = [1,3,4]) -> [1,1,2,3,4,4]" in {
    val l1 = L(1, L(2, L(4)))
    val l2 = L(1, L(3, L(4)))
    val expected = L(1, L(1, L(2, L(3, L(4, L(4))))))
    mergeTwoLists(l1, l2).toString shouldBe expected.toString
  }
  "Example 2: (l1 = [], l2 = []) -> []" in {
    mergeTwoLists(null, null) shouldBe null
  }
  "Example 3: (l1 = [], l2 = [0]) -> [0]" in {
    mergeTwoLists(null, L(0)).toString shouldBe L(0).toString
  }


}
