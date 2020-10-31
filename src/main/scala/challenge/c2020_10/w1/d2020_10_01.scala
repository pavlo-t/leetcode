package challenge.c2020_10.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class d2020_10_01 extends AnyWordSpec with Matchers {

  class RecentCounter() {
    private val calls = collection.mutable.ListBuffer[Int]()

    def ping(t: Int): Int = {
      calls addOne t
      val rangeStart = t - 3000
      calls.dropWhileInPlace(_ < rangeStart)
      calls.size
    }
  }

  /**
   * Your RecentCounter object will be instantiated and called as such:
   * var obj = new RecentCounter()
   * var param_1 = obj.ping(t)
   */

  "RecentCounter" should {
    "count 1" in {
      val recentCounter = new RecentCounter()

      // requests = [1], range is [-2999,1], return 1
      recentCounter.ping(1) shouldBe 1
      // requests = [1, 100], range is [-2900,100], return 2
      recentCounter.ping(100) shouldBe 2
      // requests = [1, 100, 3001], range is [1,3001], return 3
      recentCounter.ping(3001) shouldBe 3
      // requests = [1, 100, 3001, 3002], range is [2,3002], return 3
      recentCounter.ping(3002) shouldBe 3
    }

    "count 2" in {
      val recentCounter = new RecentCounter()

      recentCounter.ping(642) shouldBe 1
      recentCounter.ping(1849) shouldBe 2
      recentCounter.ping(4921) shouldBe 1
      recentCounter.ping(5936) shouldBe 2
      recentCounter.ping(5957) shouldBe 3
    }
  }
}
