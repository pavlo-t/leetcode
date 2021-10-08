package challenge.c2021.c2021_10

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/problems/implement-trie-prefix-tree/]] */
class c2021_10_08 extends AnyWordSpec with Matchers {

  class Trie() {
    private var wordEnd = false
    private val data = Array.fill[Option[Trie]](26)(None)

    def insert(word: String): Unit =
      word.map(_ - 'a').foldLeft(this) { (curr, i) =>
        if (curr.data(i).isEmpty)
          curr.data(i) = Some(new Trie())
        curr.data(i).get
      }.wordEnd = true

    def search(word: String): Boolean =
      find(word).exists(_.wordEnd)

    def startsWith(prefix: String): Boolean =
      find(prefix).isDefined

    private def find(prefix: String): Option[Trie] =
      prefix
        .map(_ - 'a')
        .foldLeft[Option[Trie]](Some(this)) { (curr, i) =>
          curr.flatMap(_.data(i))
        }
  }

  /**
   * Your Trie object will be instantiated and called as such:
   * var obj = new Trie()
   * obj.insert(word)
   * var param_2 = obj.search(word)
   * var param_3 = obj.startsWith(prefix)
   */

  "Example 1" in {
    val trie = new Trie()
    trie.insert("apple")
    trie.search("apple") shouldBe true
    trie.search("app") shouldBe false
    trie.startsWith("app") shouldBe true
    trie.insert("app")
    trie.search("app") shouldBe true
  }

  private def intToWord2000(i: Int): String = {
    val sb = new StringBuilder(2000)
    val str = Array.fill[Char](5)('a')
    i.toString.map(_ - '0' + 'a').map(_.toChar).zipWithIndex.foreach { case (c, i) => str(i) = c }
    for (_ <- 0 to 400) sb.appendAll(str)
    sb.toString()
  }

  "performance" should {
    "insert 300x2000 strings" in {
      val trie = new Trie()
      (1 to 300)
        .map(intToWord2000)
        .foreach(trie.insert)
    }

    // 3 sec 798 ms; 3.6 GiB
    "insert 10kx2k search 10kx2k startsWith 10kx2k" in {
      val trie = new Trie()
      val words = (1 to 10000).map(intToWord2000)
      words.foreach(trie.insert)
      words.map(trie.search).foreach(_ shouldBe true)
      words.map(trie.startsWith).foreach(_ shouldBe true)
    }
  }
}
