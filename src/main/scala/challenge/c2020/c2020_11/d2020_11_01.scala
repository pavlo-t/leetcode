package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_11_01 extends AnyWordSpec with Matchers {

  /**
   * <h3>Convert Binary Number in a Linked List to Integer</h3>
   *
   * Given `head` which is a reference node to a singly-linked list.
   * The value of each node in the linked list is either 0 or 1.
   * The linked list holds the binary representation of a number.
   *
   * Return the <em>decimal value</em> of the number in the linked list.
   *
   * <b>Constraints:</b><ul>
   * <li> The Linked List is not empty.
   * <li> Number of nodes will not exceed 30.
   * <li> Each node's value is either 0 or 1.
   * </ul>
   */
  object Solution {
    def getDecimalValue(head: ListNode): Int = {
      @scala.annotation.tailrec
      def toInt(n: ListNode, rsf: Int = 0): Int =
        if (n == null) rsf
        else toInt(n.next, (rsf << 1) + n.x)

      toInt(head)
    }
  }
  object SolutionBitwise {
    def getDecimalValue(head: ListNode): Int = {
      @scala.annotation.tailrec
      def toInt(n: ListNode, rsf: Int = 0): Int =
        if (n == null) rsf
        else toInt(n.next, (rsf << 1) | n.x)

      toInt(head)
    }
  }
  object SolutionArithmetic {
    def getDecimalValue(head: ListNode): Int = {
      @scala.annotation.tailrec
      def toInt(n: ListNode, rsf: Int = 0): Int =
        if (n == null) rsf
        else toInt(n.next, rsf * 2 + n.x)

      toInt(head)
    }
  }
  object SolutionParseString {
    def getDecimalValue(head: ListNode): Int = {
      @scala.annotation.tailrec
      def toString(h: ListNode, rsf: String = ""): String =
        if (h == null) rsf
        else toString(h.next, rsf + h.x)

      Integer.parseInt(toString(head), 2)
    }
  }
  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x
  }

  private def L(x: Int, n: ListNode = null): ListNode = new ListNode(x, n)
  import Solution.getDecimalValue

  "Example 1: ([1,0,1]) -> 5" in {
    val head = L(1, L(0, L(1)))
    getDecimalValue(head) shouldBe 5
    //Explanation: (101) in base 2 = (5) in base 10
  }
  "Example 2: ([0]) -> 0" in {
    getDecimalValue(L(0)) shouldBe 0
  }
  "Example 3: ([1]) -> 1" in {
    getDecimalValue(L(1)) shouldBe 1
  }
  "Example 4: ([1,0,0,1,0,0,1,1,1,0,0,0,0,0,0]) -> 18880" in {
    val head = L(1, L(0, L(0, L(1, L(0, L(0, L(1, L(1, L(1, L(0, L(0, L(0, L(0, L(0, L(0)))))))))))))))
    getDecimalValue(head) shouldBe 18880
  }
  "Example 5: ([0,0]) -> 0" in {
    getDecimalValue(L(0, L(0))) shouldBe 0
  }

}
