package challenge.c2022.c2022_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/problems/clone-graph/]]
 */
//noinspection DuplicatedCode
class c2022_01_23 extends AnyWordSpec with Matchers {

  /**
   * = 133. Clone Graph =
   *
   * Given a reference of a node in a [[https://en.wikipedia.org/wiki/Connectivity_(graph_theory)#Connected_graph connected]]
   * undirected graph.
   *
   * Return a [[https://en.wikipedia.org/wiki/Object_copying#Deep_copy deep copy]] (clone) of the graph.
   *
   * Each node in the graph contains a value (`int`) and a list (`List[Node]`) of its neighbors.
   *
   * {{{
   * class Node {
   *   public int val;
   *   public List<Node> neighbors;
   * }
   * }}}
   *
   * '''Test case format:'''
   *
   * For simplicity, each node's value is the same as the node's index (1-indexed).
   * For example, the first node with `val == 1`, the second node with `val == 2`, and so on.
   * The graph is represented in the test case using an adjacency list.
   *
   * '''An adjacency list''' is a collection of unordered '''lists''' used to represent a finite graph.
   * Each list describes the set of neighbors of a node in the graph.
   *
   * The given node will always be the first node with `val = 1`.
   * You must return the '''copy of the given node''' as a reference to the cloned graph.
   *
   * '''Constraints:'''
   *  - The number of nodes in the graph is in the range `[0, 100]`.
   *  - `1 <= Node.val <= 100`
   *  - `Node.val` is unique for each node.
   *  - There are no repeated edges and no self-loops in the graph.
   *  - The Graph is connected and all nodes can be visited starting from the given node.
   */
object Solution {
  def cloneGraph(graph: Node): Node =
    if (graph == null) null
    else {
      @scala.annotation.tailrec
      def toMap(todo: Set[Node], results: Map[Int, List[Int]]): Map[Int, List[Int]] =
        if (todo.isEmpty) results
        else toMap(
          todo.tail ++ todo.head.neighbors.filterNot(n => results.contains(n.value) || todo.contains(n)),
          results + (todo.head.value -> todo.head.neighbors.map(_.value)))

      val map = toMap(Set(graph), Map.empty)
        .map { case (k, neighbors) => k -> (new Node(k), neighbors) }

      for ((_, (node, neighbors)) <- map; neighbor <- neighbors.reverse)
        node.neighbors = map(neighbor)._1 :: node.neighbors

      map(1)._1
    }
}

  class Node(var _value: Int) {
    var value: Int = _value
    var neighbors: List[Node] = List()

    override def toString: String = {
      val results = toStringRec(Seq(this), Map())
      results.toList.sorted.mkString(":")
    }

    @scala.annotation.tailrec
    private def toStringRec(todo: Seq[Node], results: Map[Int, String]): Map[Int, String] =
      if (todo.isEmpty) results
      else toStringRec(
        todo.tail ++ todo.head.neighbors.filterNot(n => results.contains(n.value)),
        results.updated(todo.head.value, todo.head.neighborsStr)
      )

    private def neighborsStr: String =
      neighbors.map(_.value).mkString("[", ",", "]")
  }

  private def n(v: Int): Node = new Node(v)


  "[[2,4],[1,3],[2,4],[1,3]]" in {
    val n1 = n(1)
    val n2 = n(2)
    val n3 = n(3)
    val n4 = n(4)
    n1.neighbors = List(n2, n4)
    n2.neighbors = List(n1, n3)
    n3.neighbors = List(n2, n4)
    n4.neighbors = List(n1, n3)
    val result = Solution.cloneGraph(n1)
    assert(n1 != result)
    n1.toString shouldBe result.toString
    // Explanation: There are 4 nodes in the graph.
    // 1st node (val = 1)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
    // 2nd node (val = 2)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
    // 3rd node (val = 3)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
    // 4th node (val = 4)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
  }
  "[[]]" in {
    val n1 = n(1)
    val result = Solution.cloneGraph(n1)
    assert(n1 != result)
    n1.toString shouldBe result.toString
    // Explanation: Note that the input contains one empty list.
    // The graph consists of only one node with val = 1 and it does not have any neighbors.
  }
  "[]" in {
    Solution.cloneGraph(null) shouldBe null
    // Explanation: This an empty graph, it does not have any nodes.
  }
}
