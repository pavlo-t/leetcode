import {expect} from 'chai';

/**
 * ### Next Greater Element III
 *
 * {@link https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3578/}
 */
function nextGreaterElement(n: number): number {
    const MAX = 2_147_483_647; // 2^31 - 1
    if (n < 12) {
        return -1;
    }

    const chars = n.toString().split('');

    for (let r = chars.length - 1; r > 0; r--) {
        const l = r - 1;
        if (chars[l] < chars[r]) {
            let swap = chars.length - 1;
            while (chars[l] >= chars[swap]) swap--;
            [chars[l], chars[swap]] = [chars[swap], chars[l]];

            const head = chars.slice(0, r).join('');
            const tail = chars.slice(r).sort().join('');

            const result = parseInt(head + tail, 10);

            return result > MAX ? -1 : result;
        }
    }

    return -1;
}

describe('c2020_12_23 Next Greater Element III', () => {
    it('Example 1: (n = 12) -> 21', () => {
        expect(nextGreaterElement(12)).to.eql(21);
    });
    it('Example 2: (n = 21) -> -1', () => {
        expect(nextGreaterElement(21)).to.eql(-1);
    });

    it('Test 36: (n = 1999999999) -> -1', () => {
        expect(nextGreaterElement(1999999999)).to.eql(-1);
    });

    it('(n = 14654321) -> 15123446', () => {
        expect(nextGreaterElement(14654321)).to.eql(15123446);
    });
    it('(n = 2147483647) -> -1', () => {
        expect(nextGreaterElement(2_147_483_647)).to.eql(-1);
    });
});