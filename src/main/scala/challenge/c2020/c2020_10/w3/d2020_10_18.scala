package challenge.c2020.c2020_10.w3

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_10_18 extends AnyWordSpec with Matchers {

  /**
   * <h3>Best Time to Buy and Sell Stock IV</h3>
   *
   * You are given an integer array `prices` where `prices[i]` is the price of a given stock on the `i`th day.
   *
   * Design an algorithm to find the maximum profit.
   * You may complete at most `k` transactions.
   *
   * Notice that you may not engage in multiple transactions simultaneously
   * (i.e., you must sell the stock before you buy again).
   *
   *
   * <b>Constraints:</b><ul>
   * <li> `0 <= k <= 10**9`
   * <li> `0 <= prices.length <= 10**4`
   * <li> `0 <= prices[i] <= 1000`
   * </ul>
   */
  object Solution {
    def maxProfit(k: Int, prices: Array[Int]): Int = {
      if (prices.length < 2) 0
      else if (k > prices.length / 2)
        (1 until prices.length).map(i => math.max(0, prices(i) - prices(i - 1))).sum
      else {
        val ts = math.min(k, prices.length / 2)
        val pnl = Array.ofDim[Int](prices.length)

        for (_ <- 1 to ts) {
          var profit = 0
          // Kadane's Algorithm
          for (i <- 1 until prices.length) {
            profit = math.max(pnl(i), profit + prices(i) - prices(i - 1))
            pnl(i) = math.max(pnl(i - 1), profit)
          }
        }

        pnl.last
      }
    }
  }
  object SolutionWrong {
    def maxProfit(k: Int, prices: Array[Int]): Int = {
      if (k <= 0) 0
      else {
        var si: Option[Int] = None
        val profits = collection.mutable.Buffer[Int]()

        for (i <- prices.indices) {
          val j = i + 1
          si match {
            case None if j < prices.length && prices(i) < prices(j)     => si = Some(i)
            case Some(k) if j >= prices.length || prices(i) > prices(j) =>
              profits += prices(i) - prices(k)
              si = None
            case _                                                      =>
          }
        }

        profits.sorted.takeRight(k).sum
      }
    }
  }

  import Solution.maxProfit

  "Example 1: (k = 2, [2,4,1]) -> 2" in {
    maxProfit(2, Array(2, 4, 1)) shouldBe 2
    // Explanation: Buy on day 1 (price = 2) and sell on day 2 (price = 4), profit = 4-2 = 2.
  }
  "Example 2: (k = 2, [3,2,6,5,0,3]) -> 7" in {
    maxProfit(2, Array(3, 2, 6, 5, 0, 3)) shouldBe 7
    //Explanation:
    //  Buy on day 2 (price = 2) and sell on day 3 (price = 6), profit = 6-2 = 4.
    //  Then buy on day 5 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
  }

  "Test 202: (k = 2, [1,2,4,2,5,7,2,4,9,0]) -> 13" in {
    val prices = Array(1, 2, 4, 2, 5, 7, 2, 4, 9, 0)
    maxProfit(2, prices) shouldBe 13
  }

  "(k = 1, [1,2,4,2,5,7,2,4,9,0]) -> 8" in {
    val prices = Array(1, 2, 4, 2, 5, 7, 2, 4, 9, 0)
    maxProfit(1, prices) shouldBe 8
  }
  "(k = 1, [9,2,4,2,5,7,2,4,9,0]) -> 7" in {
    val prices = Array(9, 2, 4, 2, 5, 7, 2, 4, 9, 0)
    maxProfit(1, prices) shouldBe 7
  }
  "(k = 3, [1,2,4,2,5,7,2,4,9,0]) -> 15" in {
    val prices = Array(1, 2, 4, 2, 5, 7, 2, 4, 9, 0)
    maxProfit(3, prices) shouldBe 15
  }
  "(k = 100, [1,2,4,2,5,7,2,4,9,0]) -> 15" in {
    val prices = Array(1, 2, 4, 2, 5, 7, 2, 4, 9, 0)
    maxProfit(100, prices) shouldBe 15
  }

  "(k = 2, []) -> 0" in {
    maxProfit(2, Array()) shouldBe 0
  }
  "(k = 0, [1,2]) -> 0" in {
    val prices = Array(1, 2)
    maxProfit(0, prices) shouldBe 0
  }
  "(k = 2, [1,2]) -> 1" in {
    val prices = Array(1, 2)
    maxProfit(2, prices) shouldBe 1
  }
  "(k = 2, [1,2,3]) -> 2" in {
    val prices = Array(1, 2, 3)
    maxProfit(2, prices) shouldBe 2
  }
  "(k = 2, [1,2,1,2]) -> 2" in {
    val prices = Array(1, 2, 1, 2)
    maxProfit(2, prices) shouldBe 2
  }
  "(k = 1, [1,2,1,2]) -> 1" in {
    val prices = Array(1, 2, 1, 2)
    maxProfit(1, prices) shouldBe 1
  }
  "(k = 1, [1,3,1,2]) -> 1" in {
    val prices = Array(1, 3, 1, 2)
    maxProfit(1, prices) shouldBe 2
  }
  "(k = 1, [1,2,1,3]) -> 1" in {
    val prices = Array(1, 2, 1, 3)
    maxProfit(1, prices) shouldBe 2
  }
  "(k = 1, [1,2,3,4]) -> 3" in {
    val prices = Array(1, 2, 3, 4)
    maxProfit(1, prices) shouldBe 3
  }
  "(k = 10**9, [1,2,1,2..]10**4) -> 5000" in {
    val length = 10000
    val prices = Array.ofDim[Int](length)
    for (i <- prices.indices) {
      prices(i) = if (i % 2 == 0) 1 else 2
    }
    maxProfit(1_000_000_000, prices) shouldBe 5000
  }
  "(k = 1, [1,2,1,2..]10**4) -> 1" in {
    val length = 10000
    val prices = Array.ofDim[Int](length)
    for (i <- prices.indices) {
      prices(i) = if (i % 2 == 0) 1 else 2
    }
    maxProfit(1, prices) shouldBe 1
  }

}
