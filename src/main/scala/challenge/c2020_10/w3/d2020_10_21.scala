package challenge.c2020_10.w3

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_10_21 extends AnyWordSpec with Matchers {

  /**
   * <h3>Asteroid Collision</h3>
   *
   * We are given an array `asteroids` of integers representing asteroids in a row.
   *
   * For each asteroid, the absolute value represents its size,
   * and the sign represents its direction (positive meaning right, negative meaning left).
   * Each asteroid moves at the same speed.
   *
   * Find out the state of the asteroids after all collisions.
   * If two asteroids meet, the smaller one will explode.
   * If both are the same size, both will explode.
   * Two asteroids moving in the same direction will never meet.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= asteroids <= 10_000`
   * <li> `-1000 <= asteroids[i] <= 1000`
   * <li> `asteroids[i] != 0`
   * </ul>
   */
  object Solution {
    import scala.annotation.tailrec
    import collection.mutable

    def asteroidCollision(asteroids: Array[Int]): Array[Int] = {
      @tailrec
      def loop(as: List[Int], rsf: mutable.Stack[Int]): mutable.Stack[Int] = as match {
        case Nil       => rsf
        case x :: rest => loop(rest, updateRsf(x, rsf))
      }

      @tailrec
      def updateRsf(x: Int, rsf: mutable.Stack[Int]): mutable.Stack[Int] =
        if (rsf.isEmpty || x > 0 || rsf.top < 0) rsf.push(x)
        else if (rsf.top > -x) rsf
        else if (rsf.top == -x) {
          rsf.pop()
          rsf
        } else {
          rsf.pop()
          updateRsf(x, rsf)
        }

      loop(asteroids.toList, mutable.Stack()).toArray.reverse
    }
  }
  object Solution1 {
    import scala.annotation.tailrec

    def asteroidCollision(asteroids: Array[Int]): Array[Int] = {
      @tailrec
      def nextWorld(w: List[Int], rsf: List[Int]): List[Int] = w match {
        case Nil            => rsf
        case x :: Nil       => rsf :+ x
        case x :: y :: rest =>
          if (x < 0) nextWorld(y :: rest, rsf :+ x)
          else if (y < 0) {
            if (x == -y) nextWorld(rest, rsf)
            else if (x > -y) nextWorld(rest, rsf :+ x)
            else nextWorld(rest, rsf :+ y)
          }
          else // x > 0 && y > 0
            nextWorld(y :: rest, rsf :+ x)
      }

      @tailrec
      def getResult(c: List[Int]): List[Int] = {
        val next = nextWorld(c, List())
        if (next != c) getResult(next)
        else c
      }

      getResult(asteroids.toList).toArray
    }
  }

  import Solution.asteroidCollision

  "Example 1: ([5,10,-5]) -> [5,10]" in {
    val asteroids = Array(5, 10, -5)
    val expected = Array(5, 10)

    asteroidCollision(asteroids) shouldBe expected
    // Explanation: The 10 and -5 collide resulting in 10. The 5 and 10 never collide.
  }
  "Example 2: ([8,-8]) -> []" in {
    val asteroids = Array(8, -8)
    val expected = Array()

    asteroidCollision(asteroids) shouldBe expected
    // Explanation: The 8 and -8 collide exploding each other.
  }
  "Example 3: ([10,2,-5]) -> [10]" in {
    val asteroids = Array(10, 2, -5)
    val expected = Array(10)

    asteroidCollision(asteroids) shouldBe expected
    // Explanation: The 2 and -5 collide resulting in -5. The 10 and -5 collide resulting in 10.
  }
  "Example 4: ([-2,-1,1,2]) -> [-2,-1,1,2]" in {
    val asteroids = Array(-2, -1, 1, 2)
    val expected = Array(-2, -1, 1, 2)

    asteroidCollision(asteroids) shouldBe expected
    // Explanation: The -2 and -1 are moving left, while the 1 and 2 are moving right.
    //   Asteroids moving the same direction never meet, so no asteroids will meet each other.
  }

  "([1]) -> [1]" in {
    val asteroids = Array(1)
    asteroidCollision(asteroids) shouldBe Array(1)
  }
  "([1,-1]) -> []" in {
    val asteroids = Array(1, -1)
    asteroidCollision(asteroids) shouldBe Array()
  }
  "([-1,1]) -> [-1,1]" in {
    val asteroids = Array(-1, 1)
    asteroidCollision(asteroids) shouldBe Array(-1, 1)
  }
  "([1,1]) -> [1,1]" in {
    val asteroids = Array(1, 1)
    asteroidCollision(asteroids) shouldBe Array(1, 1)
  }
  "([1,-2]) -> [-2]" in {
    val asteroids = Array(1, -2)
    asteroidCollision(asteroids) shouldBe Array(-2)
  }
  "([-2,1]) -> [-2,1]" in {
    val asteroids = Array(-2, 1)
    asteroidCollision(asteroids) shouldBe Array(-2, 1)
  }
  "([-1,1,2,3,-1]) -> [-1,1,2,3]" in {
    val asteroids = Array(-1, 1, 2, 3, -1)
    asteroidCollision(asteroids) shouldBe Array(-1, 1, 2, 3)
  }
  "([-1,2,-2,1]) -> [-1,1]" in {
    val asteroids = Array(-1, 2, -2, 1)
    val expected = Array(-1, 1)
    asteroidCollision(asteroids) shouldBe expected
  }
  "([-1,2,-3,3,-2,1]) -> [-1,-3,3,1]" in {
    val asteroids = Array(-1, 2, -3, 3, -2, 1)
    val expected = Array(-1, -3, 3, 1)
    asteroidCollision(asteroids) shouldBe expected
  }
  "10_000 asteroids right" in {
    val len = 10_000
    val asteroids = Array.ofDim[Int](len)
    for (i <- asteroids.indices) asteroids(i) = 1

    asteroidCollision(asteroids) shouldBe asteroids
  }
  "10_000 asteroids left" in {
    val len = 10_000
    val asteroids = Array.ofDim[Int](len)
    for (i <- asteroids.indices) asteroids(i) = -1

    asteroidCollision(asteroids) shouldBe asteroids
  }
  "10_000 asteroids, none remaining" in {
    val len = 10_000
    val asteroids = Array.ofDim[Int](len)
    for (i <- asteroids.indices) {
      val a = util.Random.nextInt(1999) - 999
      asteroids(i) = if (a == 0) 1 else a
    }

    asteroids(0) = 1000
    asteroids(len - 1) = -1000

    asteroidCollision(asteroids) shouldBe Array()
  }
}
