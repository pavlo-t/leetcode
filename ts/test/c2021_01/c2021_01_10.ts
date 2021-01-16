import {expect} from 'chai';

/**
 * ### Create Sorted Array through Instructions
 * {@link https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3599/}
 */
function createSortedArray(instructions: number[]): number {
    // Binary Indexed Tree aka Fenwick Tree
    const bit = Array(100_001).fill(0);
    // Lowest Set Bit
    const lsb = (i: number): number => i & -i;
    const sum = (i: number): number => {
        let sum = 0;
        while (i > 0) {
            sum += bit[i];
            i -= lsb(i);
        }
        return sum;
    };
    const add = (i: number, v: number) => {
        while (i < bit.length) {
            bit[i] += v;
            i += lsb(i);
        }
    };

    const M = 1_000_000_007;
    let cost = 0;
    for (let i = 0; i < instructions.length; i++) {
        const v = instructions[i];
        cost += Math.min(sum(v - 1), i - sum(v));
        add(v, 1);
        cost %= M;
    }
    return cost;
}

// noinspection JSUnusedLocalSymbols
function createSortedArraySegmentTree(instructions: number[]): number {
    const m = 100_001;
    const segmentTree = Array(m * 2).fill(0);
    const update = (i: number, v: number) => {
        i += m;
        while (i > 0) {
            segmentTree[i] += v;
            i >>= 1;
        }
    };
    const query = (l: number, r: number): number => {
        l += m;
        r += m;
        let result = 0;
        while (l < r) {
            if (l & 1) result += segmentTree[l++];
            if (r & 1) result += segmentTree[--r];
            l >>= 1;
            r >>= 1;
        }
        return result;
    };

    const M = 1_000_000_007;
    let cost = 0;
    for (const v of instructions) {
        cost += Math.min(query(0, v), query(v + 1, m));
        update(v, 1);
        cost %= M;
    }
    return cost;
}

// noinspection JSUnusedLocalSymbols
function createSortedArrayBruteForce(instructions: number[]): number {
    const M = 1_000_000_007;
    let cost = 0;

    for (let i = 0; i < instructions.length; i++) {
        let lCost = 0;
        let rCost = 0;

        for (let j = 0; j < i; j++) {
            if (instructions[i] > instructions[j])
                lCost++;
            else if (instructions[i] < instructions[j])
                rCost++;
        }

        cost += Math.min(lCost, rCost);
        cost %= M;
    }
    return cost;
}

describe('c2021_01_10 Create Sorted Array through Instructions', () => {
    it('Example 1: (instructions = [1,5,6,2]) -> 1', () => {
        expect(createSortedArray([1, 5, 6, 2])).to.eql(1);
        // Explanation: Begin with nums = [].
        // Insert 1 with cost min(0, 0) = 0, now nums = [1].
        // Insert 5 with cost min(1, 0) = 0, now nums = [1,5].
        // Insert 6 with cost min(2, 0) = 0, now nums = [1,5,6].
        // Insert 2 with cost min(1, 2) = 1, now nums = [1,2,5,6].
        // The total cost is 0 + 0 + 0 + 1 = 1.
    });
    it('Example 2: (instructions = [1,2,3,6,5,4]) -> 3', () => {
        expect(createSortedArray([1, 2, 3, 6, 5, 4])).to.eql(3);
        // Explanation: Begin with nums = [].
        // Insert 1 with cost min(0, 0) = 0, now nums = [1].
        // Insert 2 with cost min(1, 0) = 0, now nums = [1,2].
        // Insert 3 with cost min(2, 0) = 0, now nums = [1,2,3].
        // Insert 6 with cost min(3, 0) = 0, now nums = [1,2,3,6].
        // Insert 5 with cost min(3, 1) = 1, now nums = [1,2,3,5,6].
        // Insert 4 with cost min(3, 2) = 2, now nums = [1,2,3,4,5,6].
        // The total cost is 0 + 0 + 0 + 0 + 1 + 2 = 3.
    });
    it('Example 3: (instructions = [1,3,3,3,2,4,2,1,2]) -> 4', () => {
        expect(createSortedArray([1, 3, 3, 3, 2, 4, 2, 1, 2])).to.eql(4);
        // Explanation: Begin with nums = [].
        // Insert 1 with cost min(0, 0) = 0, now nums = [1].
        // Insert 3 with cost min(1, 0) = 0, now nums = [1,3].
        // Insert 3 with cost min(1, 0) = 0, now nums = [1,3,3].
        // Insert 3 with cost min(1, 0) = 0, now nums = [1,3,3,3].
        // Insert 2 with cost min(1, 3) = 1, now nums = [1,2,3,3,3].
        // Insert 4 with cost min(5, 0) = 0, now nums = [1,2,3,3,3,4].
        // Insert 2 with cost min(1, 4) = 1, now nums = [1,2,2,3,3,3,4].
        // Insert 1 with cost min(0, 6) = 0, now nums = [1,1,2,2,3,3,3,4].
        // Insert 2 with cost min(2, 4) = 2, now nums = [1,1,2,2,2,3,3,3,4].
        // The total cost is 0 + 0 + 0 + 0 + 1 + 0 + 1 + 0 + 2 = 4.
    });

    it('(instructions = 1 to 10_000) -> 0', () => {
        const instructions = Array(10000).fill(0).map((_, i) => i + 1);
        expect(createSortedArray(instructions)).to.eql(0);
    });
    it('(instructions = 1 to 100_000) -> 0', () => {
        const instructions = Array(100000).fill(0).map((_, i) => i + 1);
        expect(createSortedArray(instructions)).to.eql(0);
    });
});
