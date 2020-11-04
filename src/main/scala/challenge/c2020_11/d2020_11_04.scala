package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/564/week-1-november-1st-november-7th/3519/]]
 */
//noinspection DuplicatedCode
class d2020_11_04 extends AnyWordSpec with Matchers {

  /**
   * <h3>Minimum Height Trees</h3>
   *
   * A tree is an undirected graph in which any two vertices are connected by exactly one path.
   * In other words, any connected graph without simple cycles is a tree.
   *
   * Given a tree of `n` nodes labelled from `0` to `n - 1`,
   * and an array of `n - 1` `edges` where `edges[i] = [ai, bi]` indicates that there is an undirected edge
   * between the two nodes `ai` and `bi` in the tree, you can choose any node of the tree as the root.
   * When you select a node `x` as the root, the result tree has height `h`.
   * Among all possible rooted trees,
   * those with minimum height (i.e. `min(h)`)  are called <b>minimum height trees</b> (MHTs).
   *
   * Return <em>a list of all <b>MHTs</b>' root labels<em>.
   * You can return the answer in <b>any order</b>.
   *
   * The <b>height</b> of a rooted tree is the number of edges on the longest downward path between the root and a leaf.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= n <= 20_000`
   * <li> `edges.length == n - 1`
   * <li> `0 <= ai, bi < n`
   * <li> `ai != bi`
   * <li> All the pairs `(ai, bi)` are distinct.
   * <li> The given input is <b>guaranteed</b> to be a tree and there will be <b>no repeated</b> edges.
   * </ul>
   */
  object Solution {
    def findMinHeightTrees(n: Int, edges: Array[Array[Int]]): List[Int] =
      if (n <= 2) (0 until n).toList
      else {
        import collection.mutable

        val tree = mutable.Map[Int, mutable.Set[Int]]()
        for (Array(f, t) <- edges) {
          if (!tree.contains(f)) tree(f) = mutable.Set(t) else tree(f) addOne t
          if (!tree.contains(t)) tree(t) = mutable.Set(f) else tree(t) addOne f
        }

        @scala.annotation.tailrec
        def cutLeaves(rn: Int, leaves: Iterable[Int]): Iterable[Int] =
          if (rn <= 2) leaves
          else {
            val newLeaves = mutable.ListBuffer[Int]()
            for (l <- leaves; n <- tree(l)) {
              tree(n) remove l
              if (tree(n).size == 1) newLeaves addOne n
            }
            cutLeaves(rn - leaves.size, newLeaves)
          }
        cutLeaves(n, tree.filter(_._2.size == 1).keys).toList
      }
  }

  import Solution.findMinHeightTrees

  "Example 1: (n = 4, edges = [[1,0],[1,2],[1,3]]) -> [1]" in {
    val edges = Array(Array(1, 0), Array(1, 2), Array(1, 3))
    findMinHeightTrees(4, edges) shouldBe List(1)
    // Explanation: the height of the tree is 1 when the root is the node with label 1 which is the only MHT.
  }
  "Example 2: (n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]) -> [3,4]" in {
    val edges = Array(Array(3, 0), Array(3, 1), Array(3, 2), Array(3, 4), Array(5, 4))
    findMinHeightTrees(6, edges) shouldBe List(3, 4)
  }
  "Example 3: (n = 1, edges = []) -> [0]" in {
    findMinHeightTrees(1, Array()) shouldBe List(0)
  }
  "Example 4: (n = 2, edges = [[0,1]]) -> [0,1]" in {
    val edges = Array(Array(0, 1))
    findMinHeightTrees(2, edges) shouldBe List(0, 1)
  }

  "(20_000, [...]) -> [9_999, 10_000]" in {
    val edges = Array.ofDim[Array[Int]](19_999)
    for (i <- edges.indices) edges(i) = Array(i, i + 1)

    findMinHeightTrees(20_000, edges) shouldBe List(9_999, 10_000)
  }
}
