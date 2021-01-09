import {expect} from 'chai';

/**
 * ### Longest Substring Without Repeating Characters
 * {@link https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3595/}
 */
function lengthOfLongestSubstring(s: string): number {
    const map: Map<string, number> = new Map();
    let result = 0;
    for (let l = 0, r = 0; r < s.length; r++) {
        const prev = map.get(s[r]);
        if (prev !== undefined && prev >= l)
            l = prev + 1;
        map.set(s[r], r);
        result = Math.max(result, r - l + 1);
    }
    return result;
}

describe('c2021_01_07 Longest Substring Without Repeating Characters', () => {
    it('Example 1: (s = "abcabcbb") -> 3', () => {
        expect(lengthOfLongestSubstring('abcabcbb')).to.eql(3);
        // Explanation: The answer is "abc", with the length of 3.
    });
    it('Example 2: (s = "bbbbb") -> 1', () => {
        expect(lengthOfLongestSubstring('bbbbb')).to.eql(1);
        // Explanation: The answer is "b", with the length of 1.
    });
    it('Example 3: (s = "pwwkew") -> 3', () => {
        expect(lengthOfLongestSubstring('pwwkew')).to.eql(3);
        // Explanation: The answer is "wke", with the length of 3.
        // Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
    });
    it('Example 4: (s = "") -> 0', () => {
        expect(lengthOfLongestSubstring('')).to.eql(0);
    });

    it('(s = "a") -> 1', () => {
        expect(lengthOfLongestSubstring('a')).to.eql(1);
    });
    it('(s = "abba") -> 2', () => {
        expect(lengthOfLongestSubstring('abba')).to.eql(2);
    });

});
