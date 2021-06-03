#![allow(dead_code)]
/// Find Duplicate File in System
/// =============================
///
/// Given a list `paths` of directory info, including the directory path,
/// and all the files with contents in this directory,
/// return _all the duplicate files in the file system in terms of their paths_.
/// You may return the answer in __any order__.
///
/// A group of duplicate files consists of at least two files that have the same content.
///
/// A single directory info string in the input list has the following format:
///
/// - `"root/d1/d2/.../dm f1.txt(f1_content) f2.txt(f2_content) ... fn.txt(fn_content)"`
///
/// It means there are `n` files `(f1.txt, f2.txt ... fn.txt)`
/// with content `(f1_content, f2_content ... fn_content)`
/// respectively in the directory `"root/d1/d2/.../dm"`.
/// Note that `n >= 1` and `m >= 0`.
/// If `m = 0`, it means the directory is just the root directory.
///
/// The output is a list of groups of duplicate file paths.
/// For each group, it contains all the file paths of the files that have the same content.
/// A file path is a string that has the following format:
///
/// - `"directory_path/file_name.txt"`
///
/// __Constraints:__
///
/// - `1 <= paths.length <= 20_000`
/// - `1 <= paths[i].length <= 3000`
/// - `1 <= sum(paths[i].length) <= 500_000`
/// - `paths[i]` consist of English letters, digits, `'/'`, `'.'`, `'('`, `')'`, and `' '`.
/// - You may assume no files or directories share the same name in the same directory.
/// - You may assume each given directory info represents a unique directory.
///   A single blank space separates the directory path and file info.
///
///
/// __Follow up:__
///
/// - Imagine you are given a real file system, how will you search files? DFS or BFS?
/// - If the file content is very large (GB level), how will you modify your solution?
/// - If you can only read the file by 1kb each time, how will you modify your solution?
/// - What is the time complexity of your modified solution?
///   What is the most time-consuming part and memory-consuming part of it? How to optimize?
/// - How to make sure the duplicated files you find are not false positive?
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/600/week-3-may-15th-may-21st/3747/
struct Solution;
impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut groups_by_content = HashMap::new();

        paths.into_iter().for_each(|p| {
            let mut ss = p.split(' ');
            let dir = ss.next().unwrap().to_string();
            for fc in ss {
                let mut fc = fc.split('(');
                let mut file = dir.clone();
                file.push('/');
                file.push_str(fc.next().unwrap());
                let content = fc.next().unwrap().to_string();

                groups_by_content
                    .entry(content)
                    .or_insert(vec![])
                    .push(file);
            }
        });

        groups_by_content
            .into_iter()
            .map(|(_, v)| v)
            .filter(|v| v.len() > 1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    macro_rules! vs { ($($s:tt),*) => { vec![$($s.to_string()),*] } }
    macro_rules! vvs { ($($x:tt),*) => { vec![$(vs!$x),*] } }

    #[test]
    fn example1() {
        let paths = vs![
            "root/a 1.txt(abcd) 2.txt(efgh)",
            "root/c 3.txt(abcd)",
            "root/c/d 4.txt(efgh)",
            "root 4.txt(efgh)"
        ];
        let r = Solution::find_duplicate(paths)
            .into_iter()
            .collect::<HashSet<_>>();

        let e = vvs![
            ["root/a/2.txt", "root/c/d/4.txt", "root/4.txt"],
            ["root/a/1.txt", "root/c/3.txt"]
        ]
        .into_iter()
        .collect::<HashSet<_>>();

        assert_eq!(r, e);
    }
    #[test]
    fn example2() {
        let paths = vs![
            "root/a 1.txt(abcd) 2.txt(efgh)",
            "root/c 3.txt(abcd)",
            "root/c/d 4.txt(efgh)"
        ];
        let r = Solution::find_duplicate(paths)
            .into_iter()
            .collect::<HashSet<_>>();

        let e = vvs![
            ["root/a/2.txt", "root/c/d/4.txt"],
            ["root/a/1.txt", "root/c/3.txt"]
        ]
        .into_iter()
        .collect::<HashSet<_>>();

        assert_eq!(r, e);
    }
}
