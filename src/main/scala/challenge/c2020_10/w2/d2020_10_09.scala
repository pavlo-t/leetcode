package challenge.c2020_10.w2

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d2020_10_09 extends AnyWordSpec with Matchers {

  /**
   * Serialize and Deserialize BST
   *
   * Serialization is converting a data structure or object into a sequence of bits so that it can
   * be stored in a file or memory buffer, or transmitted across a network connection link to be
   * reconstructed later in the same or another computer environment.
   *
   * Design an algorithm to serialize and deserialize a binary search tree.
   * There is no restriction on how your serialization/deserialization algorithm should work.
   * You need to ensure that a binary search tree can be serialized to a string,
   * and this string can be deserialized to the original tree structure.
   *
   * <b>The encoded string should be as compact as possible</b>.
   *
   *
   * Your Codec object will be instantiated and called as such:
   *
   * <pre>
   * val ser = new Codec()
   * val deser = new Codec()
   * val tree: String = ser.serialize(root)
   * val ans = deser.deserialize(tree)
   * return ans
   * </pre>
   *
   *
   * <b>Constraints:</b><ul>
   * <li> The number of nodes in the tree is in the range <code>[0, 10<sup>4</sup>]</code>.
   * <li> <code>0 <= Node.val <= 10<sup>4</sup></code>
   * <li> The input tree is <b>guaranteed</b> to be a binary search tree.
   * </ul>
   */
  class Codec1 {
    // Encodes a list of strings to a single string.
    def serialize(root: TreeNode): String = {
      if (root == null) ""
      else s"${root.value}${if (root.left == null) "" else "," + serialize(root.left)}${if (root.right == null) "" else "," + serialize(root.right)}"
    }

    // Decodes a single string to a list of strings.
    def deserialize(data: String): TreeNode = {
      if (data.length == 0) null
      else {
        def loop(xs: Seq[Int]): TreeNode = {
          if (xs.isEmpty) null
          else {
            val n = new TreeNode(xs.head)
            val (ls, rs) = xs.tail.partition(_ < xs.head)
            n.left = loop(ls)
            n.right = loop(rs)
            n
          }
        }
        loop(data.split(',').map(_.toInt).toSeq)
      }
    }
  }

  class Codec {
    // Encodes a list of strings to a single string.
    def serialize(root: TreeNode): String = {
      if (root == null) ""
      else {
        val sb = new StringBuilder()

        def build(n: TreeNode): Unit = {
          if (n != null) {
            sb.append(n.value)
            if (n.left != null) {
              sb.append(',')
              build(n.left)
            }
            if (n.right != null) {
              sb.append(',')
              build(n.right)
            }
          }
        }
        build(root)

        sb.toString()
      }
    }

    // Decodes a single string to a list of strings.
    def deserialize(data: String): TreeNode = {
      if (data.length == 0) null
      else {
        def loop(xs: Seq[Int]): TreeNode = {
          if (xs.isEmpty) null
          else {
            val n = new TreeNode(xs.head)
            val (ls, rs) = xs.tail.partition(_ < xs.head)
            n.left = loop(ls)
            n.right = loop(rs)
            n
          }
        }
        loop(data.split(',').map(_.toInt).toSeq)
      }
    }
  }

  class TreeNode(var _value: Int) {
    var value: Int = _value
    var left: TreeNode = _
    var right: TreeNode = _

    override def toString: String = s"{${if (left == null) "" else left},$value,${if (right == null) "" else right}}"
  }
  private def tn(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = {
    val n = new TreeNode(v)
    n.left = l
    n.right = r
    n
  }


  "Example 1" in {
    val root = tn(2, tn(1), tn(3))

    val ser = new Codec()
    val deser = new Codec()
    val tree: String = ser.serialize(root)
    val ans = deser.deserialize(tree)

    ans.toString shouldBe root.toString
  }
  "Example 2" in {
    //val root = null

    val ser = new Codec()
    val deser = new Codec()
    val tree: String = ser.serialize(null)
    val ans = deser.deserialize(tree)

    ans shouldBe null
  }

  "My test: serialize" should {
    "null" in {
      new Codec().serialize(null) shouldBe ""
    }
    "1 el" in {
      new Codec().serialize(tn(1)) shouldBe "1"
    }
    "2 els" in {
      new Codec().serialize(tn(2, tn(1))) shouldBe "2,1"
      new Codec().serialize(tn(2, r = tn(3))) shouldBe "2,3"
    }
    "3 els" in {
      new Codec().serialize(tn(2, tn(1), tn(3))) shouldBe "2,1,3"
    }
    "7 els" in {
      val root = tn(4, tn(2, tn(1), tn(3)), tn(6, tn(5), tn(7)))
      new Codec().serialize(root) shouldBe "4,2,1,3,6,5,7"
    }
  }

  "My test: deserialize" should {
    "null" in {
      new Codec().deserialize("") shouldBe null
    }
    "1 el" in {
      new Codec().deserialize("1").toString shouldBe tn(1).toString
    }
    "2 els" in {
      new Codec().deserialize("2,1").toString shouldBe tn(2, tn(1)).toString
      new Codec().deserialize("2,3").toString shouldBe tn(2, r = tn(3)).toString
    }
    "3 els" in {
      new Codec().deserialize("2,1,3").toString shouldBe tn(2, tn(1), tn(3)).toString
    }
    "7 els" in {
      val root = tn(4, tn(2, tn(1), tn(3)), tn(6, tn(5), tn(7)))
      new Codec().deserialize("4,2,1,3,6,5,7").toString shouldBe root.toString
    }
  }
}
