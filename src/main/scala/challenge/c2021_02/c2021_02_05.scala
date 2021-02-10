package challenge.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3629/]]
 */
class c2021_02_05 extends AnyWordSpec with Matchers {
  /**
   * === Simplify Path ===
   *
   * Given a string `path`, which is an '''absolute path''' (starting with a slash '/') to a file or directory
   * in a Unix-style file system, convert it to the simplified '''canonical path'''.
   *
   * In a Unix-style file system,
   * a period `'.'` refers to the current directory,
   * a double period `'..'` refers to the directory up a level,
   * and any multiple consecutive slashes (i.e. `'//'`) are treated as a single slash `'/'`.
   * For this problem, any other format of periods such as `'...'` are treated as file/directory names.
   *
   * The '''canonical path''' should have the following format:
   *  - The path starts with a single slash `'/'`.
   *  - Any two directories are separated by a single slash `'/'`.
   *  - The path does not end with a trailing `'/'`.
   *  - The path only contains the directories on the path from the root directory to the target file or directory
   *    (i.e., no period `'.'` or double period `'..'`)
   *
   * Return ''the simplified '''canonical path'''''.
   *
   * '''Constraints:'''
   *  - `1 <= path.length <= 3000`
   *  - `path` consists of English letters, digits, period `'.'`, slash `'/'` or `'_'`.
   *  - `path` is a valid absolute Unix path.
   */
  object Solution {
    def simplifyPath(path: String): String = {
      @scala.annotation.tailrec
      def process(p: Seq[String], rsf: Seq[String]): Seq[String] = p match {
        case Nil                => rsf
        case ("." | "") +: rest => process(rest, rsf)
        case ".." +: rest       => process(rest, rsf.dropRight(1))
        case dir +: rest        => process(rest, rsf :+ dir)
      }
      process(path.split("/").toSeq, Seq()).mkString("/", "/", "")
    }
  }

  import Solution.simplifyPath

  """Example 1: (path = "/home/") -> "/home"""" in {
    simplifyPath("/home/") shouldBe "/home"
    //Explanation: Note that there is no trailing slash after the last directory name.
  }
  """Example 2: (path = "/../") -> "/"""" in {
    simplifyPath("/../") shouldBe "/"
    //Explanation:
    // Going one level up from the root directory is a no-op, as the root level is the highest level you can go.
  }
  """Example 3: (path = "/home//foo/") -> "/home/foo"""" in {
    simplifyPath("/home//foo/") shouldBe "/home/foo"
    //Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
  }
  """Example 4: (path = "/a/./b/../../c/") -> "/c"""" in {
    simplifyPath("/a/./b/../../c/") shouldBe "/c"
  }

  """(path = "/"*3000) -> "/"""" in {
    simplifyPath("/" * 3000) shouldBe "/"
  }
  """(path = "/a"*1500) -> "/a"*1500""" in {
    simplifyPath("/a" * 1500) shouldBe "/a" * 1500
  }
}
