package c2020_10.w3

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_10_20 extends AnyWordSpec with Matchers {

  /**
   * <h3>Clone Graph</h3>
   *
   * Given a reference of a node in a
   * [[https://en.wikipedia.org/wiki/Connectivity_(graph_theory)#Connected_graph connected]] undirected graph.
   *
   * Return a [[https://en.wikipedia.org/wiki/Object_copying#Deep_copy deep copy]] (clone) of the graph.
   *
   * Each node in the graph contains a val (`int`) and a list (`List[Node]`) of its neighbors.
   *
   * <b>Test case format:</b>
   *
   * For simplicity sake, each node's value is the same as the node's index (1-indexed).
   * For example, the first node with `val = 1`, the second node with `val = 2`, and so on.
   * The graph is represented in the test case using an adjacency list.
   *
   * <b>Adjacency list</b> is a collection of unordered <b>lists</b> used to represent a finite graph.
   * Each list describes the set of neighbors of a node in the graph.
   *
   * The given node will always be the first node with `val = 1`.
   * You must return the <b>copy of the given node</b> as a reference to the cloned graph.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= Node.val <= 100`
   * <li> `Node.val` is unique for each node.
   * <li> Number of Nodes will not exceed 100.
   * <li> There is no repeated edges and no self-loops in the graph.
   * <li> The Graph is connected and all nodes can be visited starting from the given node.
   * </ul>
   *
   */
  object Solution {
    import collection.mutable

    def cloneGraph(graph: Node): Node =
      if (graph == null) null
      else {
        val cache = mutable.Map[Int, Node]()
        def deepCopy(node: Node): Node =
          cache.get(node.value) match {
            case Some(n) => n
            case None    =>
              val n = new Node(node.value)
              cache(n.value) = n
              n.neighbors = node.neighbors map deepCopy
              n
          }
        deepCopy(graph)
      }
  }
  object SolutionMyWithMap {
    import scala.annotation.tailrec

    def cloneGraph(graph: Node): Node =
      if (graph == null) null
      else {
        @tailrec
        def graphToMap(todo: List[Node],
                       seen: Set[Node],
                       rsf: Map[Int, List[Int]]): Map[Int, List[Int]] = todo match {
          case Nil     => rsf
          case n :: ns =>
            val nextTodo = ns ++ n.neighbors.filterNot(nn => seen.contains(nn))
            val nextSeen = seen ++ n.neighbors

            graphToMap(nextTodo, nextSeen, rsf.updated(n.value, n.neighbors.map(_.value)))
        }

        val nodesMap = graphToMap(List(graph), Set(graph), Map())
        val nodes = nodesMap.map { case (v, _) => new Node(v) }.toList.sortBy(_.value)

        for (n <- nodes) n.neighbors = nodes.filter(nn => nodesMap(n.value).contains(nn.value))

        nodes.find(_.value == 1).get
      }
  }
  class Node(var _value: Int) {
    import scala.annotation.tailrec

    var value: Int = _value
    var neighbors: List[Node] = List()

    private def nsStr = neighbors.map(_.value).mkString(",")

    override def toString: String = {
      @tailrec
      def loop(todo: List[Node], seen: Set[Int], rsf: String): String = todo match {
        case Nil     => rsf + ']'
        case n :: ns =>
          val nextTodo = n.neighbors.filterNot(n => seen.contains(n.value) || todo.exists(_.value == n.value)) ++ ns
          loop(nextTodo, seen + n.value, rsf + s",[${n.nsStr}]")
      }
      loop(neighbors, Set(value), s"[[$nsStr]")
    }
  }
  def N(v: Int, ns: List[Node] = List()): Node = {
    val n = new Node(v)
    n.neighbors = ns
    n
  }

  import Solution.cloneGraph

  "Example 1: [[2,4],[1,3],[2,4],[1,3]]" in {
    val n1 = N(1)
    val n2 = N(2)
    val n3 = N(3)
    val n4 = N(4)
    n1.neighbors = List(n2, n4)
    n2.neighbors = List(n1, n3)
    n3.neighbors = List(n2, n4)
    n4.neighbors = List(n1, n3)

    val result = cloneGraph(n1)

    result.toString shouldBe n1.toString
    result should not equal n1
    // Explanation: There are 4 nodes in the graph.
    //   1st node (val = 1)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
    //   2nd node (val = 2)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
    //   3rd node (val = 3)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
    //   4th node (val = 4)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
  }
  "Example 2: [[]]" in {
    val n1 = N(1)

    val result = cloneGraph(n1)

    result.toString shouldBe n1.toString
    result should not equal n1
    // Explanation:
    //   Note that the input contains one empty list.
    //   The graph consists of only one node with val = 1 and it does not have any neighbors.
  }
  "Example 3: []" in {
    val result = cloneGraph(null)

    result shouldBe null
    // Explanation: This an empty graph, it does not have any nodes.
  }
  "Example 4: [[2],[1]]" in {
    val n1 = N(1)
    val n2 = N(2)
    n1.neighbors = List(n2)
    n2.neighbors = List(n1)

    val result = cloneGraph(n1)

    result.toString shouldBe n1.toString
    result should not equal n1
  }

  "100 elements graph" in {
    val length = 500
    val nodes = Array.ofDim[Node](length)
    for (i <- nodes.indices) nodes(i) = N(i + 1)
    for (n <- nodes) n.neighbors = nodes.filterNot(_ == n).toList

    val n1 = nodes(0)

    val result = cloneGraph(n1)

    result.toString shouldBe n1.toString
    result should not equal n1
  }
}
