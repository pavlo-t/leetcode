import {expect} from 'chai';

/**
 * ### Diagonal Traverse
 *
 * {@link https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3580/}
 */
function findDiagonalOrder(M: number[][]): number[] {
    if (M.length == 0 || M[0].length == 0)
        return [];
    const m = M.length;
    const n = M[0].length;

    const lastRow = m - 1;
    const lastCol = n - 1;
    let r = 0;
    let c = 0;
    let goingUp = true;

    const result = [];

    for (let i = 0; i < n * m; i++) {
        result[i] = M[r][c];
        if (goingUp) {
            if (c == lastCol) {
                goingUp = false;
                r++;
            } else if (r == 0) {
                goingUp = false;
                c++;
            } else {
                r--;
                c++;
            }
        } else {
            if (r == lastRow) {
                goingUp = true;
                c++;
            } else if (c == 0) {
                goingUp = true;
                r++;
            } else {
                r++;
                c--;
            }
        }
    }

    return result;
}

describe('c2020_12_25 Diagonal Traverse', () => {
    it('Example: ([[1,2,3],[4,5,6],[7,8,9]]) -> [1,2,4,7,5,3,6,8,9]', () => {
        const matrix = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]];
        expect(findDiagonalOrder(matrix)).to.eql([1, 2, 4, 7, 5, 3, 6, 8, 9]);
    });

    it('([]) -> []', () => {
        const matrix: number[][] = [];
        expect(findDiagonalOrder(matrix)).to.eql([]);
    });
    it('([[]]) -> []', () => {
        const matrix: number[][] = [[]];
        expect(findDiagonalOrder(matrix)).to.eql([]);
    });
    it('([[1]]) -> [1]', () => {
        const matrix: number[][] = [[1]];
        expect(findDiagonalOrder(matrix)).to.eql([1]);
    });
    it('([[1,2]]) -> [1,2]', () => {
        const matrix = [[1, 2]];
        expect(findDiagonalOrder(matrix)).to.eql([1, 2]);
    });
    it('([[1],[2]]) -> [1,2]', () => {
        const matrix = [[1], [2]];
        expect(findDiagonalOrder(matrix)).to.eql([1, 2]);
    });
    it('([[1,2],[3,4]]) -> [1,2,3,4]', () => {
        const matrix = [
            [1, 2],
            [3, 4]];
        expect(findDiagonalOrder(matrix)).to.eql([1, 2, 3, 4]);
    });
    it('([100x100]) -> ???', () => {
        const m = 100;
        const n = 100;
        const matrix =
            Array(m).fill(0)
                .map((_, i) => Array(n).fill(0)
                    .map((_, j) => i * n + j + 1));
        const expected = Array(m * n).fill(0).map((_, i) => i + 1);
        expect(findDiagonalOrder(matrix)).to.have.members(expected);
    });
});
