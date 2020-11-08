package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_11_07 extends AnyWordSpec with Matchers {

  /**
   * <h3>Add Two Numbers II</h3>
   *
   * You are given two <b>non-empty</b> linked lists representing two non-negative integers.
   * The most significant digit comes first and each of their nodes contain a single digit.
   * Add the two numbers and return it as a linked list.
   *
   * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
   *
   * <b>Follow up:</b>
   *
   * What if you cannot modify the input lists? In other words, reversing the lists is not allowed.
   */
  object Solution {
    def addTwoNumbers(l1: ListNode, l2: ListNode): ListNode = {
      @scala.annotation.tailrec
      def getSize(l: ListNode, rsf: Int = 0): Int =
        if (l == null) rsf
        else getSize(l.next, rsf + 1)

      def addWithCarry(a: Int, b: Int): (Int, Boolean) =
        a + b match {
          case x if x < 10 => (x, false)
          case x           => (x - 10, true)
        }

      val s1 = getSize(l1)
      val s2 = getSize(l2)

      val preRoot = new ListNode()
      val arr = Array.ofDim[ListNode]((s1 max s2) + 1)
      arr(0) = preRoot

      @scala.annotation.tailrec
      def carryOver(i: Int): Unit = {
        val (x, carry) = addWithCarry(arr(i).x, 1)
        arr(i).x = x
        if (carry) carryOver(i - 1)
      }

      @scala.annotation.tailrec
      def buildResult(n1: ListNode, s1: Int,
                      n2: ListNode, s2: Int,
                      i: Int = 0,
                      prev: ListNode = preRoot): Unit = {
        if (n1 != null) {
          if (s1 > s2) {
            val n = new ListNode(n1.x)
            arr(i + 1) = n
            prev.next = n
            buildResult(n1.next, s1 - 1, n2, s2, i + 1, n)
          } else {
            val (x, carry) = addWithCarry(n1.x, n2.x)
            val n = new ListNode(x)
            arr(i + 1) = n
            prev.next = n
            if (carry) carryOver(i)
            buildResult(n1.next, s1 - 1, n2.next, s2 - 1, i + 1, n)
          }
        }
      }

      if (s1 < s2) buildResult(l2, s2, l1, s1)
      else buildResult(l1, s1, l2, s2)

      if (preRoot.x == 0) preRoot.next else preRoot
    }
  }

  object SolutionReverse {
    def addTwoNumbers(l1: ListNode, l2: ListNode): ListNode = {
      @scala.annotation.tailrec
      def reverse(l: ListNode, rsf: ListNode = null): ListNode = {
        if (l == null) rsf
        else reverse(l.next, new ListNode(l.x, rsf))
      }

      def addWithCarry(n1: ListNode, n2: ListNode, v3: Int): (Int, Int) = {
        val x2 = if (n2 != null) n2.x else 0
        val r = n1.x + x2 + v3
        if (r >= 10) (r - 10, 1)
        else (r, 0)
      }
      def getNexts(n1: ListNode, n2: ListNode): (ListNode, ListNode) =
        if (n2 == null) (n1.next, null)
        else if (n1.next == null) (n2.next, n1.next)
        else (n1.next, n2.next)

      @scala.annotation.tailrec
      def addReversed(n1: ListNode, n2: ListNode,
                      carry: Int = 0,
                      root: ListNode = null,
                      prev: ListNode = null): ListNode = {
        if (n1 == null && n2 == null) {
          if (carry > 0)
            prev.next = new ListNode(carry)
          root
        } else {
          val (x, newCarry) = addWithCarry(n1, n2, carry)
          val n = new ListNode(x)
          val (next1, next2) = getNexts(n1, n2)
          if (root == null)
            addReversed(next1, next2, newCarry, n, n)
          else {
            prev.next = n
            addReversed(next1, next2, newCarry, root, n)
          }
        }
      }

      reverse(addReversed(reverse(l1), reverse(l2)))
    }
  }

  object SolutionWithBigInt {
    def addTwoNumbers(l1: ListNode, l2: ListNode): ListNode = {
      @scala.annotation.tailrec
      def toString(l: ListNode, rsf: StringBuilder = new StringBuilder()): String =
        if (l == null) rsf.toString
        else toString(l.next, rsf.addOne((l.x + '0').toChar))

      @scala.annotation.tailrec
      def fromString(s: String, root: ListNode = null, prev: ListNode = null): ListNode = {
        if (s.isEmpty) root
        else {
          val n = new ListNode(s.head - '0')
          if (root == null) fromString(s.tail, n, n)
          else {
            prev.next = n
            fromString(s.tail, root, n)
          }
        }
      }

      val r = BigInt(toString(l1)) + BigInt(toString(l2))
      fromString(r.toString)
    }
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = toStringTailRec(this)
    @scala.annotation.tailrec
    private def toStringTailRec(l: ListNode, rsf: StringBuilder = new StringBuilder()): String =
      if (l == null) rsf.toString
      else toStringTailRec(l.next, rsf.addOne((l.x + '0').toChar))
  }

  import Solution.addTwoNumbers

  private def LN(x: Int, n: ListNode = null): ListNode = new ListNode(x, n)
  @scala.annotation.tailrec
  private def L(s: String, root: ListNode = null, prev: ListNode = null): ListNode = {
    if (s.isEmpty) root
    else {
      val n = new ListNode(s.head - '0')
      if (root == null) L(s.tail, n, n)
      else {
        prev.next = n
        L(s.tail, root, n)
      }
    }
  }

  "Example: (7 -> 2 -> 4 -> 3) + (5 -> 6 -> 4): (7 -> 8 -> 0 -> 7)" in {
    val l1 = L("7243")
    val l2 = L("564")
    val expected = LN(7, LN(8, LN(0, LN(7))))

    addTwoNumbers(l1, l2).toString shouldBe expected.toString
  }

  "(1 repeat 1000) + (2 repeat 1000): (3 repeat 1000)" in {
    val l1 = L("1" * 1000)
    val l2 = L("2" * 1000)
    val expected = L("3" * 1000)

    addTwoNumbers(l1, l2).toString shouldBe expected.toString
  }
  "(3 repeat 1000) + (7 repeat 1000): (1 repeat 1000 + 0)" in {
    val l1 = L("3" * 1000)
    val l2 = L("7" * 1000)
    val expected = L("1" * 1000 + "0")

    addTwoNumbers(l1, l2).toString shouldBe expected.toString
  }
  "(3 repeat 10_000) + (7 repeat 10_000): (1 repeat 10_000 + 0)" in {
    val size = 10000
    val l1 = L("3" * size)
    val l2 = L("7" * size)
    val expected = L("1" * size + "0")

    addTwoNumbers(l1, l2).toString shouldBe expected.toString
  }
  "(3 repeat 20_000) + (7 repeat 20_000): (1 repeat 20_000 + 0)" in {
    val size = 20000
    val l1 = L("3" * size)
    val l2 = L("7" * size)
    val expected = L("1" * size + "0")

    addTwoNumbers(l1, l2).toString shouldBe expected.toString
  }
  "(9 repeat 20_000) + (1): (1 + 0 repeat 20_000)" in {
    val size = 20000
    val l1 = L("9" * size)
    val l2 = L("1")
    val expected = L("1" + "0" * size)

    addTwoNumbers(l1, l2).toString shouldBe expected.toString
  }
  "(1) + (9 repeat 20_000): (1 + 0 repeat 20_000)" in {
    val size = 20000
    val l1 = L("1")
    val l2 = L("9" * size)
    val expected = L("1" + "0" * size)

    addTwoNumbers(l1, l2).toString shouldBe expected.toString
  }
}
