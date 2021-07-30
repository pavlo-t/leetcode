#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;

/// Map Sum Pairs
/// =============
///
/// Implement the `MapSum` class:
///
/// - `MapSum()` Initializes the `MapSum` object.
/// - `void insert(String key, int val)` Inserts the `key-val` pair into the map.
///   If the `key` already existed, the original `key-value` pair will be overridden to the new one.
/// - `int sum(string prefix)` Returns the sum of all the pairs' value whose `key` starts with the `prefix`.
///
/// __Constraints:__
///
/// - `1 <= key.length, prefix.length <= 50`
/// - `key` and `prefix` consist of only lowercase English letters.
/// - `1 <= val <= 1000`
/// - At most `50` calls will be made to `insert` and `sum`.
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/612/week-5-july-29th-july-31st/3832/
struct MapSum {
    data: RefCell<HashMap<String, i32>>,
}
impl MapSum {
    fn new() -> Self {
        let data = RefCell::new(HashMap::new());
        Self { data }
    }

    fn insert(&self, key: String, val: i32) {
        self.data.borrow_mut().insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        let d = self.data.borrow();
        d.keys().filter(|k| k.starts_with(&prefix)).map(|k| d.get(k).unwrap()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let ms = MapSum::new();
        assert_eq!(ms.sum("ap".to_string()), 0);
        ms.insert("apple".to_string(), 3);
        assert_eq!(ms.sum("ap".to_string()), 3);
        assert_eq!(ms.sum("aps".to_string()), 0);
        ms.insert("app".to_string(), 2);
        assert_eq!(ms.sum("ap".to_string()), 5);
        ms.insert("bag".to_string(), 2);
        assert_eq!(ms.sum("ap".to_string()), 5);
        assert_eq!(ms.sum("apple".to_string()), 3);
        assert_eq!(ms.sum("bag".to_string()), 2);
    }
}
