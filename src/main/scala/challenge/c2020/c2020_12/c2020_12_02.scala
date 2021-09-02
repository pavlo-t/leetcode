package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/december-leetcoding-challenge/569/week-1-december-1st-december-7th/3552/]]
 */
//noinspection AccessorLikeMethodIsEmptyParen,DuplicatedCode
class c2020_12_02 extends AnyWordSpec with Matchers {

  /**
   * === Linked List Random Node ===
   *
   * Given a singly linked list, return a random node's value from the linked list.
   * Each node must have the '''same probability''' of being chosen.
   *
   * '''Follow up:'''
   * What if the linked list is extremely large and its length is unknown to you?
   * Could you solve this efficiently without using extra space?
   *
   * @param head The linked list's head.
   *             Note that the head is guaranteed to be not null, so it contains at least one node.
   * @see [[https://leetcode.com/problems/linked-list-random-node/solution/]]
   * @see [[https://en.wikipedia.org/wiki/Reservoir_sampling]]
   */
  class Solution(head: ListNode) {
    /** Returns a random node's value. */
    def getRandom(): Int = {
      @scala.annotation.tailrec
      def tailrec(n: ListNode, scope: Int, rsf: Int): Int =
        if (n == null) rsf
        else tailrec(n.next, scope + 1, if (util.Random.nextDouble() < 1.0 / scope) n.x else rsf)

      tailrec(head, 1, head.x)
    }
  }

  class SolutionArray(head: ListNode) {
    import collection.mutable
    import util.Random.nextInt

    /** Returns a random node's value. */
    def getRandom(): Int = data(nextInt(data.length))

    private val data: Array[Int] = {
      @scala.annotation.tailrec
      def tailrec(n: ListNode, rsf: mutable.ArrayBuffer[Int]): Array[Int] =
        if (n == null) rsf.toArray
        else tailrec(n.next, rsf.addOne(n.x))
      tailrec(head, mutable.ArrayBuffer())
    }
  }

  class SolutionMy(head: ListNode) {
    /** Returns a random node's value. */
    def getRandom(): Int = {
      get(head, util.Random.nextInt(size))
    }

    @scala.annotation.tailrec
    private def get(n: ListNode, i: Int): Int =
      if (i == 0) n.x
      else get(n.next, i - 1)

    private val size: Int = {
      @scala.annotation.tailrec
      def tailrec(n: ListNode, rsf: Int): Int = {
        if (n == null) rsf
        else tailrec(n.next, rsf + 1)
      }

      tailrec(head, 0)
    }
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = s"$x,${if (next == null) "n" else next}"
  }
  private def L(x: Int, n: ListNode = null): ListNode = new ListNode(x, n)

  /**
   * Your Solution object will be instantiated and called as such:
   * var obj = new Solution(head)
   * var param_1 = obj.getRandom()
   */
  "Example: [1,2,3]" in {
    val counts = collection.mutable.Map[Int, Int]()
    for (i <- 1 to 3) counts(i) = 0

    val head = counts.keys.toSeq.sorted(Ordering[Int].reverse).foldLeft(null: ListNode)((acc, i) => L(i, acc))
    val solution = new Solution(head)

    for (_ <- 1 to 30000) counts(solution.getRandom()) += 1
    println(counts)

    counts.foreach(kv => kv._2 should be >= 9500)
  }

  "[1,2] get some randoms; update list to [1,2,3,4]; get more randoms" in {
    val counts = collection.mutable.Map[Int, Int]()
    for (i <- 1 to 2) counts(i) = 0

    val head = counts.keys.toSeq.sorted(Ordering[Int].reverse).foldLeft(null: ListNode)((acc, i) => L(i, acc))
    val solution = new Solution(head)

    for (_ <- 1 to 1000) counts(solution.getRandom()) += 1
    println(counts)

    counts(1) should be >= 450
    counts(2) should be >= 450

    head.next.next = L(3, L(4))
    counts(3) = 0
    counts(4) = 0

    for (_ <- 1 to 1000) counts(solution.getRandom()) += 1
    println(counts)

    counts(1) should be >= 650
    counts(2) should be >= 650
    counts(3) should be >= 200
    counts(4) should be >= 200
  }

  "[1,...,1000]" in {
    val counts = collection.mutable.Map[Int, Int]()
    for (i <- 1 to 1000) counts(i) = 0

    val head = counts.keys.toSeq.sorted(Ordering[Int].reverse).foldLeft(null: ListNode)((acc, i) => L(i, acc))
    val solution = new Solution(head)

    for (_ <- 1 to 30000) counts(solution.getRandom()) += 1
    println(counts.take(30))

    counts.foreach(kv => kv._2 should be >= 1)
  }
}
