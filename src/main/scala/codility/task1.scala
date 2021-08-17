package codility

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class task1 extends AnyWordSpec with Matchers {
  /**
   *
   */
  object SolutionInitial {
    def solution(X: Int, Y: Int, A: Array[Int]): Int = {
      var N: Int = A.length;
      var result: Int = -1;
      var nX: Int = 0;
      var nY: Int = 0;
      var i: Int = 0;
      while (i < N) {
        if (A(i) == X)
          nX += 1;
        else if (A(i) == Y)
          nY += 1;
        if (nX == nY)
          result = i;
        i += 1;
      }
      return result;
    }
  }

  object Solution {
    def solution(X: Int, Y: Int, A: Array[Int]): Int = {
      var N: Int = A.length;
      if (X == Y)
        return N - 1;
      var result: Int = -1;
      var nX: Int = 0;
      var nY: Int = 0;
      var i: Int = 0;
      while (i < N) {
        if (A(i) == X)
          nX += 1;
        else if (A(i) == Y)
          nY += 1;
        if (nX == nY)
          result = i;
        i += 1;
      }
      return result;
    }
  }

  "test 1" in (Solution.solution(7, 42, Array(6, 42, 11, 7, 1, 42)) shouldBe 4)
  "test 2" in (Solution.solution(6, 13, Array(13, 13, 1, 6)) shouldBe -1)
  "test 3" in (Solution.solution(100, 63, Array(100, 63, 1, 6, 2, 13)) shouldBe 5)

  "test 4" in (Solution.solution(7, 7, Array(6, 42, 11, 7, 1, 42)) shouldBe 5)
  "test 5" in (Solution.solution(7, 7, Array(7)) shouldBe 0)
  "test 6" in (Solution.solution(7, 7, Array(6)) shouldBe 0)
  "test 7" in (Solution.solution(7, 7, Array(6, 8, 1)) shouldBe 2)
}
