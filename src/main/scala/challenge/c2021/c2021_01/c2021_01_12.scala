package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3601/]]
 */
//noinspection DuplicatedCode
class c2021_01_12 extends AnyWordSpec with Matchers {
  /**
   * === Add Two Numbers ===
   *
   * You are given two '''non-empty''' linked lists representing two non-negative integers.
   * The digits are stored in '''reverse order''', and each of their nodes contains a single digit.
   * Add the two numbers and return the sum as a linked list.
   *
   * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
   *
   * '''Constraints:'''
   *  - The number of nodes in each linked list is in the range `[1, 100]`.
   *  - `0 <= Node.val <= 9`
   *  - It is guaranteed that the list represents a number that does not have leading zeros.
   */
  object Solution {
    def addTwoNumbers(l1: ListNode, l2: ListNode): ListNode = {
      def rec(n1: ListNode, n2: ListNode, carry: Int = 0): ListNode = (n1, n2, carry) match {
        case (null, null, 0) => null
        case (null, null, c) => new ListNode(c)
        case (n1, null, c)   => new ListNode((n1.x + c) % 10, rec(n1.next, null, (n1.x + c) / 10))
        case (null, n2, c)   => rec(n2, null, c)
        case (n1, n2, c)     => new ListNode((n1.x + n2.x + c) % 10, rec(n1.next, n2.next, (n1.x + n2.x + c) / 10))
      }
      rec(l1, l2)
    }
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = if (next != null) s"$x$next" else x.toString
  }

  import Solution.addTwoNumbers

  private def l(x: Int, n: ListNode = null): ListNode = new ListNode(x, n)

  "Example 1: (l1 = [2,4,3], l2 = [5,6,4]) -> [7,0,8]" in {
    val l1 = l(2, l(4, l(3)))
    val l2 = l(5, l(6, l(4)))
    addTwoNumbers(l1, l2).toString shouldBe l(7, l(0, l(8))).toString
    //Explanation: 342 + 465 = 807.
  }
  "Example 2: (l1 = [0], l2 = [0]) -> [0]" in {
    addTwoNumbers(l(0), l(0)).toString shouldBe l(0).toString
  }
  "Example 3: (l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]) -> [8,9,9,9,0,0,0,1]" in {
    val l1 = l(9, l(9, l(9, l(9, l(9, l(9, l(9)))))))
    val l2 = l(9, l(9, l(9, l(9))))
    val e = l(8, l(9, l(9, l(9, l(0, l(0, l(0, l(1))))))))
    addTwoNumbers(l1, l2).toString shouldBe e.toString

  }


}
