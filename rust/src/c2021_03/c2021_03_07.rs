#![allow(dead_code)]

/// # Design HashMap
///
/// Design a HashMap without using any built-in hash table libraries.
///
/// To be specific, your design should include these functions:
///
/// - `put(key, value)`: Insert a (key, value) pair into the HashMap.
///   If the value already exists in the HashMap, update the value.
/// - `get(key)`: Returns the value to which the specified key is mapped,
///   or -1 if this map contains no mapping for the key.
/// - `remove(key)`: Remove the mapping for the value key if this map contains the mapping for the key.
///
/// __Note:__
///
/// - All keys and values will be in the range of `[0, 1_000_000]`.
/// - The number of operations will be in the range of `[1, 10_000]`.
/// - Please do not use the built-in HashMap library.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3663/
struct MyHashMap {
    hash_table: Vec<Bucket>,
    key_space: usize,
}

impl MyHashMap {
    fn new() -> Self {
        let key_space = 2069;
        Self {
            hash_table: vec![Bucket::new(); key_space],
            key_space,
        }
    }
    fn put(&mut self, key: i32, value: i32) {
        let hash = self.hash(key);
        self.hash_table[hash].update(key, value);
    }
    fn get(&self, key: i32) -> i32 {
        let hash = self.hash(key);
        self.hash_table[hash].get(key)
    }
    fn remove(&mut self, key: i32) {
        let hash = self.hash(key);
        self.hash_table[hash].delete(key);
    }

    fn hash(&self, key: i32) -> usize {
        key as usize % self.key_space
    }
}

#[derive(Clone)]
struct Bucket {
    bucket: Vec<(i32, i32)>,
}

impl Bucket {
    fn new() -> Self {
        Self { bucket: Vec::new() }
    }
    fn get(&self, key: i32) -> i32 {
        for (k, v) in &self.bucket {
            if k == &key {
                return *v;
            }
        }
        -1
    }
    fn update(&mut self, key: i32, value: i32) {
        for (k, v) in self.bucket.iter_mut() {
            if k == &key {
                *v = value;
                return;
            }
        }
        self.bucket.push((key, value));
    }
    fn delete(&mut self, key: i32) {
        if let Some((i, _)) = self.bucket.iter().enumerate().find(|(_, (k, _))| k == &key) {
            self.bucket.remove(i);
        }
    }
}

struct MyHashMapArray {
    data: [i32; 1000001],
}

impl MyHashMapArray {
    fn new() -> Self {
        Self {
            data: [-1; 1000001],
        }
    }
    fn put(&mut self, key: i32, value: i32) {
        self.data[key as usize] = value;
    }
    fn get(&self, key: i32) -> i32 {
        self.data[key as usize]
    }
    fn remove(&mut self, key: i32) {
        self.data[key as usize] = -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut hash_map = MyHashMap::new();
        hash_map.put(1, 1);
        hash_map.put(2, 2);
        assert_eq!(hash_map.get(1), 1);
        assert_eq!(hash_map.get(2), 2);
        assert_eq!(hash_map.get(3), -1);
        hash_map.put(2, 1);
        assert_eq!(hash_map.get(2), 1);
        hash_map.remove(2);
        assert_eq!(hash_map.get(2), -1);
        // hash_map.put(1_000_000, 7);
        // assert_eq!(hash_map.get(1_000_000), 7);
    }

    #[test]
    fn test_performance() {
        let from = 995_000;
        let to = 1_000_000;

        let mut map = MyHashMap::new();
        for i in from..to {
            map.put(i, i - from)
        }
        for i in from..to {
            assert_eq!(map.get(i), i - from);
        }
        for i in from..to {
            map.put(i, i - from + 100_000);
        }
        for i in from..to {
            assert_eq!(map.get(i), i - from + 100_000);
        }
        for i in from..to {
            map.remove(i);
        }
    }
}
