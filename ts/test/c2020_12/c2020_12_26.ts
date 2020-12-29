import {expect} from 'chai';

/**
 * ### Decode Ways
 *
 * {@link https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3581/}
 */
function numDecodings(s: string): number {
    let pp = 1;
    let p = s[0] !== '0' ? 1 : 0;

    for (let i = 1; i < s.length; i++) {
        let c = 0;

        if (s[i] !== '0') {
            c += p;
        }
        const num = Number(s.slice(i - 1, i + 1));
        if (9 < num && num < 27) {
            c += pp;
        }

        pp = p;
        p = c;
    }

    return p;
}

describe('c2020_12_26 Decode Ways', () => {
    it('Example 1: (s = "12") -> 2', () => {
        expect(numDecodings('12')).to.eql(2);
        // Explanation: It could be decoded as "AB" (1 2) or "L" (12).
    });
    it('Example 2: (s = "226") -> 3', () => {
        expect(numDecodings('226')).to.eql(3);
        // Explanation: It could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
    });
    it('Example 3: (s = "0") -> 0', () => {
        expect(numDecodings('0')).to.eql(0);
        // Explanation:
        // There is no character that is mapped to a number starting with '0'.
        // We cannot ignore a zero when we face it while decoding.
        // So, each '0' should be part of "10" --> 'J' or "20" --> 'T'.
    });
    it('Example 4: (s = "1") -> 1', () => {
        expect(numDecodings('1')).to.eql(1);
    });

    it('("3".repeat(100)) -> 1', () => {
        expect(numDecodings('3'.repeat(100))).to.eql(1);
    });
    it('("1".repeat(100)) -> a lot', () => {
        expect(numDecodings('1'.repeat(100))).to.eql(573147844013817200000);
    });
});
