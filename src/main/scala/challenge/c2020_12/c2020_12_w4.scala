package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3576/]]
 */
//noinspection DuplicatedCode
class c2020_12_w4 extends AnyWordSpec with Matchers {
  /**
   * === Find Nearest Right Node in Binary Tree ===
   *
   * Given the `root` of a binary tree and a node `u` in the tree,
   * return ''the '''nearest''' node on the '''same level''' that is to the '''right''' of'' `u`,
   * ''or return'' `null` ''if'' `u` ''is the rightmost node in its level''.
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[1, 100_000]`.
   *  - `1 <= Node.val <= 100_000`
   *  - All values in the tree are '''distinct'''.
   *  - `u` is a node in the binary tree rooted at `root`.
   */
  object Solution {
    import collection.mutable

    def findNearestRightNode(root: TreeNode, u: TreeNode): TreeNode = {
      val queue = mutable.Queue(root, null)
      while (queue.size > 1) {
        val n = queue.dequeue()
        if (n == null) queue.enqueue(null)
        else if (n.value == u.value) return queue.dequeue()
        else {
          if (n.left != null) queue.enqueue(n.left)
          if (n.right != null) queue.enqueue(n.right)
        }
      }
      null
    }
  }

  object SolutionRecursionImmutable {
    import collection.immutable.Queue
    import scala.annotation.tailrec

    def findNearestRightNode(root: TreeNode, u: TreeNode): TreeNode = {
      @tailrec
      def bfs(ns: Queue[TreeNode]): TreeNode = {
        ns.dequeueOption match {
          case None | Some((null, Queue()))         => null
          case Some((null, nns))                    => bfs(nns.appended(null))
          case Some((n, nns)) if n.value == u.value => nns.head
          case Some((n, nns))                       => bfs(nns.enqueueAll(Iterable(n.left, n.right).filter(_ != null)))
        }
      }
      bfs(Queue(root, null))
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String =
      if (left == null && right == null) value.toString
      else s"$value,$left,$right"
  }

  import Solution.findNearestRightNode

  private def N(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [1,2,3,null,4,5,6], u = 4) -> 5" in {
    val n4 = N(4)
    val n5 = N(5)
    val root =
      N(1,
        N(2,
          null,
          n4),
        N(3,
          n5,
          N(6)))

    findNearestRightNode(root, n4).toString shouldBe n5.toString
    //Explanation: The nearest node on the same level to the right of node 4 is node 5.
  }
  "Example 2: (root = [3,null,4,2], u = 2) -> null" in {
    val n2 = N(2)
    val root = N(3, null, N(4, n2))

    findNearestRightNode(root, n2) shouldBe null
    //Explanation: There are no nodes to the right of 2.
  }
  "Example 3: (root = [1], u = 1) -> null" in {
    val root = N(1)
    findNearestRightNode(root, root) shouldBe null
  }
  "Example 4: (root = [3,4,2,null,null,null,1], u = 4) -> 2" in {
    val n4 = N(4)
    val n2 = N(2, null, N(1))
    val root = N(3, n4, n2)
    findNearestRightNode(root, n4).toString shouldBe n2.toString
  }

  private def buildBST(from: Int, to: Int): TreeNode = {
    if (from > to) null
    else {
      val mid = from + (to - from) / 2
      N(mid, buildBST(from, mid - 1), buildBST(mid + 1, to))
    }
  }

  "(root = 100_000 balanced nodes, u = 99_998) -> 100_000" in {
    val root = buildBST(1, 100_000)
    findNearestRightNode(root, N(99_998)).toString shouldBe N(100_000).toString
  }

  "(root = 100_000 balanced nodes, u = 99_999) -> null" in {
    val root = buildBST(1, 100_000)
    findNearestRightNode(root, N(99_999)) shouldBe null
  }

  "(root = 100_000 balanced nodes, u = 50_000) -> null" in {
    val root = buildBST(1, 100_000)
    findNearestRightNode(root, N(50_000)) shouldBe null
  }

}
