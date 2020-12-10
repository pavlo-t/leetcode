package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/570/week-2-december-8th-december-14th/3560/]]
 */
//noinspection AccessorLikeMethodIsEmptyParen,DuplicatedCode,SameParameterValue
class c2020_12_09 extends AnyWordSpec with Matchers {

  /**
   * === Binary Search Tree Iterator ===
   *
   * Implement the `BSTIterator` class that represents an iterator over the
   * [[https://en.wikipedia.org/wiki/Tree_traversal#In-order_(LNR) in-order traversal]] of a binary search tree (BST):
   *  - `BSTIterator(TreeNode root)` Initializes an object of the `BSTIterator` class.
   *    The `root` of the BST is given as part of the constructor.
   *    The pointer should be initialized to a non-existent number smaller than any element in the BST.
   *  - `boolean hasNext()` Returns `true` if there exists a number in the traversal to the right of the pointer,
   *    otherwise returns `false`.
   *  - `int next()` Moves the pointer to the right, then returns the number at the pointer.
   *
   * Notice that by initializing the pointer to a non-existent smallest number,
   * the first call to `next()` will return the smallest element in the BST.
   *
   * You may assume that `next()` calls will always be valid.
   * That is, there will be at least a next number in the in-order traversal when `next()` is called.
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[1, 100_000]`.
   *  - `0 <= Node.val <= 1_000_000`
   *  - At most `100_000` calls will be made to `hasNext`, and `next`.
   *
   * '''Follow up:'''
   *  - Could you implement `next()` and `hasNext()` to run in average `O(1)` time and use `O(h)` memory,
   *    where `h` is the height of the tree?
   */
  class BSTIterator(_root: TreeNode) {
    private val stack = collection.mutable.Stack[TreeNode]()

    @scala.annotation.tailrec
    private def prependLeftSubtree(n: TreeNode): Unit =
      if (n != null) {
        stack.push(n)
        prependLeftSubtree(n.left)
      }

    prependLeftSubtree(_root)

    def next(): Int = {
      val n = stack.pop()
      prependLeftSubtree(n.right)

      n.value
    }

    def hasNext(): Boolean = stack.nonEmpty
  }

  /** `O(1)` time; `O(h)` memory; h = tree height */
  class BSTIteratorArrayDequeue(_root: TreeNode) {
    import collection.mutable

    private val queue = mutable.ArrayDeque[TreeNode]()
    @scala.annotation.tailrec
    private def prependLeftSubtree(n: TreeNode, rsf: mutable.ArrayDeque[TreeNode]): Unit =
      if (n != null) {
        rsf.prepend(n)
        prependLeftSubtree(n.left, rsf)
      }
    prependLeftSubtree(_root, queue)

    def next(): Int = {
      val n = queue.removeHead()
      prependLeftSubtree(n.right, queue)

      n.value
    }

    def hasNext(): Boolean = queue.nonEmpty
  }

  /** `O(1)` time; `O(n)` memory */
  class BSTIteratorQueue(_root: TreeNode) {
    private val queue = collection.mutable.Queue[Int]()

    private def fillQueue(n: TreeNode): Unit =
      if (n != null) {
        fillQueue(n.left)
        queue.addOne(n.value)
        fillQueue(n.right)
      }

    fillQueue(_root)

    def next(): Int = queue.dequeue()
    def hasNext(): Boolean = queue.nonEmpty
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String =
      if (left == null && right == null) value.toString
      else s"$value," +
        s"${if (left == null) "" else left}," +
        s"${if (right == null) "" else right}"
  }

  private def n(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [7,3,15,n,n,9,20])" in {
    val root: TreeNode = n(7, n(3), n(15, n(9), n(20)))
    val bSTIterator = new BSTIterator(root)

    bSTIterator.next() shouldBe 3
    bSTIterator.next() shouldBe 7
    bSTIterator.hasNext() shouldBe true
    bSTIterator.next() shouldBe 9
    bSTIterator.hasNext() shouldBe true
    bSTIterator.next() shouldBe 15
    bSTIterator.hasNext() shouldBe true
    bSTIterator.next() shouldBe 20
    bSTIterator.hasNext() shouldBe false
  }

  "(root = null)" in {
    val bSTIterator = new BSTIterator(null)
    bSTIterator.hasNext() shouldBe false
  }

  private def buildTree(depth: Int): TreeNode = {
    var value = 1
    def loop(depth: Int): TreeNode =
      if (depth == 0) null
      else {
        val l = loop(depth - 1)
        val root = n(value, l)
        value += 1
        root.right = loop(depth - 1)

        root
      }
    loop(depth)
  }

  "(root = 65535 nodes" in {
    val root = buildTree(16)

    val bst = new BSTIterator(root)

    var expected = 1
    while (bst.hasNext()) {
      bst.next() shouldBe expected
      expected += 1
    }
    expected shouldBe 65536
  }
  "(root = 131071 nodes" in {
    val root = buildTree(17)

    val bst = new BSTIterator(root)

    var expected = 1
    while (bst.hasNext()) {
      bst.next() shouldBe expected
      expected += 1
    }
    expected shouldBe 131072
  }

}
