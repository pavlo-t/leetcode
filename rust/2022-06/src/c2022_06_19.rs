#![allow(dead_code)]
//! \#1268. Search Suggestions System
//! =================================
//!
//! You are given an array of strings `products` and a string `searchWord`.
//!
//! Design a system that suggests at most three product names from `products`
//! after each character of `searchWord` is typed.
//! Suggested products should have common prefix with `searchWord`.
//! If there are more than three products with a common prefix return the three lexicographically minimums products.
//!
//! Return _a list of lists of the suggested products after each character of `searchWord` is typed_.
//!
//! __Constraints:__
//!
//! - `1 <= products.length <= 1000`
//! - `1 <= products[i].length <= 3000`
//! - `1 <= sum(products[i].length) <= 20_000`
//! - All the strings of `products` are __unique__.
//! - `products[i]` consists of lowercase English letters.
//! - `1 <= searchWord.length <= 1000`
//! - `searchWord` consists of lowercase English letters.
//!
//! <https://leetcode.com/problems/search-suggestions-system>

pub struct Solution;
impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        #[derive(Default)]
        struct Trie {
            next: [Option<Box<Trie>>; 26],
            is_end: bool,
            words: Vec<usize>,
        }
        impl Trie {
            fn new(products: &[String]) -> Self {
                let mut result = Self::default();
                for (i, p) in products.iter().enumerate() {
                    result.add(&p, i);
                }
                result
            }
            fn add(&mut self, w: &str, wi: usize) {
                let mut curr = self;
                for i in w.bytes().map(|b| (b - b'a') as usize) {
                    if curr.next[i].is_none() {
                        curr.next[i] = Some(Box::new(Self::default()))
                    }
                    curr = curr.next[i].as_mut().unwrap();
                    curr.words.push(wi);
                }
                curr.is_end = true;
            }
            fn suggest(&self, w: &str) -> Vec<Vec<usize>> {
                let mut curr = self;
                let mut results = vec![];
                let mut stopped = false;
                for i in w.bytes().map(|b| (b - b'a') as usize) {
                    if !stopped && curr.next[i].is_some() {
                        curr = curr.next[i].as_ref().unwrap();
                        results.push(curr.words.iter().take(3).map(|&i| i).collect());
                    } else {
                        stopped = true;
                        results.push(vec![]);
                    }
                }
                results
            }
        }

        products.sort_unstable();
        let trie = Trie::new(&products);
        trie.suggest(&search_word)
            .iter()
            .map(|results| {
                results
                    .iter()
                    .map(|&i| products[i].clone())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($x:expr),*) => { vec![$($x.to_string()),*] };}
    macro_rules! vvs {($($x:tt),*) => { vec![$(vs!$x),*] };}

    #[test]
    fn mobile_mouse_moneypot_monitor_mousepad_s_mouse() {
        let p = vs!["mobile", "mouse", "moneypot", "monitor", "mousepad"];
        let s = "mouse".to_string();
        let e = vvs![
            ["mobile", "moneypot", "monitor"],
            ["mobile", "moneypot", "monitor"],
            ["mouse", "mousepad"],
            ["mouse", "mousepad"],
            ["mouse", "mousepad"]
        ];
        assert_eq!(Solution::suggested_products(p, s), e);
        // Explanation: products sorted lexicographically = ["mobile","moneypot","monitor","mouse","mousepad"]
        // After typing m and mo all products match and we show user ["mobile","moneypot","monitor"]
        // After typing mou, mous and mouse the system suggests ["mouse","mousepad"]
    }
    #[test]
    fn havana_s_havana() {
        let p = vs!["havana"];
        let s = "havana".to_string();
        let r = vvs![
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"]
        ];
        assert_eq!(Solution::suggested_products(p, s), r);
    }
    #[test]
    fn bags_baggage_banner_box_cloths_s_bags() {
        let p = vs!["bags", "baggage", "banner", "box", "cloths"];
        let s = "bags".to_string();
        let e = vvs![
            ["baggage", "bags", "banner"],
            ["baggage", "bags", "banner"],
            ["baggage", "bags"],
            ["bags"]
        ];
        assert_eq!(Solution::suggested_products(p, s), e);
    }
    #[test]
    fn bags_baggage_banner_box_cloths_s_bagdad() {
        let p = vs!["bags", "baggage", "banner", "box", "cloths"];
        let s = "bagdad".to_string();
        let e = vvs![
            ["baggage", "bags", "banner"],
            ["baggage", "bags", "banner"],
            ["baggage", "bags"],
            [],
            [],
            []
        ];
        assert_eq!(Solution::suggested_products(p, s), e);
    }

    fn read_data(file: &str) -> (Vec<String>, String, Vec<Vec<String>>) {
        let contents = std::fs::read_to_string(file).expect("NO FILE");
        let mut p = vec![];
        let mut s = String::new();
        let mut e = vec![];
        let (mut done_p, mut done_s) = (false, false);
        let mut curr = String::new();
        let mut curr_arr = vec![];
        for c in contents.chars() {
            if !done_p {
                match c {
                    '"' if !curr.is_empty() => {
                        p.push(curr);
                        curr = String::new();
                    }
                    ']' => done_p = true,
                    '[' | ',' | '"' => (),
                    c => curr.push(c),
                }
            } else if !done_s {
                match c {
                    '\n' => (),
                    '"' if s.is_empty() => (),
                    '"' => done_s = true,
                    c => s.push(c),
                }
            } else {
                match c {
                    ']' => {
                        e.push(curr_arr);
                        curr_arr = Vec::new();
                    },
                    '"' if !curr.is_empty() => {
                        curr_arr.push(curr);
                        curr = String::new();
                    },
                    '\n' | '[' | '"' | ',' => (),
                    c => curr.push(c),
                }
            }
        }
        e.pop();
        //println!("p: {p:#?}");
        //println!("s: '{s}'");
        //println!("e: {e:#?}");
        (p, s, e)
    }

    #[test]
    fn test18() {
        let (p, s, e) = read_data("src/c2022_06_19_test_18.txt");
        assert_eq!(Solution::suggested_products(p, s), e);
    }
}
