import {expect} from 'chai';

/**
 * ### Reach a Number
 *
 * {@link https://leetcode.com/explore/featured/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3583/}
 */
function reachNumber(target: number): number {
    target = Math.abs(target);
    let steps = 0;
    while (target > 0 || target % 2 != 0) {
        steps++;
        target -= steps;
    }
    return steps;
}

describe('c2020_12_28 Reach a Number', () => {
    it('(target = 0) -> 0', () => {
        expect(reachNumber(0)).to.eql(0);
    });
    it('(target = 1) -> 1', () => {
        expect(reachNumber(1)).to.eql(1);
    });
    it('Example 2: (target = 2) -> 3', () => {
        expect(reachNumber(2)).to.eql(3);
        // Explanation:
        // On the first move we step from 0 to 1.
        // On the second move we step  from 1 to -1.
        // On the third move we step from -1 to 2.
    });
    it('Example 1: (target = 3) -> 2', () => {
        expect(reachNumber(3)).to.eql(2);
        // Explanation:
        // On the first move we step from 0 to 1.
        // On the second step we step from 1 to 3.
    });
    it('(target = 4) -> 3', () => {
        expect(reachNumber(4)).to.eql(3);
    });
    it('(target = 5) -> 5', () => {
        expect(reachNumber(5)).to.eql(5);
    });
    it('(target = 6) -> 3', () => {
        expect(reachNumber(6)).to.eql(3);
    });
    it('(target = 7) -> 5', () => {
        expect(reachNumber(7)).to.eql(5);
    });
    it('(target = 12) -> 7', () => {
        expect(reachNumber(12)).to.eql(7);
    });
    it('(target = 1,000,000,000) -> 44723', () => {
        expect(reachNumber(1_000_000_000)).to.eql(44723);
    });
    it('(target = -1,000,000,000) -> 44723', () => {
        expect(reachNumber(-1_000_000_000)).to.eql(44723);
    });
});
