#![allow(dead_code)]

/// Search Suggestions System
/// =========================
///
/// Given an array of strings `products` and a string `searchWord`.
/// We want to design a system that suggests at most three product names from `products`
/// after each character of `searchWord` is typed.
/// Suggested products should have common prefix with the searchWord.
/// If there are more than three products with a common prefix return the three lexicographically minimums products.
///
/// Return _list of lists_ of the suggested `products` after each character of `searchWord` is typed.
///
/// __Constraints:__
///
/// - `1 <= products.length <= 1000`
/// - There are no repeated elements in `products`.
/// - `1 <= Σ products[i].length <= 20_000`
/// - All characters of `products[i]` are lower-case English letters.
/// - `1 <= searchWord.length <= 1000`
/// - All characters of `searchWord` are lower-case English letters.
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/602/week-5-may-29th-may-31st/3762/
struct Solution;
impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        #[derive(Default)]
        struct TrieNode {
            children: [Option<Box<TrieNode>>; 26],
            word: Option<String>,
        }
        impl TrieNode {
            fn add(&mut self, word: String) {
                let mut curr = self;
                for b in word.bytes() {
                    let i = (b - b'a') as usize;
                    curr = curr.children[i].get_or_insert_with(|| Box::new(TrieNode::default()));
                }
                curr.word = Some(word);
            }

            fn get_three(&self, output: &mut Vec<String>) {
                if output.len() < 3 {
                    if let Some(word) = self.word.as_ref() {
                        output.push(word.clone());
                    }
                    for child in &self.children {
                        if let Some(child) = child.as_ref() {
                            child.get_three(output);
                        }
                    }
                }
            }
        }

        let mut trie = Box::new(TrieNode::default());
        for prod in products {
            trie.add(prod);
        }

        let mut result = vec![];

        let mut curr = Some(&trie);
        for b in search_word.bytes() {
            let mut three = vec![];
            if let Some(node) = curr {
                let i = (b - b'a') as usize;
                let child = node.children[i].as_ref();
                if let Some(node) = child {
                    node.get_three(&mut three);
                }
                curr = child;
            }
            result.push(three);
        }

        result
    }

    pub fn suggested_products_binary_search(
        mut products: Vec<String>,
        search_word: String,
    ) -> Vec<Vec<String>> {
        products.sort_unstable();

        fn bs(sw: &str, products: &[String]) -> Vec<String> {
            let mut l = 0;
            let mut r = products.len() - 1;
            let mut result = vec![];
            while l < r {
                let m = l + (r - l) / 2;
                if products[m].starts_with(sw) || sw < &products[m] {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            for i in l..products.len().min(l + 3) {
                if products[i].starts_with(sw) {
                    result.push(products[i].clone())
                } else {
                    break;
                }
            }
            result
        }

        let mut results = vec![];
        for r in 1..=search_word.len() {
            let sw = &search_word[0..r];
            let result = bs(sw, &products);
            results.push(result);
        }
        results
    }

    pub fn suggested_products_brute_force(
        mut products: Vec<String>,
        search_word: String,
    ) -> Vec<Vec<String>> {
        products.sort_unstable();

        let mut results = vec![];

        for r in 1..=search_word.len() {
            let sw = &search_word[0..r];
            let result = products
                .iter()
                .filter(|&w| w.starts_with(sw))
                .take(3)
                .map(|w| w.clone())
                .collect::<Vec<_>>();
            results.push(result);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}
    macro_rules! vvs { ($($s:tt),*) => { vec![$(vs!$s),*] }; }
    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn example1() {
        let products = vs!["mobile", "mouse", "moneypot", "monitor", "mousepad"];
        let search_word = "mouse".to_string();
        let e = vvs![
            ["mobile", "moneypot", "monitor"],
            ["mobile", "moneypot", "monitor"],
            ["mouse", "mousepad"],
            ["mouse", "mousepad"],
            ["mouse", "mousepad"]
        ];
        assert_eq!(Solution::suggested_products(products, search_word), e);
        // Explanation: products sorted lexicographically = ["mobile","moneypot","monitor","mouse","mousepad"]
        // After typing m and mo all products match and we show user ["mobile","moneypot","monitor"]
        // After typing mou, mous and mouse the system suggests ["mouse","mousepad"]
    }
    #[test]
    fn example2() {
        let products = vs!["havana"];
        let search_word = "havana".to_string();
        let e = vvs![
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"]
        ];
        assert_eq!(Solution::suggested_products(products, search_word), e);
    }
    #[test]
    fn example3() {
        let products = vs!["bags", "baggage", "banner", "box", "cloths"];
        let search_word = "bags".to_string();
        let e = vvs![
            ["baggage", "bags", "banner"],
            ["baggage", "bags", "banner"],
            ["baggage", "bags"],
            ["bags"]
        ];
        assert_eq!(Solution::suggested_products(products, search_word), e);
    }
    #[test]
    fn example4() {
        let products = vs!["havana"];
        let search_word = "tatiana".to_string();
        let e: Vec<Vec<String>> = vv![[], [], [], [], [], [], []];
        assert_eq!(Solution::suggested_products(products, search_word), e);
    }

    mod performance {
        use super::*;

        #[test]
        fn p_1000_words_sw_1000_chars_produces_empty_array() {
            let products = vec!["a".repeat(20); 1000];
            let search_word = "b".repeat(1000);
            let e: Vec<Vec<String>> = vec![vec![]; 1000];
            assert_eq!(Solution::suggested_products(products, search_word), e);
        }
        #[ignore]
        #[test]
        fn p_1000_words_sw_1000_chars_produces_lots_of_results() {
            let mut products = vec!["a".repeat(20); 997];
            products.push("a".repeat(1000));
            products.push("a".repeat(1000));
            products.push("a".repeat(1000));
            let search_word = "a".repeat(1000);
            let mut e: Vec<Vec<String>> = vec![vec!["a".repeat(20); 3]; 20];
            let mut rest = vec![vec!["a".repeat(1000); 3]; 980];
            e.append(&mut rest);
            assert_eq!(Solution::suggested_products(products, search_word), e);
        }
    }
}
