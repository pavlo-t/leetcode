package c2020_10.w4

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_10_22 extends AnyWordSpec with Matchers {

  /**
   * <h3>Minimum Depth of Binary Tree</h3>
   *
   * Given a binary tree, find its minimum depth.
   *
   * The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
   *
   * <b>Note:</b> A leaf is a node with no children.
   *
   * <b>Constraints:</b><ul>
   * <li> The number of nodes in the tree is in the range `[0, 100_000]`.
   * <li> `-1000 <= Node.val <= 1000`
   * </ul>
   */
  object Solution {
    import scala.annotation.tailrec

    def minDepth(root: TreeNode): Int = {
      if (root == null) 0
      else {
        @tailrec
        def loop(todo: Map[Int, List[TreeNode]]): Int = {
          val (depth, nss) = todo.minBy(_._1)

          nss match {
            case Nil                                         => loop(todo.removed(depth))
            case n :: ns if n == null                        => loop(todo.updated(depth, ns))
            case n :: _ if n.left == null && n.right == null => depth
            case n :: ns                                     =>
              loop(todo
                .updated(depth, ns)
                .updated(depth + 1, n.left :: n.right :: todo.getOrElse(depth + 1, List())))

          }
        }
        loop(Map(1 -> List(root)))
      }
    }
  }
  object SolutionRecursivePlain {
    def minDepth(root: TreeNode): Int = {
      if (root == null) return 0
      (root.left, root.right) match {
        case (null, _) => minDepth(root.right) + 1
        case (_, null) => minDepth(root.left) + 1
        case (l, r)    => (minDepth(l) min minDepth(r)) + 1
      }
    }
  }
  object SolutionMutableMapList {
    import scala.annotation.tailrec
    import collection.mutable

    def minDepth(root: TreeNode): Int = {
      if (root == null) 0
      else {
        val todo = mutable.Map(1 -> mutable.ListBuffer(root))
        @tailrec
        def loop(): Int = {
          val (depth, nss) = todo.minBy(_._1)

          if (nss.isEmpty) {
            todo.remove(depth)
            loop()
          } else if (nss.head == null) {
            nss.remove(0)
            loop()
          } else if (nss.head.left == null && nss.head.right == null) {
            depth
          } else {
            if (!todo.contains(depth + 1))
              todo(depth + 1) = mutable.ListBuffer()
            val n = nss.remove(0)
            todo(depth + 1).addOne(n.left).addOne(n.right)
            loop()
          }
        }
        loop()
      }
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String = s"{$left,$value,$right}"
  }
  private def tn(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode =
    new TreeNode(v, l, r)

  import Solution.minDepth

  "Example 1: ([3,9,20,null,null,15,7]) -> 2" in {
    val root = tn(3, tn(9), tn(20, tn(15, tn(7))))
    minDepth(root) shouldBe 2
  }
  "Example 2: ([2,null,3,null,4,null,5,null,6]) -> 5" in {
    val root = tn(2, r = tn(3, r = tn(4, r = tn(5, r = tn(6)))))
    minDepth(root) shouldBe 5
  }

  "([]) -> 0" in {
    minDepth(null) shouldBe 0
  }
  "([1,null,null]) -> 1" in {
    val root = tn(1)
    minDepth(root) shouldBe 1
  }
  "([1,2,null,null,null]) -> 1" in {
    val root = tn(1, tn(2))
    minDepth(root) shouldBe 2
  }
  "(100_000 nodes tree) -> 100_000" in {
    val size = 100_000
    var root: TreeNode = tn(1)
    for (i <- 2 to size) root = tn(i, root)

    minDepth(root) shouldBe size
  }
}
