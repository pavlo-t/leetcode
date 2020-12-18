package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/571/week-3-december-15th-december-21st/3566/]]
 */
//noinspection DuplicatedCode
class c2020_12_w3 extends AnyWordSpec with Matchers {

  /**
   * === Plus One Linked List ===
   *
   * Given a non-negative integer represented as a linked list of digits, ''plus one to the integer''.
   *
   * The digits are stored such that the most significant digit is at the `head` of the list.
   *
   * '''Constraints:'''
   *  - The number of nodes in the linked list is in the range `[1, 100]`.
   *  - `0 <= Node.val <= 9`
   *  - The number represented by the linked list does not contain leading zeros except for the zero itself.
   */
  object Solution {
    def plusOne(head: ListNode): ListNode = {
      def rec(n: ListNode): (ListNode, Int) = {
        if (n.next == null) {
          if (n.x == 9) (new ListNode(0), 1)
          else (new ListNode(n.x + 1), 0)
        } else
          rec(n.next) match {
            case (next, 0)             => (new ListNode(n.x, next), 0)
            case (next, 1) if n.x == 9 => (new ListNode(0, next), 1)
            case (next, 1)             => (new ListNode(n.x + 1, next), 0)
            case _                     => throw new IllegalStateException()
          }
      }

      rec(head) match {
        case (n, 0) => n
        case (n, 1) => new ListNode(1, n)
      }
    }
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = if (next == null) x.toString else s"$x,$next"
  }

  import Solution.plusOne

  private def L(v: Int, n: ListNode = null): ListNode = new ListNode(v, n)

  "Example 1: (head = [1,2,3]) -> [1,2,4]" in {
    val head = L(1, L(2, L(3)))
    val expected = L(1, L(2, L(4)))

    plusOne(head).toString shouldBe expected.toString
  }
  "Example 2: (head = [0]) -> [1]" in {
    plusOne(L(0)).toString shouldEqual L(1).toString
  }

  "(head = [1,2,9]) -> [1,3,0]" in {
    val head = L(1, L(2, L(9)))
    val expected = L(1, L(3, L(0)))

    plusOne(head).toString shouldBe expected.toString
  }
  "(head = [9,9,9]) -> [1,0,0,0]" in {
    val head = L(9, L(9, L(9)))
    val expected = L(1, L(0, L(0, L(0))))

    plusOne(head).toString shouldBe expected.toString
  }
}
