import {expect} from 'chai';

// noinspection JSUnusedLocalSymbols
/**
 * ### Palindrome Permutation
 * {@link https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3588/}
 */
function canPermutePalindrome(s: string): boolean {
    const map = new Map<string, boolean>();
    let odds = 0;
    for (let i = 0; i < s.length; i++) {
        const prev = map.get(s[i]);
        if (prev) {
            map.set(s[i], false);
            odds--;
        } else {
            map.set(s[i], true);
            odds++;
        }
    }
    return odds < 2;
}

// noinspection JSUnusedLocalSymbols
function canPermutePalindromeSet(s: string): boolean {
    const set = new Set<string>();
    for (let i = 0; i < s.length; i++) {
        if (set.has(s[i])) {
            set.delete(s[i]);
        } else {
            set.add(s[i]);
        }
    }
    return set.size < 2;
}

describe('c2021_01_w1 Palindrome Permutation', () => {
    it('Example 1: ("code") -> false', () => {
        expect(canPermutePalindrome('code')).to.be.false;
    });
    it('Example 2: ("aab") -> true', () => {
        expect(canPermutePalindrome('aab')).to.be.true;
    });
    it('Example 3: ("carerac") -> true', () => {
        expect(canPermutePalindrome('carerac')).to.be.true;
    });

    it('("a".repeat(500000)) -> true', () => {
        expect(canPermutePalindrome('a'.repeat(500000))).to.be.true;
    });
});