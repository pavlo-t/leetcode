#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::AddAssign;

/// # Encode and Decode TinyURL
///
/// > Note: This is a companion problem to the [System Design] problem: [Design TinyURL].
///
/// TinyURL is a URL shortening service where you enter a URL such as
/// `https://leetcode.com/problems/design-tinyurl` and it returns a short URL such as
/// `http://tinyurl.com/4e9iAk`.
///
/// Design the `encode` and `decode` methods for the TinyURL service.
/// There is no restriction on how your encode/decode algorithm should work.
/// You just need to ensure that a URL can be encoded to a tiny URL and the tiny URL can be
/// decoded to the original URL.
///
/// [System Design]:https://leetcode.com/discuss/interview-question/system-design/
/// [Design TinyURL]:https://leetcode.com/discuss/interview-question/124658/Design-a-URL-Shortener-(-TinyURL-)-System/
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3673/
struct Codec {
    data: RefCell<HashMap<String, String>>,
    next: RefCell<i64>,
}

impl Codec {
    fn new() -> Self {
        Codec {
            data: RefCell::new(HashMap::new()),
            next: RefCell::new(0),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&self, long_url: String) -> String {
        let mut data = self.data.borrow_mut();
        let mut next = self.next.borrow_mut();
        let k = format!("http://tinyurl.com/{}", next);
        data.insert(k.clone(), long_url);
        next.add_assign(1);
        k
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {
        self.data.borrow().get(&short_url).unwrap().clone()
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let codec = Codec::new();
        let urls = vec![
            "https://google.com".to_string(),
            "https://leetcode.com/problems/design-tinyurl".to_string(),
            "https://leetcode.com/discuss/interview-question/system-design/".to_string(),
        ];
        let mut t_urls = Vec::new();
        for u in &urls {
            let tu = codec.encode(u.clone());
            assert_ne!(&tu, u);
            t_urls.push(tu);
        }
        for (i, tu) in t_urls.iter().enumerate() {
            let u = codec.decode(tu.clone());
            assert_eq!(u, urls[i]);
        }
    }
}
