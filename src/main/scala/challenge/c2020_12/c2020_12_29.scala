package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/december-leetcoding-challenge/573/week-5-december-29th-december-31st/3585/]]
 */
//noinspection DuplicatedCode
class c2020_12_29 extends AnyWordSpec with Matchers {
  /**
   * === Pseudo-Palindromic Paths in a Binary Tree ===
   *
   * Given a binary tree where node values are digits from 1 to 9.
   * A path in the binary tree is said to be '''pseudo-palindromic'''
   * if at least one permutation of the node values in the path is a palindrome.
   *
   * ''Return the number of '''pseudo-palindromic''' paths going from the root node to leaf nodes''.
   *
   * '''Constraints:'''
   *  - The given binary tree will have between `1` and `100_000` nodes.
   *  - Node values are digits from `1` to `9`.
   */
  object Solution {
    def pseudoPalindromicPaths(root: TreeNode): Int = {
      def isLeaf(n: TreeNode): Boolean = n.left == null && n.right == null

      /**
       * @example
       * {{{
       * isPseudoPalindromic(000000000) -> true
       * isPseudoPalindromic(000001000) -> true
       * isPseudoPalindromic(000000010) -> true
       * isPseudoPalindromic(000010010) -> false
       * }}}
       * @param path Int represents bit mask of odd and even counts
       * @return
       */
      def isPseudoPalindromic(path: Int): Boolean = (path & (path - 1)) == 0

      /**
       * @example
       * {{{
       * updatePath({v:1},000000000) -> 000000001
       * updatePath({v:1},000000001) -> 000000000
       * updatePath({v:3},000000001) -> 000000101
       * updatePath({v:3},000000101) -> 000000001
       * }}}
       * @param path Int represents bit mask of odd and even counts
       * @return
       */
      def updatePath(n: TreeNode, path: Int): Int = path ^ (1 << (n.value - 1))

      @scala.annotation.tailrec
      def tailRec(todo: Seq[(TreeNode, Int)], rsf: Int): Int = todo match {
        case Nil => rsf

        case (n, path) +: rest if isLeaf(n) && isPseudoPalindromic(path) => tailRec(rest, rsf + 1)
        case (n, _) +: rest if isLeaf(n)                                 => tailRec(rest, rsf)

        case (n, path) +: rest =>
          val nTodo = Seq(n.left, n.right).filter(_ != null).map(n => (n, updatePath(n, path)))
          tailRec(nTodo ++ rest, rsf)
      }

      tailRec(Seq((root, updatePath(root, 0))), 0)
    }
  }

  object SolutionMyTailRec {
    def pseudoPalindromicPaths(root: TreeNode): Int = {
      def isPseudoPalindromic(path: Seq[Int]): Boolean = path.map(_ % 2).count(_ > 0) <= 1

      def isLeaf(n: TreeNode): Boolean = n.left == null && n.right == null

      def updatePath(n: TreeNode, path: Seq[Int]): Seq[Int] = path.updated(n.value - 1, path(n.value - 1) + 1)

      @scala.annotation.tailrec
      def tailRec(todo: Seq[(TreeNode, Seq[Int])], rsf: Int): Int = todo match {
        case Nil => rsf

        case (n, path) +: rest if isLeaf(n) && isPseudoPalindromic(path) => tailRec(rest, rsf + 1)
        case (n, _) +: rest if isLeaf(n)                                 => tailRec(rest, rsf)

        case (n, path) +: rest =>
          val nTodo = Seq(n.left, n.right).filter(_ != null).map(n => (n, updatePath(n, path)))
          tailRec(nTodo ++ rest, rsf)
      }

      tailRec(Seq((root, Seq.fill(9)(0).updated(root.value - 1, 1))), 0)
    }
  }

  object SolutionBruteForceStackOverflow {
    def pseudoPalindromicPaths(root: TreeNode): Int = {
      def isPseudoPalindromic(ns: Seq[Int]): Boolean =
        ns.foldLeft(Array.fill(9)(0)) { case (acc, n) => acc(n - 1) += 1; acc }.map(_ % 2).count(_ > 0) <= 1

      def getPaths(node: TreeNode, path: Seq[Int]): Int = {
        if (node.left == null && node.right == null)
          if (isPseudoPalindromic(path)) 1 else 0
        else {
          val l = if (node.left != null) getPaths(node.left, path :+ node.left.value) else 0
          val r = if (node.right != null) getPaths(node.right, path :+ node.right.value) else 0
          l + r
        }
      }

      getPaths(root, Seq(root.value))
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right
  }

  import Solution.pseudoPalindromicPaths

  private def N(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [2,3,1,3,1,null,1]) -> 2" in {
    val root = N(2, N(3, N(3), N(1)), N(1, null, N(1)))
    pseudoPalindromicPaths(root) shouldBe 2
    //Explanation:
    //    2
    //  3   1
    // 3 1   1
    // There are three paths going from the root node to leaf nodes: [2,3,3], [2,1,1], [2,3,1].
    // Among these paths only 1st and 3rd paths are pseudo-palindromic
    // since the 1st path [2,3,3] can be rearranged in [3,2,3] (palindrome)
    // and the 3rd path [2,1,1] can be rearranged in [1,2,1] (palindrome).
  }
  "Example 2: (root = [2,1,1,1,3,null,null,null,null,null,1]) -> 1" in {
    val root = N(2, N(1, N(1), N(3, null, N(1))), N(1))
    pseudoPalindromicPaths(root) shouldBe 1
    //Explanation:
    //    2
    //  1   1
    // 1 3
    //    1
    // There are three paths going from the root node to leaf nodes: [2,1,1], [2,1,3,1], [2,1].
    // Among these paths only the 1st is pseudo-palindromic since [2,1,1] can be rearranged in [1,2,1] (palindrome).
  }
  "Example 3: (root = [9]) -> 1" in {
    pseudoPalindromicPaths(N(9)) shouldBe 1
  }

  "(root = [1,2,null]) -> 0" in {
    pseudoPalindromicPaths(N(1, N(2))) shouldBe 0
  }
  "(root = [2,2,null,2,null,3,null,3,null,3,null,3,null]) -> 1" in {
    val root = N(2, N(2, N(2, N(3, N(3, N(3, N(3)))))))
    pseudoPalindromicPaths(root) shouldBe 1
    //Explanation: [2,2,2,3,3,3,3] -> [3,3,2,2,2,3,3]
  }

  "(root = balanced tree of 131071 (2^17 - 1) 1s) -> 65536" in {
    def buildTree(depth: Int): TreeNode =
      if (depth == 0) null
      else N(1, buildTree(depth - 1), buildTree(depth - 1))

    val root = buildTree(17)
    pseudoPalindromicPaths(root) shouldBe 65536
  }

  "(root = left tree of 100000 1s) -> 1" in {
    @scala.annotation.tailrec
    def buildTree(depth: Int, curr: TreeNode = null, root: TreeNode = null): TreeNode =
      if (depth == 0) root
      else if (root == null) {
        val n = N(1)
        buildTree(depth - 1, n, n)
      } else {
        val n = N(1)
        curr.left = n
        buildTree(depth - 1, n, root)
      }

    val root = buildTree(100000)
    pseudoPalindromicPaths(root) shouldBe 1
  }


}
