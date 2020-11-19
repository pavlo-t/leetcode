package contest.w215

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/contest/weekly-contest-215/problems/design-an-ordered-stream/]]
 */
//noinspection DuplicatedCode
class w215_1 extends AnyWordSpec with Matchers {
  /**
   * === 5601. Design an Ordered Stream ===
   *
   */
  class OrderedStream(_n: Int) {
    import collection.mutable

    private val data = Array.ofDim[String](_n + 1)
    private var pointer = 1

    def insert(id: Int, value: String): List[String] = {
      data(id) = value
      val lb = mutable.ListBuffer[String]()
      while (pointer < data.length && data(pointer) != null) {
        lb.addOne(data(pointer))
        pointer += 1
      }
      lb.toList
    }
  }

  /**
   * Your OrderedStream object will be instantiated and called as such:
   * var obj = new OrderedStream(n)
   * var param_1 = obj.insert(id,value)
   */


  """Example""" in {
    val obj = new OrderedStream(5)
    obj.insert(3, "ccccc") shouldBe List()
    obj.insert(1, "aaaaa") shouldBe List("aaaaa")
    obj.insert(2, "bbbbb") shouldBe List("bbbbb", "ccccc")
    obj.insert(5, "eeeee") shouldBe List()
    obj.insert(4, "ddddd") shouldBe List("ddddd", "eeeee")
  }
}
