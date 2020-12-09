package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/569/week-1-december-1st-december-7th/3556/]]
 */
//noinspection ConvertNullInitializerToUnderscore,DuplicatedCode
class c2020_12_06 extends AnyWordSpec with Matchers {

  /**
   * === Populating Next Right Pointers in Each Node II ===
   *
   * Given a binary tree
   *
   * {{{
   * struct Node {
   *   int val;
   *   Node *left;
   *   Node *right;
   *   Node *next;
   * }
   * }}}
   *
   * Populate each next pointer to point to its next right node.
   * If there is no next right node, the next pointer should be set to `NULL`.
   *
   * Initially, all next pointers are set to `NULL`.
   *
   * '''Follow up:'''
   *  - You may only use constant extra space.
   *  - Recursive approach is fine, you may assume implicit stack space does not count as extra space for this problem.
   *
   * '''Constraints:'''
   *  - The number of nodes in the given tree is less than `6000`.
   *  - `-100 <= node.val <= 100`
   */
  object Solution {
    def connect(root: Node): Node =
      if (root == null) root
      else {
        @scala.annotation.tailrec
        def connectNodes(leftmost: Node, currLvlCurr: Node, nextLvlCurr: Node): Unit =
          (leftmost, currLvlCurr, nextLvlCurr) match {
            case (null, null, _) => ()
            case (lm, null, _)   => connectNodes(null, lm, null)

            case (null, c, null) if c.left != null => connectNodes(c.left, c, c.left)
            case (null, c, null)                   => connectNodes(c.right, c.next, c.right)

            case (lm, clc, nlc) =>
              val nNlc = (clc.left, clc.right) match {
                case (null, null)          => nlc
                case (l, null) if l == nlc => nlc
                case (l, null)             => nlc.next = l; l
                case (null, r)             => nlc.next = r; r
                case (l, r)                => nlc.next = l; l.next = r; r
              }
              connectNodes(lm, clc.next, nNlc)
          }

        connectNodes(null, root, null)
        root
      }
  }

  /** [[https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii/solution/]] */
  object SolutionIterateOverNextPointers {
    var prev: Node = null
    var leftmost: Node = null

    def processChild(childNode: Node): Unit = {
      if (childNode != null) {
        // If the "prev" pointer is already set, i.e. if we already found at least one node on the next level,
        // setup its next pointer
        if (this.prev != null) this.prev.next = childNode
        else { // Else it means this child node is the first node
          // we have encountered on the next level, so, we
          // set the leftmost pointer
          this.leftmost = childNode
        }
        this.prev = childNode
      }
    }

    def connect(root: Node): Node = {
      if (root == null) return root

      // The root node is the only node on the first level
      // and hence its the leftmost node for that level
      this.leftmost = root

      // Variable to keep track of leading node on the "current" level
      var curr = leftmost

      // We have no idea about the structure of the tree, so, we keep going until we do find the last level.
      // the nodes on the last level won't have any children
      while (this.leftmost != null) {

        // "prev" tracks the latest node on the "next" level while "curr" tracks the latest node on the current level.
        this.prev = null
        curr = this.leftmost

        // We reset this so that we can re-assign it to the leftmost node of the next level.
        // Also, if there isn't one, this would help break us out of the outermost loop.
        this.leftmost = null

        // Iterate on the nodes in the current level using the next pointers already established.
        while (curr != null) {
          // Process both the children and update the prev and leftmost pointers as necessary.
          this.processChild(curr.left)
          this.processChild(curr.right)
          // Move onto the next node.
          curr = curr.next
        }
      }

      root
    }

  }

  object SolutionQueueWithLevelSize {
    def connect(root: Node): Node =
      if (root == null) root
      else {
        @scala.annotation.tailrec
        def connectNodes(size: Int, todo: Seq[Node]): Unit = todo match {
          case Nil               => ()
          case todo if size == 0 => connectNodes(todo.size, todo)
          case n :: rest         =>
            if (size > 1)
              n.next = rest.head
            val nexts = Seq(n.left, n.right).filter(_ != null)
            connectNodes(size - 1, rest ++ nexts)
        }

        connectNodes(1, Seq(root))
        root
      }
  }
  object SolutionQueueNulls {
    def connect(root: Node): Node =
      if (root == null) root
      else {
        @scala.annotation.tailrec
        def connectNodes(todo: Seq[Node]): Unit = todo match {
          case Nil                 => ()
          case null :: Nil         => ()
          case null :: rest        => connectNodes(rest :+ null)
          case n :: (rest@n2 :: _) =>
            val nexts = Seq(n.left, n.right).filter(_ != null)
            n.next = n2
            connectNodes(rest ++ nexts)
        }

        connectNodes(Seq(root, null))
        root
      }
  }
  object SolutionQueueTuples {
    def connect(root: Node): Node =
      if (root == null) root
      else {
        @scala.annotation.tailrec
        def connectNodes(todo: Seq[(Node, Int)]): Unit = todo match {
          case Nil            => ()
          case (n, l) :: rest =>
            val nexts = Seq(n.left, n.right).filter(_ != null).map((_, l + 1))
            if (rest.nonEmpty) {
              val (n2, l2) :: _ = rest
              if (l == l2) n.next = n2
            }
            connectNodes(rest ++ nexts)
        }

        connectNodes(Seq((root, 0)))
        root
      }
  }

  class Node(var _value: Int) {
    var value: Int = _value
    var left: Node = null
    var right: Node = null
    var next: Node = null

    override def toString: String =
      s"{$value," +
        s"${if (left != null) left else "n"}," +
        s"${if (right != null) right else "n"};" +
        s"${if (next != null) next.value else "n"}}"
  }

  import Solution.connect

  private def N(v: Int, l: Node = null, r: Node = null, n: Node = null) = {
    val node = new Node(v)
    node.left = l
    node.right = r
    node.next = n
    node
  }

  "Example 1: (root = [1,2,3,4,5,null,7]) -> [1,#,2,3,#,4,5,7,#]" in {
    val root = N(1, N(2, N(4), N(5)), N(3, r = N(7)))
    val n7 = N(7)
    val n5 = N(5, n = n7)
    val n4 = N(4, n = n5)
    val n3 = N(3, r = n7)
    val n2 = N(2, n4, n5, n3)
    val expected = N(1, n2, n3)

    val result = connect(root)
    result.toString shouldBe expected.toString
    //Explanation:
    // Given the above binary tree, the function should populate each next pointer to point to its next right node.
    // The serialized output is in level order as connected by the next pointers,
    // with '#' signifying the end of each level.
  }

  "Test 4: (root = [1,2]) -> [1,#,2,#]" in {
    val root = N(1, N(2))
    val expected = N(1, N(2))

    val result = connect(root)
    result.toString shouldBe expected.toString
  }

  "(root = [1,n,2]) -> [1,#,2,#]" in {
    val root = N(1, r = N(2))
    val expected = N(1, r = N(2))

    val result = connect(root)
    result.toString shouldBe expected.toString
  }

  "(root = null) -> null" in {
    connect(null) shouldBe null
  }
  "(root = [6000 nodes])" in {
    def buildTree(size: Int, level: Int = 0): Node = {
      if (size < 1) null
      else N(level, buildTree(size / 2, level + 1), buildTree(size / 2, level + 1))
    }
    val root = buildTree(6000)

    connect(root)
  }

}
