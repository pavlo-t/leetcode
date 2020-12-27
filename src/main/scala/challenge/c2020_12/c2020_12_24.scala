package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3579/]]
 */
//noinspection DuplicatedCode
class c2020_12_24 extends AnyWordSpec with Matchers {
  /**
   * === Swap Nodes in Pairs ===
   *
   * Given a linked list, swap every two adjacent nodes and return its head.
   *
   * You may '''not''' modify the values in the list's nodes.
   * Only nodes itself may be changed.
   *
   * '''Constraints:'''
   *  - The number of nodes in the list is in the range `[0, 100]`.
   *  - `0 <= Node.val <= 100`
   */
  object Solution {
    def swapPairs(head: ListNode): ListNode =
      if (head == null) null
      else if (head.next == null) new ListNode(head.x)
      else {
        val x = new ListNode(head.x)
        val y = new ListNode(head.next.x)
        x.next = swapPairs(head.next.next)
        y.next = x
        y
      }
  }

  object SolutionNewListMyTailrec {
    def swapPairs(head: ListNode): ListNode = {
      if (head == null || head.next == null) head
      else {
        @scala.annotation.tailrec
        def rec(n: ListNode, p: ListNode, root: ListNode): ListNode = {
          if (n == null) root
          else if (n.next == null) {
            p.next = new ListNode(n.x)
            root
          } else {
            p.next = new ListNode(n.next.x)
            p.next.next = new ListNode(n.x)
            rec(n.next.next, p.next.next, root)
          }
        }
        val newRoot = new ListNode(head.next.x, new ListNode(head.x))
        rec(head.next.next, newRoot.next, newRoot)
      }
    }
  }

  object SolutionMutateInPlace {
    def swapPairs(head: ListNode): ListNode =
      if (head == null || head.next == null) head
      else {
        val x = head
        val y = head.next
        x.next = swapPairs(y.next)
        y.next = x
        y
      }
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = s"$x,$next"
  }

  import Solution.swapPairs

  private def L(x: Int, n: ListNode = null): ListNode = new ListNode(x, n)

  "Example 1: (head = [1,2,3,4]) -> [2,1,4,3]" in {
    val head = L(1, L(2, L(3, L(4))))
    val expected = L(2, L(1, L(4, L(3))))
    swapPairs(head).toString shouldBe expected.toString
  }
  "Example 2: (head = []) -> []" in {
    swapPairs(null) shouldBe null
  }
  "Example 3: (head = [1]) -> [1]" in {
    val head = L(1)
    val expected = L(1)
    swapPairs(head).toString shouldBe expected.toString
  }

  "(head = [1,2,3,4,5]) -> [2,1,4,3,5]" in {
    val head = L(1, L(2, L(3, L(4, L(5)))))
    val expected = L(2, L(1, L(4, L(3, L(5)))))
    swapPairs(head).toString shouldBe expected.toString
  }
}
