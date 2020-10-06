package c2020_04.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d05 extends AnyWordSpec with Matchers {

  /**
   * Best Time to Buy and Sell Stock II
   *
   * Say you have an array `prices` for which the <em>i</em><sup>th</sup> element is the price of a given stock on day <em>i</em>.
   *
   * Design an algorithm to find the maximum profit.
   * You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times).
   *
   * <b>Note</b>: You may not engage in multiple transactions at the same time
   * (i.e., you must sell the stock before you buy again).
   *
   * <b>Constraints</b>:<ul>
   * <li> `1 <= prices.length <= 3 * 10 ^ 4`
   * <li> `0 <= prices[i] <= 10 ^ 4`
   */
  object Solution {
    def maxProfitMy(prices: Array[Int]): Int =
      if (prices.length < 2) 0
      else {
        var result = 0
        var buyDate: Option[Int] = None

        for {
          i <- 0 until (prices.length - 1)
          j = i + 1
        } buyDate match {
          case None    =>
            if (prices(j) > prices(i))
              buyDate = Some(i)
          case Some(k) =>
            if (prices(j) < prices(i)) {
              buyDate = None
              result += (prices(i) - prices(k))
            }
        }
        if (buyDate.nonEmpty)
          result += (prices(prices.length - 1) - prices(buyDate.get))

        result
      }

    def maxProfit(prices: Array[Int]): Int = {
      var result = 0
      for (i <- 1 until prices.length)
        if (prices(i) > prices(i - 1))
          result += prices(i) - prices(i - 1)

      result
    }
  }

  "Example 1" in {
    Solution.maxProfit(Array(7, 1, 5, 3, 6, 4)) shouldBe 7
    // Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
    //   Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
  }
  "Example 2" in {
    Solution.maxProfit(Array(1, 2, 3, 4, 5)) shouldBe 4
    // Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
    //   Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
    //   engaging multiple transactions at the same time. You must sell before buying again.
  }
  "Example 3" in {
    Solution.maxProfit(Array(7, 6, 4, 3, 1)) shouldBe 0
    // Explanation: In this case, no transaction is done, i.e. max profit = 0.
  }

  "My test: empty array" in {
    Solution.maxProfit(Array()) shouldBe 0
  }
  "My test: 1 element array" in {
    Solution.maxProfit(Array(1)) shouldBe 0
  }
  "My test: array of 30,000 elements" in {
    val length = 30000
    val arr: Array[Int] = Array.ofDim[Int](length)
    for (i <- 0 until length) arr(i) = i

    Solution.maxProfit(arr) shouldBe 29999
  }
}
