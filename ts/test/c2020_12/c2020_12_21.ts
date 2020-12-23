import {expect} from 'chai';

// https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/571/week-3-december-15th-december-21st/3573/
/**
 * ### Smallest Range II
 *
 * Given an array `A` of integers,
 * for each integer `A[i]` we need to choose either `x = -K` or `x = K`,
 * and add `x` to `A[i]` **(only once)**.
 *
 * After this process, we have some array `B`.
 *
 * Return the smallest possible difference between the maximum value of `B` and the minimum value of `B`.
 *
 * **Note:**
 * 1. `1 <= A.length <= 10000`
 * 2. `0 <= A[i] <= 10000`
 * 3. `0 <= K <= 10000`
 *
 * {@link https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/571/week-3-december-15th-december-21st/3573/}
 */
function smallestRangeII(A: number[], K: number): number {
    A = [...A].sort((a, b) => a - b);

    let result = A[A.length - 1] - A[0];
    const hi = A[A.length - 1] - K;
    const lo = A[0] + K;

    for (let i = 1; i < A.length; i++) {
        const max = Math.max(hi, A[i - 1] + K);
        const min = Math.min(lo, A[i] - K);
        result = Math.min(result, max - min);
    }

    return result;
}

describe('c2020_12_21 Smallest Range II', () => {
    it('Example 1: (A = [1], K = 0) -> 0', () => {
        expect(smallestRangeII([1], 0)).to.eql(0);
        // Explanation: B = [1]
    });
    it('Example 2: (A = [0,10], K = 2) -> 6', () => {
        expect(smallestRangeII([0, 10], 2)).to.eql(6);
        // Explanation: B = [2,8]
    });
    it('Example 3: (A = [1,3,6], K = 3) -> 3', () => {
        expect(smallestRangeII([1, 3, 6], 3)).to.eql(3);
        // Explanation: B = [4,6,3]
    });

    it('(A = [1,2,3,4,5], K = 4) -> 4', () => {
        expect(smallestRangeII([1, 2, 3, 4, 5], 4)).to.eql(4);
        // Explanation: B = [5,6,7,8,9]
    });

    it('(A = 10000 zeros, K = 3) -> 0', () => {
        expect(smallestRangeII(Array(10000).fill(0), 3)).to.eql(0);
    });
    it('(A = [1..=10000], K = 100) -> 9799', () => {
        const A = Array(10000).fill(0).map((_, i) => i + 1);
        expect(smallestRangeII(A, 100)).to.eql(9799);
    });
    it('(A = [10000..=1], K = 100) -> 9799', () => {
        const A = Array(10000).fill(0).map((_, i) => 10000 - i);
        expect(smallestRangeII(A, 100)).to.eql(9799);
    });
});
