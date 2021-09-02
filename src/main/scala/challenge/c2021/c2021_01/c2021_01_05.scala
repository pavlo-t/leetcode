package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3593/]]
 */
//noinspection DuplicatedCode,NameBooleanParameters
class c2021_01_05 extends AnyWordSpec with Matchers {
  /**
   * === Remove Duplicates from Sorted List II ===
   *
   * Given the `head` of a sorted linked list,
   * ''delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list''.
   * Return ''the linked list '''sorted''' as well''.
   *
   * '''Constraints:'''
   *  - The number of nodes in the list is in the range `[0, 300]`.
   *  - `-100 <= Node.val <= 100`
   *  - The list is guaranteed to be '''sorted''' in ascending order.
   */
  object Solution {
    def deleteDuplicates(head: ListNode): ListNode = {
      @scala.annotation.tailrec
      def tailRec(n: ListNode, p: ListNode, del: Boolean, sentinel: ListNode): ListNode = {
        if (n == null)
          sentinel.next
        else if (del)
          tailRec(n.next, p, n.next != null && n.x == n.next.x, sentinel)
        else if (n.next == null || n.x != n.next.x) {
          p.next = new ListNode(n.x)
          tailRec(n.next, p.next, false, sentinel)
        } else
          tailRec(n.next, p, true, sentinel)
      }

      val sentinel = new ListNode(0)
      tailRec(head, sentinel, false, sentinel)
    }
  }

  object SolutionRec {
    def deleteDuplicates(head: ListNode): ListNode = {
      def rec(h: ListNode, del: Boolean): ListNode = {
        if (h == null) null
        else if (h.next == null)
          if (del) null
          else new ListNode(h.x)
        else if (h.x == h.next.x) rec(h.next, true)
        else if (del) rec(h.next, false)
        else new ListNode(h.x, rec(h.next, false))
      }

      rec(head, false)
    }
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = s"$x,$next"
  }

  import Solution.deleteDuplicates

  private def L(x: Int, n: ListNode = null): ListNode = new ListNode(x, n)

  "Example 1: (head = [1,2,3,3,4,4,5]) -> [1,2,5]" in {
    val head = L(1, L(2, L(3, L(3, L(4, L(4, L(5)))))))
    val expected = L(1, L(2, L(5)))

    deleteDuplicates(head).toString shouldBe expected.toString
  }
  "Example 2: (head = [1,1,1,2,3]) -> [2,3]" in {
    val head = L(1, L(1, L(1, L(2, L(3)))))
    val expected = L(2, L(3))

    deleteDuplicates(head).toString shouldBe expected.toString
  }

  "(head = []) -> []" in {
    deleteDuplicates(null) shouldBe null
  }
  "(head = [1,1]) -> []" in {
    deleteDuplicates(L(1, L(1))) shouldBe null
  }
  "(head = [1,2,3]) -> [1,2,3]" in {
    val head = L(1, L(2, L(3)))
    val expected = L(1, L(2, L(3)))

    deleteDuplicates(head).toString shouldBe expected.toString
  }
  "(head = [1,2,3,3]) -> [1,2]" in {
    val head = L(1, L(2, L(3, L(3))))
    val expected = L(1, L(2))

    deleteDuplicates(head).toString shouldBe expected.toString
  }
  "(head = [1,2,2,2,2,3]) -> [1,3]" in {
    val head = L(1, L(2, L(2, L(2, L(2, L(3))))))
    val expected = L(1, L(3))

    deleteDuplicates(head).toString shouldBe expected.toString
  }

}
