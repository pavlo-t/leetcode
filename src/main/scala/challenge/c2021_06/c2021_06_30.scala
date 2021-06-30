package challenge.c2021_06

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/607/week-5-june-29th-june-30th/3797/]]
 */
class c2021_06_30 extends AnyWordSpec with Matchers {
  /**
   * == Lowest Common Ancestor of a Binary Tree ==
   *
   * Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
   *
   * According to the [[https://en.wikipedia.org/wiki/Lowest_common_ancestor definition of LCA on Wikipedia]]:
   * "The lowest common ancestor is defined between two nodes `p` and `q` as the lowest node in `T` that has
   * both `p` and `q` as descendants (where we allow '''a node to be a descendant of itself''')."
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[2, 100_000]`.
   *  - `-109 <= Node.val <= 109`
   *  - All `Node.val` are '''unique'''.
   *  - `p != q`
   *  - `p` and `q` will exist in the tree.
   */
  object Solution {
    def lowestCommonAncestor(root: TreeNode, p: TreeNode, q: TreeNode): TreeNode = {
      @scala.annotation.tailrec
      def getParents(todo: List[TreeNode], rsf: Map[TreeNode, TreeNode]): Map[TreeNode, TreeNode] =
        if (rsf.contains(p) && rsf.contains(q)) rsf
        else {
          val n :: rest = todo
          val (nTodo, nRsf) = (n.left, n.right) match {
            case (null, null) => (rest, rsf)
            case (l, null)    => (l :: rest, rsf.updated(l, n))
            case (null, r)    => (r :: rest, rsf.updated(r, n))
            case (l, r)       => (l :: r :: rest, rsf.updated(r, n).updated(l, n))
          }
          getParents(nTodo, nRsf)
        }
      val parents = getParents(List(root), Map(root -> null))

      @scala.annotation.tailrec
      def getPath(n: TreeNode, rsf: Set[TreeNode]): Set[TreeNode] =
        if (n == null) rsf
        else getPath(parents(n), rsf + n)
      val path = getPath(p, Set())

      @scala.annotation.tailrec
      def getLCA(n: TreeNode): TreeNode =
        if (path.contains(n)) n
        else getLCA(parents(n))
      getLCA(q)
    }

    /** [[https://afteracademy.com/blog/lowest-common-ancestor-of-a-binary-tree]]
     * 2. Iterative Approach */
    def lowestCommonAncestorIterative(root: TreeNode, p: TreeNode, q: TreeNode): TreeNode = {
      import collection.mutable

      val parents = mutable.Map[TreeNode, TreeNode](root -> null)
      val stack = mutable.Stack(root)

      while (!parents.contains(p) || !parents.contains(q)) {
        val n = stack.pop()
        if (n.left != null) {
          parents(n.left) = n
          stack.push(n.left)
        }
        if (n.right != null) {
          parents(n.right) = n
          stack.push(n.right)
        }
      }

      val path = mutable.Set[TreeNode]()
      var n = p
      while (n != null) {
        path += n
        n = parents(n)
      }

      n = q
      while (!path.contains(n)) {
        n = parents(n)
      }

      n
    }

    /** [[https://afteracademy.com/blog/lowest-common-ancestor-of-a-binary-tree]]
     * 1. Recursive Approach */
    def lowestCommonAncestorRecursionStackOverflow(root: TreeNode, p: TreeNode, q: TreeNode): TreeNode =
      if (root == null) null
      else if (root == p || root == q) root
      else {
        val l = lowestCommonAncestor(root.left, p, q)
        val r = lowestCommonAncestor(root.right, p, q)
        if (l == null) r
        else if (r == null) l
        else root
      }

    def lowestCommonAncestorMyRecursionStackOverflow(root: TreeNode, p: TreeNode, q: TreeNode): TreeNode = {
      def getPath(v: Int, n: TreeNode): Option[Seq[TreeNode]] =
        if (n == null) None
        else if (n.value == v) Some(Seq(n))
        else getPath(v, n.left).orElse(getPath(v, n.right)).map(n +: _)

      getPath(p.value, root).get
        .zip(getPath(q.value, root).get)
        .takeWhile { case (l, r) => l == r }
        .last._1
    }
  }

  //noinspection ConvertNullInitializerToUnderscore
  class TreeNode(var _value: Int) {
    var value: Int = _value
    var left: TreeNode = null
    var right: TreeNode = null
  }

  import Solution.lowestCommonAncestor

  private def n(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = {
    val n = new TreeNode(v)
    n.left = l
    n.right = r
    n
  }

  "Example 1: (root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1) -> 3" in {
    val p = n(5, n(6), n(2, n(7), n(4)))
    val q = n(1, n(0), n(8))
    val root = n(3, p, q)
    lowestCommonAncestor(root, p, q) shouldBe root
    //       3
    //     /   \
    //   5       1
    //  / \     / \
    // 6   2   0   8
    //    / \
    //   7   4
    //Explanation: The LCA of nodes 5 and 1 is 3.
  }
  "Example 2: (root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4) -> 5" in {
    val q = n(4)
    val p = n(5, n(6), n(2, n(7), q))
    val root = n(3, p, n(1, n(0), n(8)))
    lowestCommonAncestor(root, p, q) shouldBe p
    //       3
    //     /   \
    //   5       1
    //  / \     / \
    // 6   2   0   8
    //    / \
    //   7   4
    //Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
  }
  "Example 3: (root = [1,2], p = 1, q = 2) -> 1" in {
    val q = n(2)
    val p = n(1, q)
    val root = p
    lowestCommonAncestor(root, p, q) shouldBe p
  }

  "(root = [1..100_000], p = 99_999, q = 99_998) -> 99_998" in {
    val root = n(1)
    var v = 2
    var curr = root
    while (v < 99_997) {
      curr.left = n(v)
      v += 1
      curr = curr.left
    }
    val p = n(99_999)
    val q = n(99_998, p)
    curr.left = q
    lowestCommonAncestor(root, p, q) shouldBe q
  }

}
