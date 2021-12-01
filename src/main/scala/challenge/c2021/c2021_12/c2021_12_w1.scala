package challenge.c2021.c2021_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/problems/insert-into-a-sorted-circular-linked-list/]] */
//noinspection DuplicatedCode
class c2021_12_w1 extends AnyWordSpec with Matchers {
  /**
   * = 708. Insert into a Sorted Circular Linked List =
   *
   * Given a Circular Linked List node, which is sorted in ascending order,
   * write a function to insert a value `insertVal` into the list such that it remains a sorted circular list.
   * The given node can be a reference to any single node in the list
   * and may not necessarily be the smallest value in the circular list.
   *
   * If there are multiple suitable places for insertion, you may choose any place to insert the new value.
   * After the insertion, the circular list should remain sorted.
   *
   * If the list is empty (i.e., the given node is `null`),
   * you should create a new single circular list and return the reference to that single node.
   * Otherwise, you should return the originally given node.
   *
   * '''Constraints:'''
   *  - `0 <= Number of Nodes <= 50_000`
   *  - `-1_000_000 <= Node.val, insertVal <= 1_000_000`
   */
  object Solution {
    def insert(head: Node, insertVal: Int): Node = {
      if (head == null) {
        val n = new Node(insertVal)
        n.next = n
        n
      } else if (head.value == insertVal) {
        val n = new Node(insertVal)
        n.next = head.next
        head.next = n
        head
      } else if (head.value < insertVal) {
        var p = head
        while (p.next != head && p.next.value < insertVal && p.value <= p.next.value)
          p = p.next

        val n = new Node(insertVal)
        n.next = p.next
        p.next = n
        head
      } else {
        var p = head
        while (p.next != head && p.value <= p.next.value) p = p.next
        while (p.next != head && p.next.value < insertVal) p = p.next

        val n = new Node(insertVal)
        n.next = p.next
        p.next = n
        head
      }
    }
  }


  //noinspection ConvertNullInitializerToUnderscore
  class Node(var _value: Int) {
    var value: Int = _value
    var next: Node = null

    override def toString: String = {
      val sb = new StringBuilder
      sb.addOne('[')
      sb.addAll(value.toString)
      var curr = this.next
      while (curr != this) {
        if (curr == null) {
          sb.addOne(',')
          sb.addAll("NULL")
          curr = this
        } else {
          sb.addOne(',')
          sb.addAll(curr.value.toString)
          curr = curr.next
        }
      }
      sb.addOne(']')
      sb.toString
    }
  }

  private def ns(vs: Int*): Node = {
    if (vs.isEmpty) null
    else {
      val head = new Node(vs.head)
      var curr = head
      for (v <- vs.tail) {
        val n = new Node(v)
        curr.next = n
        curr = n
      }
      curr.next = head
      head
    }
  }

  "Example 1: (head = [3,4,1], insertVal = 2) -> [3,4,1,2]" in {
    val head = ns(3, 4, 1)
    val e = ns(3, 4, 1, 2)
    Solution.insert(head, 2).toString shouldBe e.toString
    // 1
    // ↑↘
    // 4←3 - head
    //
    // Explanation: In the figure above, there is a sorted circular list of three elements.
    // You are given a reference to the node with value 3, and we need to insert 2 into the list.
    // The new node should be inserted between node 1 and node 3.
    // After the insertion, the list should look like this, and we should still return node 3.
    //
    // 1→2
    // ↑ ↓
    // 4←3 - head
  }
  "Example 2: (head = [], insertVal = 1) -> [1]" in {
    val e = ns(1)
    Solution.insert(null, 1).toString shouldBe e.toString
    // Explanation: The list is empty (given head is null). We create a new single circular list and return the reference to that single node.
  }
  "Example 3: (head = [1], insertVal = 0) -> [1,0]" in {
    val head = ns(1)
    val e = ns(1, 0)
    Solution.insert(head, 0).toString shouldBe e.toString
  }

  "Test 92: (head = [3,5,1], insertVal = 0) -> [3,5,0,1]" in {
    val head = ns(3, 5, 1)
    val e = ns(3, 5, 0, 1)
    Solution.insert(head, 0).toString shouldBe e.toString
  }
  "Test 98: (head = [3,5,1], insertVal = 6) -> [3,5,6,1]" in {
    val head = ns(3, 5, 1)
    val e = ns(3, 5, 6, 1)
    Solution.insert(head, 6).toString shouldBe e.toString
  }
  "Test 104: (head = [3,3,5], insertVal = 0) -> [3,3,5,0]" in {
    val head = ns(3, 3, 5)
    val e = ns(3, 3, 5, 0)
    Solution.insert(head, 0).toString shouldBe e.toString
  }

  "(head = [1], insertVal = 2) -> [1,2]" in {
    val head = ns(1)
    val e = ns(1, 2)
    Solution.insert(head, 2).toString shouldBe e.toString
  }
  "(head = [1,3,4], insertVal = 2) -> [1,2,3,4]" in {
    val head = ns(1, 3, 4)
    val e = ns(1, 2, 3, 4)
    Solution.insert(head, 2).toString shouldBe e.toString
  }
  "(head = [1,2,3], insertVal = 1) -> [1,1,2,3]" in {
    val head = ns(1, 2, 3)
    val e = ns(1, 1, 2, 3)
    Solution.insert(head, 1).toString shouldBe e.toString
  }
  "(head = [1,2,3], insertVal = 0) -> [1,2,3,0]" in {
    val head = ns(1, 2, 3)
    val e = ns(1, 2, 3, 0)
    Solution.insert(head, 0).toString shouldBe e.toString
  }
  "(head = [1,2,3], insertVal = 4) -> [1,2,3,4]" in {
    val head = ns(1, 2, 3)
    val e = ns(1, 2, 3, 4)
    Solution.insert(head, 4).toString shouldBe e.toString
  }

  "(head = [7,8,9,1,2,3,4,6], insertVal = 5) -> [7,8,9,1,2,3,4,5,6]" in {
    val head = ns(7, 8, 9, 1, 2, 3, 4, 6)
    val e = ns(7, 8, 9, 1, 2, 3, 4, 5, 6)
    Solution.insert(head, 5).toString shouldBe e.toString
  }

}
