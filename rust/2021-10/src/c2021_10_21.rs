#![allow(dead_code)]
use rand::prelude::*;
use std::collections::HashMap;
/// 380. Insert Delete GetRandom O(1)
/// =================================
///
/// Implement the `RandomizedSet` class:
///
/// - `RandomizedSet()`      Initializes the `RandomizedSet` object.
/// - `bool insert(int val)` Inserts an item `val` into the set if not present.
///                          Returns `true` if the item was not present, `false` otherwise.
/// - `bool remove(int val)` Removes an item `val` from the set if present.
///                          Returns `true` if the item was present, `false` otherwise.
/// - `int getRandom()`      Returns a random element from the current set of elements
///                          (it's guaranteed that at least one element exists when this method is called).
///                          Each element must have the __same probability__ of being returned.
///
/// You must implement the functions of the class such that each function works in __average__ `O(1)` time complexity.
///
/// __Constraints:__
///
/// - `-2^31 <= val <= 2^31 - 1`
/// - At most `200_000` calls will be made to `insert`, `remove`, and `getRandom`.
/// - There will be __at least one__ element in the data structure when `getRandom` is called.
///
/// https://leetcode.com/problems/insert-delete-getrandom-o1/
struct RandomizedSet {
    vec: Vec<i32>,
    map: HashMap<i32, usize>,
    rng: rand::rngs::ThreadRng,
}
impl RandomizedSet {
    #[rustfmt::skip]
    fn new() -> Self { Self { vec: vec![], map: HashMap::new(), rng: rand::thread_rng() } }
    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            false
        } else {
            self.map.insert(val, self.vec.len());
            self.vec.push(val);
            true
        }
    }
    fn remove(&mut self, val: i32) -> bool {
        if let Some(i) = self.map.remove(&val) {
            if i == self.vec.len() - 1 {
                self.vec.pop();
            } else {
                let last = self.vec.pop().unwrap();
                self.map.insert(last, i);
                self.vec[i] = last;
            }
            true
        } else {
            false
        }
    }
    fn get_random(&mut self) -> i32 {
        let i = self.rng.gen::<usize>();
        self.vec[i % self.vec.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut rs = RandomizedSet::new();
        assert!(rs.insert(1)); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
        assert!(!rs.remove(2)); // Returns false as 2 does not exist in the set.
        assert!(rs.insert(2)); // Inserts 2 to the set, returns true. Set now contains [1,2].
        assert!(vec![1, 2].contains(&rs.get_random())); // getRandom() should return either 1 or 2 randomly.
        assert!(rs.remove(1)); // Removes 1 from the set, returns true. Set now contains [2].
        assert!(!rs.insert(2)); // 2 was already in the set, so return false.
        assert_eq!(rs.get_random(), 2); // Since 2 is the only number in the set, getRandom() will always return 2.
        assert!(rs.remove(2)); // Removes 2 from the set, returns true. Set now is empty [].
    }

    mod performance {
        use super::*;

        //#[ignore]
        #[test]
        fn insert_100k_get_random_100k_remove_100k() {
            let mut rs = RandomizedSet::new();
            for i in 0..50000 {
                assert!(rs.insert(i));
            }
            for i in 0..50000 {
                assert!(!rs.insert(i));
            }
            for _ in 0..50000 {
                let rand = rs.get_random();
                assert!(rand >= 0 && rand < 50000);
            }
            for i in 0..50000 {
                assert!(rs.remove(i));
            }
        }
    }
}
