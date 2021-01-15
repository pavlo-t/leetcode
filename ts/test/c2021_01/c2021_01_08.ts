import {expect} from 'chai';

/**
 * ### Check If Two String Arrays are Equivalent
 * {@link https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3597/}
 */
function arrayStringsAreEqual(word1: string[], word2: string[]): boolean {
    return word1.join('') === word2.join('');
}

// noinspection JSUnusedLocalSymbols
function arrayStringsAreEqualO1Memory(word1: string[], word2: string[]): boolean {
    let i1 = 0;
    let i2 = 0;
    let j1 = 0;
    let j2 = 0;

    while (i1 < word1.length && i2 < word2.length) {
        if (word1[i1][j1] !== word2[i2][j2]) {
            return false;
        }
        if (j1 === word1[i1].length - 1) {
            i1++;
            j1 = 0;
        } else {
            j1++;
        }
        if (j2 === word2[i2].length - 1) {
            i2++;
            j2 = 0;
        } else {
            j2++;
        }
    }

    return i1 === word1.length && i2 === word2.length;
}


describe('c2021_01_08 Check If Two String Arrays are Equivalent', () => {
    it('Example 1: (word1 = ["ab", "c"], word2 = ["a", "bc"]) -> true', () => {
        expect(arrayStringsAreEqual(['ab', 'c'], ['a', 'bc'])).to.be.true;
        // Explanation:
        // word1 represents string "ab" + "c" -> "abc"
        // word2 represents string "a" + "bc" -> "abc"
        // The strings are the same, so return true.
    });
    it('Example 2: (word1 = ["a", "cb"], word2 = ["ab", "c"]) -> false', () => {
        expect(arrayStringsAreEqual(['a', 'cb'], ['ab', 'c'])).to.be.false;
    });
    it('Example 3: (word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]) -> true', () => {
        expect(arrayStringsAreEqual(['abc', 'd', 'defg'], ['abcddefg'])).to.be.true;
    });
});
