#![allow(dead_code)]
/// 588. Design In-Memory File System
/// =================================
///
/// Design a data structure that simulates an in-memory file system.
///
/// Implement the FileSystem class:
///
/// - `FileSystem()` Initializes the object of the system.
/// - `List<String> ls(String path)`
///   - If `path` is a file path, returns a list that only contains this file's name.
///   - If `path` is a directory path, returns the list of file and directory names __in this directory__.
///   The answer should in __lexicographic order__.
/// - `void mkdir(String path)` Makes a new directory according to the given `path`.
///   The given directory path does not exist.
///   If the middle directories in the path do not exist, you should create them as well.
/// - `void addContentToFile(String filePath, String content)`
///   - If `filePath` does not exist, creates that file containing given `content`.
///   - If `filePath` already exists, appends the given `content` to original content.
/// - `String readContentFromFile(String filePath)` Returns the content in the file at `filePath`.
///
/// __Constraints:__
///
/// - `1 <= path.length, filePath.length <= 100`
/// - `path` and `filePath` are absolute paths which begin with `'/'` and do not end with `'/'`
///   except that the path is just `"/"`.
/// - You can assume that all directory names and file names only contain lowercase letters,
///   and the same names will not exist in the same directory.
/// - You can assume that all operations will be passed valid parameters,
///   and users will not attempt to retrieve file content or list a directory or file that does not exist.
/// - `1 <= content.length <= 50`
/// - At most `300` calls will be made to `ls`, `mkdir`, `addContentToFile`, and `readContentFromFile`.
///
/// https://leetcode.com/problems/design-in-memory-file-system/
#[derive(Debug)]
struct FileSystem {
    root: FS,
}
impl FileSystem {
    fn new() -> Self {
        Self {
            root: FS::Dir { data: Vec::new() },
        }
    }

    fn ls(&self, mut path: String) -> Vec<String> {
        println!("ls({})", path);
        if path.ends_with('/') {
            path.pop();
        }
        let mut curr = &self.root;
        let mut curr_p: &str = "/";
        for p in path.split('/').skip(1) {
            curr = match curr {
                FS::Dir { data } => {
                    let i = data.binary_search_by_key(&p, |(name, _)| name).unwrap();
                    &data[i].1
                }
                _ => unreachable!(),
            };
            curr_p = p;
        }
        match curr {
            FS::Dir { data } => data.iter().map(|(name, _)| name.clone()).collect(),
            FS::File { .. } => vec![curr_p.to_string()],
        }
    }

    fn mkdir(&mut self, path: String) {
        println!("mkdir({})", path);
        let mut curr = &mut self.root;
        for p in path.split('/').skip(1) {
            curr = match curr {
                FS::Dir { data } => match data.binary_search_by_key(&p, |(name, _)| name) {
                    Ok(i) => &mut data[i].1,
                    Err(i) => {
                        let name = p.to_string();
                        let dir = FS::Dir { data: vec![] };
                        data.insert(i, (name, dir));
                        &mut data[i].1
                    }
                },
                _ => unreachable!(),
            };
        }
    }

    fn add_content_to_file(&mut self, file_path: String, content: String) {
        println!("add_content_to_file({}, {})", file_path, content);
        let mut curr = &mut self.root;
        for p in file_path.split('/').skip(1) {
            curr = match curr {
                FS::Dir { data } => match data.binary_search_by_key(&p, |(name, _)| name) {
                    Ok(i) => &mut data[i].1,
                    Err(i) => {
                        let name = p.to_string();
                        let file = FS::File {
                            contents: String::new(),
                        };
                        data.insert(i, (name, file));
                        &mut data[i].1
                    }
                },
                _ => unreachable!(),
            };
        }

        match curr {
            FS::File { contents } => contents.push_str(&content),
            _ => unreachable!(),
        }
    }

    fn read_content_from_file(&self, file_path: String) -> String {
        println!("read_content_from_file({})", file_path);
        let mut curr = &self.root;
        for p in file_path.split('/').skip(1) {
            curr = match curr {
                FS::Dir { data } => {
                    let i = data.binary_search_by_key(&p, |(name, _)| name).unwrap();
                    &data[i].1
                }
                _ => unreachable!(),
            };
        }

        match curr {
            FS::File { contents } => contents.clone(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum FS {
    File { contents: String },
    Dir { data: Vec<(String, FS)> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut fs = FileSystem::new();
        assert_eq!(fs.ls("/".into()).len(), 0);
        fs.mkdir("/a/b/c".into());
        assert_eq!(fs.ls("/".into()), ["a"]);
        assert_eq!(fs.ls("/a/b".into()), ["c"]);

        fs.mkdir("/a/b/c/b".into());
        fs.mkdir("/a/b/c/a".into());
        fs.mkdir("/a/b/c/e".into());
        assert_eq!(fs.ls("/a/b/c".into()), ["a", "b", "e"]);

        fs.add_content_to_file("/a/b/c/d".into(), "hello".into());
        assert_eq!(fs.ls("/".into()), ["a"]);
        assert_eq!(fs.ls("/a".into()), ["b"]);
        assert_eq!(fs.ls("/a/b/c".into()), ["a", "b", "d", "e"]);
        assert_eq!(fs.read_content_from_file("/a/b/c/d".into()), "hello");
    }

    #[test]
    fn test_12() {
        let mut fs = FileSystem::new();
        fs.mkdir("/goowmfn".into());
        assert_eq!(fs.ls("/goowmfn".into()).len(), 0);
        assert_eq!(fs.ls("/".into()), ["goowmfn"]);
        fs.mkdir("/z".into());
        assert_eq!(fs.ls("/".into()), ["goowmfn", "z"]);
        assert_eq!(fs.ls("/".into()), ["goowmfn", "z"]);
        fs.add_content_to_file("/goowmfn/c".into(), "shetopcy".into());
        assert_eq!(fs.ls("/z".into()).len(), 0);
        assert_eq!(fs.ls("/goowmfn/c".into()), ["c"]);
        assert_eq!(fs.ls("/goowmfn".into()), ["c"]);
    }
}
