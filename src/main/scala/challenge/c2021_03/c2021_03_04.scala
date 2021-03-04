package challenge.c2021_03

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3660/]]
 */
//noinspection ConvertNullInitializerToUnderscore,DuplicatedCode
class c2021_03_04 extends AnyWordSpec with Matchers {
  /**
   * === Intersection of Two Linked Lists ===
   *
   * Write a program to find the node at which the intersection of two singly linked lists begins.
   *
   * For example, the following two linked lists:
   *
   * {{{
   * A:       a1 -> a2 \
   *                     c1 -> c2 -> c3
   * B: b1 -> b2 -> b3 /
   * }}}
   *
   * begin to intersect at node c1.
   *
   * '''Notes:'''
   *  - If the two linked lists have no intersection at all, return `null`.
   *  - The linked lists must retain their original structure after the function returns.
   *  - You may assume there are no cycles anywhere in the entire linked structure.
   *  - Each value on each linked list is in the range `[1, 10^9]`.
   *  - Your code should preferably run in `O(n)` time and use only `O(1)` memory.
   */
  object Solution {
    import scala.annotation.tailrec

    def getIntersectionNode(headA: ListNode, headB: ListNode): ListNode = {
      @tailrec def getIntersection(la: ListNode, lb: ListNode): ListNode =
        if (la == lb) la
        else getIntersection(
          if (la == null) headB else la.next,
          if (lb == null) headA else lb.next)
      getIntersection(headA, headB)
    }
  }

  object SolutionMy {
    import scala.annotation.tailrec

    def getIntersectionNode(headA: ListNode, headB: ListNode): ListNode = {
      if (headA == null || headB == null) null
      else if (headA == headB) headA
      else {
        @tailrec def size(l: ListNode, rsf: Int = 0): Int =
          if (l == null) rsf else size(l.next, rsf + 1)
        @tailrec def drop(l: ListNode, n: Int): ListNode =
          if (n == 0) l else drop(l.next, n - 1)
        @tailrec def getIntersection(l1: ListNode, l2: ListNode): ListNode =
          if (l1 == l2) l1 else getIntersection(l1.next, l2.next)

        val sa = size(headA)
        val sb = size(headB)

        if (sa > sb) getIntersection(drop(headA, sa - sb), headB)
        else if (sa < sb) getIntersection(headA, drop(headB, sb - sa))
        else getIntersection(headA, headB)
      }
    }
  }

  class ListNode(var _x: Int = 0) {
    var next: ListNode = null
    var x: Int = _x

    override def toString: String = s"$x,${if (next == null) "n" else next}"
  }

  import Solution.getIntersectionNode

  private def l(x: Int, next: ListNode = null): ListNode = {
    val n = new ListNode(x)
    n.next = next
    n
  }

  "Example 1: (intersectVal = 8, listA = [4,1,8,4,5], listB = [5,6,1,8,4,5], skipA = 2, skipB = 3) -> " +
    "Reference of the node with value = 8" in {
    // @formatter:off
    val intersect =         l(8, l(4, l(5)))
    val la =      l(4, l(1, intersect))
    val lb = l(5, l(6, l(1, intersect)))
    // @formatter:on
    val las = la.toString
    val lbs = lb.toString

    getIntersectionNode(la, lb) shouldBe intersect
    la.toString shouldBe las
    lb.toString shouldBe lbs
    //Explanation:
    // The intersected node's value is 8 (note that this must not be 0 if the two lists intersect).
    // From the head of A, it reads as [4,1,8,4,5].
    // From the head of B, it reads as [5,6,1,8,4,5].
    // There are 2 nodes before the intersected node in A;
    // There are 3 nodes before the intersected node in B.
  }
  "Example 2: (intersectVal = 2, listA = [1,9,1,2,4], listB = [3,2,4], skipA = 3, skipB = 1) -> " +
    "Reference of the node with value = 2" in {
    // @formatter:off
    val intersect =         l(2, l(4))
    val la = l(1, l(9, l(1, intersect)))
    val lb =           l(3, intersect)
    // @formatter:on
    val las = la.toString
    val lbs = lb.toString

    getIntersectionNode(la, lb) shouldBe intersect
    la.toString shouldBe las
    lb.toString shouldBe lbs
    //Explanation:
    // The intersected node's value is 2 (note that this must not be 0 if the two lists intersect).
    // From the head of A, it reads as [1,9,1,2,4].
    // From the head of B, it reads as [3,2,4].
    // There are 3 nodes before the intersected node in A;
    // There are 1 node before the intersected node in B.
  }
  "Example 3: (intersectVal = 0, listA = [2,6,4], listB = [1,5], skipA = 3, skipB = 2) -> null" in {
    val la = l(2, l(6, l(4)))
    val lb = l(1, l(5))
    val las = la.toString
    val lbs = lb.toString

    getIntersectionNode(la, lb) shouldBe null
    la.toString shouldBe las
    lb.toString shouldBe lbs
    //Explanation:
    // From the head of A, it reads as [2,6,4].
    // From the head of B, it reads as [1,5].
    // Since the two lists do not intersect, intersectVal must be 0, while skipA and skipB can be arbitrary values.
    // The two lists do not intersect, so return null.
  }

}
