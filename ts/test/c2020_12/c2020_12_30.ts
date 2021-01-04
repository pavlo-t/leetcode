import {expect} from 'chai';

/**
 * ### Game of Life
 *
 * {@link https://leetcode.com/explore/featured/card/december-leetcoding-challenge/573/week-5-december-29th-december-31st/3586/}
 *
 Do not return anything, modify board in-place instead.
 */
function gameOfLife(board: number[][]): void {
    const m = board.length;
    const n = board[0].length;

    const countNeighbors = (r: number, c: number): number => [
        [r - 1, c - 1], [r - 1, c], [r - 1, c + 1],
        [r, c - 1], [r, c + 1],
        [r + 1, c - 1], [r + 1, c], [r + 1, c + 1]
    ].reduce((acc, [r, c]) => (r >= 0 && r < m && c >= 0 && c < n) ? acc + (board[r][c] & 1) : acc, 0);

    for (let r = 0; r < m; r++) {
        for (let c = 0; c < n; c++) {
            const ns = countNeighbors(r, c);
            if (ns == 3) board[r][c] |= 2;
            else if (ns == 2) board[r][c] |= (board[r][c] << 1);
        }
    }
    for (let r = 0; r < m; r++) {
        for (let c = 0; c < n; c++) {
            board[r][c] >>= 1;
        }
    }
}

describe('c2020_12_30 Game of Life', () => {
    it('Example 1: (board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]) -> [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]', () => {
        const board = [[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]];
        gameOfLife(board);
        expect(board).to.eql([[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]]);
    });
    it('Example 2: (board = [[1,1],[1,0]]) -> [[1,1],[1,1]]', () => {
        const board = [[1, 1], [1, 0]];
        gameOfLife(board);
        expect(board).to.eql([[1, 1], [1, 1]]);
    });

    it('(board = 100x100 0s) -> 100x100 0s', () => {
        const side = 100;
        const board: number[][] = Array(side).fill(0).map(() => Array(side).fill(0));

        gameOfLife(board);

        const expected = Array(side).fill(0).map(() => Array(side).fill(0));
        expect(board).to.eql(expected);
    });
    it('(board = 100x100 1s) -> 100x100 0s with 1s in the corners', () => {
        const side = 100;
        const board: number[][] = Array(side).fill(0).map(() => Array(side).fill(1));

        gameOfLife(board);

        const expected = Array(side).fill(0).map(() => Array(side).fill(0));
        expected[0][0] = 1;
        expected[0][side - 1] = 1;
        expected[side - 1][0] = 1;
        expected[side - 1][side - 1] = 1;
        expect(board).to.eql(expected);
    });
});
