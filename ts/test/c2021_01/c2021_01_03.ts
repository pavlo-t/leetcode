import {expect} from 'chai';

/**
 * ### Beautiful Arrangement
 *
 * {@link https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3591/}
 */
function countArrangement(n: number): number {
    const isBeautiful = (i: number, j: number): boolean => i % j == 0 || j % i == 0;

    const backtrack = (seen: number, i: number): number => {
        if (i > n) {
            return 1;
        }

        let result = 0;
        for (let j = 1; j <= n; j++) {
            if ((seen & (1 << (j - 1))) === 0 && isBeautiful(i, j)) {
                result += backtrack(seen | (1 << (j - 1)), i + 1);
            }
        }
        return result;
    };

    return backtrack(0, 1);
}

describe('c2021-01-03 Beautiful Arrangement', () => {
    it('Example 1: (n = 2) -> 2', () => {
        expect(countArrangement(2)).to.eql(2);
        // Explanation:
        // The first beautiful arrangement is [1,2]:
        //     - perm[1] = 1 is divisible by i = 1
        //     - perm[2] = 2 is divisible by i = 2
        // The second beautiful arrangement is [2,1]:
        //     - perm[1] = 2 is divisible by i = 1
        //     - i = 2 is divisible by perm[2] = 1
    });
    it('Example 2: (n = 1) -> 1', () => {
        expect(countArrangement(1)).to.eql(1);
    });

    it('(n = 15) -> 24679', () => {
        expect(countArrangement(15)).to.eql(24679);
    });
});
