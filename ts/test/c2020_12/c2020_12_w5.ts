import {expect} from 'chai';

/**
 * ### Longest Substring with At Most K Distinct Characters
 *
 * {@link https://leetcode.com/explore/featured/card/december-leetcoding-challenge/573/week-5-december-29th-december-31st/3584/}
 */
function lengthOfLongestSubstringKDistinct(s: string, k: number): number {
    const counts: Map<string, number> = new Map();
    let result = 0;

    let l = 0;
    let r = 0;
    while (r < s.length) {
        const rc = s[r];
        counts.set(rc, (counts.get(rc) || 0) + 1);
        while (counts.size > k) {
            const lc = s[l];
            if (counts.get(lc)! === 1) counts.delete(lc);
            else counts.set(lc, counts.get(lc)! - 1);
            l++;
        }
        result = Math.max(result, r - l + 1);
        r++;
    }

    return result;
}

// noinspection JSUnusedLocalSymbols
function lengthOfLongestSubstringKDistinctMapNumberNumber(s: string, k: number): number {
    const counts: Map<number, number> = new Map();
    let result = 0;

    let l = 0;
    let r = 0;
    while (r < s.length) {
        const rc = s.charCodeAt(r);
        counts.set(rc, (counts.get(rc) || 0) + 1);
        while (counts.size > k) {
            const lc = s.charCodeAt(l);
            if (counts.get(lc)! === 1) counts.delete(lc);
            else counts.set(lc, counts.get(lc)! - 1);
            l++;
        }
        result = Math.max(result, r - l + 1);
        r++;
    }

    return result;
}

describe('c2020_12_w5 Longest Substring with At Most K Distinct Characters', () => {
    it('Example 1: (s = "eceba", k = 2) -> 3', () => {
        const s = 'eceba';
        const k = 2;
        expect(lengthOfLongestSubstringKDistinct(s, k)).to.eql(3);
        // Explanation: The substring is "ece" with length 3.
    });
    it('Example 2: (s = "aa", k = 1) -> 2', () => {
        const s = 'aa';
        const k = 1;
        expect(lengthOfLongestSubstringKDistinct(s, k)).to.eql(2);
        // Explanation: The substring is "aa" with length 2.
    });

    it('(s = "a".repeat(50000), k = 0) -> 0', () => {
        const s = 'a'.repeat(50000);
        const k = 0;
        expect(lengthOfLongestSubstringKDistinct(s, k)).to.eql(0);
    });
    it('(s = "a".repeat(50000), k = 1) -> 50000', () => {
        const s = 'a'.repeat(50000);
        const k = 1;
        expect(lengthOfLongestSubstringKDistinct(s, k)).to.eql(50000);
    });
    it('(s = "ab".repeat(25000), k = 2) -> 50000', () => {
        const s = 'ab'.repeat(25000);
        const k = 2;
        expect(lengthOfLongestSubstringKDistinct(s, k)).to.eql(50000);
    });
    it('(s = "ab".repeat(25000), k = 1) -> 1', () => {
        const s = 'ab'.repeat(25000);
        const k = 1;
        expect(lengthOfLongestSubstringKDistinct(s, k)).to.eql(1);
    });
});
