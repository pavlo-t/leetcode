#![allow(dead_code)]
/// 71. Simplify Path
/// ======================
///
/// Given a string `path`, which is an __absolute path__ (starting with a slash `'/'`)
/// to a file or directory in a Unix-style file system,
/// convert it to the simplified __canonical path__.
///
/// In a Unix-style file system, a period `'.'` refers to the current directory,
/// a double period `'..'` refers to the directory up a level,
/// and any multiple consecutive slashes (i.e. `'//'`) are treated as a single slash `'/'`.
/// For this problem, any other format of periods such as `'...'` are treated as file/directory names.
///
/// The __canonical path__ should have the following format:
///
/// - The path starts with a single slash `'/'`.
/// - Any two directories are separated by a single slash `'/'`.
/// - The path does not end with a trailing `'/'`.
/// - The path only contains the directories on the path from the root directory to the target file or directory
///   (i.e., no period `'.'` or double period `'..'`)
///
/// Return _the simplified __canonical path___.
///
/// __Constraints:__
///
/// - `1 <= path.length <= 3000`
/// - `path` consists of English letters, digits, period `'.'`, slash `'/'` or `'_'`.
/// - `path` is a valid absolute Unix path.
///
/// https://leetcode.com/problems/simplify-path/
struct Solution;
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        for c in path.chars().chain(std::iter::once('/')) {
            match c {
                '/' if stack.is_empty() => stack.push("/".to_string()),
                '/' if stack.last().filter(|w| w == &".").is_some() => {
                    stack.pop();
                }
                '/' if stack.last().filter(|w| w == &"..").is_some() => {
                    stack.pop();
                    stack.pop();
                    if stack.is_empty() {
                        stack.push("/".to_string())
                    } else {
                        stack.pop();
                    }
                }
                '/' if stack.last().filter(|w| w == &"/").is_some() => (),
                '/' => stack.push("/".to_string()),
                c if stack.last().filter(|w| w == &"/").is_some() => stack.push(c.to_string()),
                c => {
                    let last = stack.last_mut().unwrap();
                    last.push(c);
                }
            }
        }
        if stack.len() > 1 {
            stack.pop();
        }

        let mut result = String::new();
        for w in &stack {
            result.push_str(w);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_home_s() {
        let p = "/home/".to_string();
        assert_eq!(Solution::simplify_path(p), "/home");
        // Explanation: Note that there is no trailing slash after the last directory name.
    }
    #[test]
    fn s_d_d_s() {
        let p = "/../".to_string();
        assert_eq!(Solution::simplify_path(p), "/");
        // Explanation: Going one level up from the root directory is a no-op,
        // as the root level is the highest level you can go.
    }
    #[test]
    fn s_home_s_s_foo_s() {
        let p = "/home//foo/".to_string();
        assert_eq!(Solution::simplify_path(p), "/home/foo");
        // Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
    }
    #[test]
    fn s_home_s_s_foo_s_d() {
        let p = "/home//foo/.".to_string();
        assert_eq!(Solution::simplify_path(p), "/home/foo");
    }
    #[test]
    fn s_home_s_s_foo_s_d_s() {
        let p = "/home//foo/./".to_string();
        assert_eq!(Solution::simplify_path(p), "/home/foo");
    }
    #[test]
    fn s_home_s_s_foo_s_d_d() {
        let p = "/home//foo/..".to_string();
        assert_eq!(Solution::simplify_path(p), "/home");
    }
    #[test]
    fn s_home_s_s_foo_s_d_d_s() {
        let p = "/home//foo/../".to_string();
        assert_eq!(Solution::simplify_path(p), "/home");
    }

    #[test]
    fn s_d_d_d_s_s() {
        let p = "/...//".to_string();
        assert_eq!(Solution::simplify_path(p), "/...");
    }
}
