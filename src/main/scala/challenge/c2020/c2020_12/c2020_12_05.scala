package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/569/week-1-december-1st-december-7th/3555/]]
 */
class c2020_12_05 extends AnyWordSpec with Matchers {

  /**
   * === Can Place Flowers ===
   *
   * You have a long flowerbed in which some of the plots are planted, and some are not.
   * However, flowers cannot be planted in '''adjacent''' plots.
   *
   * Given an integer array `flowerbed` containing `0`'s and `1`'s,
   * where `0` means empty and `1` means not empty, and an integer `n`, return
   * ''if ''`n`'' new flowers can be planted in the ''`flowerbed`'' without violating the no-adjacent-flowers rule''.
   *
   * '''Constraints:'''
   *  - `1 <= flowerbed.length <= 20000`
   *  - `flowerbed[i]` is `0` or `1`.
   *  - There are no two adjacent flowers in `flowerbed.`
   *  - `0 <= n <= flowerbed.length`
   */
  object Solution {
    def canPlaceFlowers(flowerbed: Array[Int], n: Int): Boolean =
      flowerbed.foldLeft((0, 0)) { case ((planted, last), current) =>
        if (planted > n)
          return true
        (last, current) match {
          case (0, 0) => (planted + 1, 1)
          case (1, 1) => (planted - 1, 1)
          case (1, 0) => (planted, 0)
          case (0, 1) => (planted, 1)

          case (_, _) => throw new IllegalStateException(s"Unexpected value in flowerbed: $current")
        }
      }._1 >= n
  }

  object SolutionSlide3Fold {
    def canPlaceFlowers(flowerbed: Array[Int], n: Int): Boolean =
      if (n == 0) true
      else if (flowerbed.length == 1) n == 1 && flowerbed(0) == 0
      else if (flowerbed.length == 2) n == 1 && (flowerbed(0) == 0 || flowerbed(1) == 1)
      else {
        val init = flowerbed match {
          case Array(0, 0, _*) => (1, true)
          case _               => (0, false)
        }
        flowerbed
          .sliding(3)
          .foldLeft(init) { case ((planted, plantedOnLast), spots) =>
            if (planted == n) return true
            else if (!plantedOnLast && spots.forall(_ == 0)) (planted + 1, true)
            else (planted, false)
          } match {
          case (planted, true)  => planted >= n
          case (planted, false) =>
            flowerbed.takeRight(2) match {
              case Array(0, 0) => (planted + 1) >= n
              case _           => planted >= n
            }
        }
      }
  }

  import Solution.canPlaceFlowers

  "Example 1: (flowerbed = [1,0,0,0,1], n = 1) -> true" in {
    canPlaceFlowers(Array(1, 0, 0, 0, 1), 1) shouldBe true
  }
  "Example 2: (flowerbed = [1,0,0,0,1], n = 2) -> false" in {
    canPlaceFlowers(Array(1, 0, 0, 0, 1), 2) shouldBe false
  }

  "Test 105: (flowerbed = [0,0], n = 2) -> false" in {
    canPlaceFlowers(Array(0, 0), 2) shouldBe false
  }
  "Test 106: (flowerbed = [0,0,0,0,0], n = 4) -> false" in {
    canPlaceFlowers(Array(0, 0, 0, 0, 0), 4) shouldBe false
  }
  "Test 120: (flowerbed = [1,0,0,0,0,1], n = 2) -> false" in {
    canPlaceFlowers(Array(1, 0, 0, 0, 0, 1), 2) shouldBe false
  }

  "(flowerbed = [0,0,0,0,0], n = 3) -> true" in {
    canPlaceFlowers(Array(0, 0, 0, 0, 0), 3) shouldBe true
  }
  "(flowerbed = [0,0], n = 1) -> true" in {
    canPlaceFlowers(Array(0, 0), 1) shouldBe true
  }
  "(flowerbed = [0,1], n = 1) -> false" in {
    canPlaceFlowers(Array(0, 1), 1) shouldBe false
  }
  "(flowerbed = [1,0], n = 1) -> false" in {
    canPlaceFlowers(Array(1, 0), 1) shouldBe false
  }

  "(flowerbed = [20k 0s], n = 9999) -> true" in {
    canPlaceFlowers(Array.fill(20000)(0), 9999) shouldBe true
  }
  "(flowerbed = [20k 0s], n = 10k) -> true" in {
    canPlaceFlowers(Array.fill(20000)(0), 10000) shouldBe true
  }
  "(flowerbed = [19999 0s], n = 10k) -> true" in {
    canPlaceFlowers(Array.fill(19999)(0), 10000) shouldBe true
  }
  "(flowerbed = [20k 0s], n = 10_001) -> false" in {
    canPlaceFlowers(Array.fill(20000)(0), 10001) shouldBe false
  }

}
