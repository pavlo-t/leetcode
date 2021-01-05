import {expect} from 'chai';

/**
 * ### Check Array Formation Through Concatenation
 * {@link https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3589/}
 */
function canFormArray(arr: number[], pieces: number[][]): boolean {
    const map: { [i: number]: number[] } = {};
    pieces.forEach(p => map[p[0]] = p);
    let i = 0;
    let piece = map[arr[i]];

    while (piece) {
        for (const v of piece) {
            if (arr[i] !== v) {
                return false;
            }
            i++;
        }
        if (i == arr.length) {
            return true;
        }
        piece = map[arr[i]];
    }

    return false;
}

// noinspection JSUnusedLocalSymbols
function canFormArrayMapNumberNumber(arr: number[], pieces: number[][]): boolean {
    const map = new Map(pieces.map(p => [p[0], p]));
    let i = 0;
    let piece = map.get(arr[i]);

    while (piece) {
        for (const v of piece) {
            if (arr[i] !== v) {
                return false;
            }
            i++;
        }
        if (i == arr.length) {
            return true;
        }
        piece = map.get(arr[i]);
    }

    return false;
}

// noinspection JSUnusedLocalSymbols
function canFormArrayFindPiece(arr: number[], pieces: number[][]): boolean {
    let i = 0;
    let piece = pieces.find(p => p[0] == arr[i]);

    while (piece) {
        for (const v of piece) {
            if (arr[i] !== v) {
                return false;
            }
            i++;
        }
        if (i == arr.length) {
            return true;
        }
        piece = pieces.find(p => p[0] == arr[i]);
    }

    return false;
}

describe('c2021_01_01 Check Array Formation Through Concatenation', () => {
    it('Example 1: (arr = [85], pieces = [[85]]) -> true', () => {
        const arr = [85];
        const pieces = [[85]];
        expect(canFormArray(arr, pieces)).to.be.true;
    });
    it('Example 2: ([15,88], pieces = [[88],[15]]) -> true', () => {
        const arr = [15, 88];
        const pieces = [[88], [15]];
        expect(canFormArray(arr, pieces)).to.be.true;
        // Explanation: Concatenate [15] then [88]
    });
    it('Example 3: (arr = [49,18,16], pieces = [[16,18,49]]) -> false', () => {
        const arr = [49, 18, 16];
        const pieces = [[16, 18, 49]];
        expect(canFormArray(arr, pieces)).to.be.false;
        // Explanation: Even though the numbers match, we cannot reorder pieces[0].
    });
    it('Example 4: (arr = [91,4,64,78], pieces = [[78],[4,64],[91]]) -> true', () => {
        const arr = [91, 4, 64, 78];
        const pieces = [[78], [4, 64], [91]];
        expect(canFormArray(arr, pieces)).to.be.true;
        // Explanation: Concatenate [91] then [4,64] then [78]
    });
    it('Example 5: (arr = [1,3,5,7], pieces = [[2,4,6,8]]) -> false', () => {
        const arr = [1, 3, 5, 7];
        const pieces = [[2, 4, 6, 8]];
        expect(canFormArray(arr, pieces)).to.be.false;
    });

    it('(arr = 1 to 10000, pieces = 10000 to 1 map []) -> true', () => {
        const size = 10000;
        const arr = Array(size).fill(0).map((_, i) => i + 1);
        const pieces = Array(size).fill(0).map((_, i) => [size - i]);
        expect(canFormArray(arr, pieces)).to.be.true;
    });
    it('(arr = 1 to 10000, pieces = 10000 to 10 by 10 map []) -> true', () => {
        const size = 10000;
        const pSize = 10;
        const arr = Array(size).fill(0).map((_, i) => i + 1);
        const pieces =
            Array(size / pSize).fill(0)
                .map((_, i) => Array(pSize).fill(0)
                    .map((_, j) => size - (i * pSize) - pSize + j + 1));
        expect(canFormArray(arr, pieces)).to.be.true;
    });
});

