import {expect} from 'chai';

// bottom up dp
// noinspection JSUnusedLocalSymbols
function cherryPickupBottomUpDp(grid: number[][]): number {
    const n = grid.length;
    const m = grid[0].length;

    const dp: number[][][] = [...Array(n)].map(_ => [...Array(m)].map(_ => Array(m).fill(0)));

    for (let i = n - 1; i >= 0; i--) {
        for (let r1 = 0; r1 < m - 1; r1++) {
            for (let r2 = r1 + 1; r2 < m; r2++) {
                let max_next = 0;
                if (i < n - 1) {
                    for (let nr1 = r1 - 1; nr1 <= r1 + 1; nr1++) {
                        for (let nr2 = r2 - 1; nr2 <= r2 + 1; nr2++) {
                            if (nr1 >= 0 && nr1 < nr2 && nr2 < m) {
                                max_next = Math.max(max_next, dp[i + 1][nr1][nr2]);
                            }
                        }
                    }
                }
                dp[i][r1][r2] = grid[i][r1] + grid[i][r2] + max_next;
            }
        }
    }

    return dp[0][0][m - 1];
}

// top down dp / memoization
// noinspection JSUnusedLocalSymbols
function cherryPickup(grid: number[][]): number {
    const n = grid.length;
    const m = grid[0].length;

    const dp: number[][][] = [...Array(n)].map(_ => [...Array(m)].map(_ => Array(m).fill(-1)));

    const rec = (i: number, r1: number, r2: number): number => {
        if (i == n) return 0;
        else if (dp[i][r1][r2] != -1) return dp[i][r1][r2];
        else {
            let max_next = 0;
            for (let nr1 = r1 - 1; nr1 <= r1 + 1; nr1++) {
                for (let nr2 = r2 - 1; nr2 <= r2 + 1; nr2++) {
                    if (nr1 >= 0 && nr1 < nr2 && nr2 < m) {
                        max_next = Math.max(max_next, rec(i + 1, nr1, nr2));
                    }
                }
            }

            dp[i][r1][r2] = grid[i][r1] + grid[i][r2] + max_next;
            return dp[i][r1][r2];
        }
    };

    return rec(0, 0, m - 1);
}

// noinspection JSUnusedLocalSymbols
function cherryPickupBruteForce(grid: number[][]): number {
    const n = grid.length;
    const m = grid[0].length;

    const rec = (i: number, r1: number, r2: number): number => {
        if (i == n) return 0;
        else {
            let max_next = 0;
            for (let nr1 = r1 - 1; nr1 <= r1 + 1; nr1++) {
                for (let nr2 = r2 - 1; nr2 <= r2 + 1; nr2++) {
                    if (nr1 >= 0 && nr1 < nr2 && nr2 < m) {
                        max_next = Math.max(max_next, rec(i + 1, nr1, nr2));
                    }
                }
            }

            return grid[i][r1] + grid[i][r2] + max_next;
        }
    };

    return rec(0, 0, m - 1);
}

describe('c2020_12_19 Cherry Pickup II', () => {
    it('Example 1: (grid = [[3,1,1],[2,5,1],[1,5,5],[2,1,1]]) -> 24', () => {
        const grid = [
            [3, 1, 1],
            [2, 5, 1],
            [1, 5, 5],
            [2, 1, 1]];
        expect(cherryPickup(grid)).to.eql(24);
        //Explanation:
        // Cherries taken by Robot #1, (3 + 2 + 5 + 2) = 12.
        // Cherries taken by Robot #2, (1 + 5 + 5 + 1) = 12.
        // Total of cherries: 12 + 12 = 24.
    });
    it('Example 2: (grid = [[1,0,0,0,0,0,1],[2,0,0,0,0,3,0],[2,0,9,0,0,0,0],[0,3,0,5,4,0,0],[1,0,2,3,0,0,6]]) -> 28', () => {
        const grid = [
            [1, 0, 0, 0, 0, 0, 1],
            [2, 0, 0, 0, 0, 3, 0],
            [2, 0, 9, 0, 0, 0, 0],
            [0, 3, 0, 5, 4, 0, 0],
            [1, 0, 2, 3, 0, 0, 6]];
        expect(cherryPickup(grid)).to.eql(28);
        //Explanation:
        // Cherries taken by Robot #1, (1 + 0 + 9 + 5 + 2) = 17.
        // Cherries taken by Robot #2, (1 + 3 + 0 + 4 + 3) = 11.
        // Total of cherries: 17 + 11 = 28.
    });
    it('Example 3: (grid = [[1,0,0,3],[0,0,0,3],[0,0,3,3],[9,0,3,3]]) -> 22', () => {
        const grid = [
            [1, 0, 0, 3],
            [0, 0, 0, 3],
            [0, 0, 3, 3],
            [9, 0, 3, 3]];
        expect(cherryPickup(grid)).to.eql(22);
    });
    it('Example 4: (grid = [[1,1],[1,1]]) -> 4"', () => {
        const grid = [
            [1, 1],
            [1, 1]];
        expect(cherryPickup(grid)).to.eql(4);
    });

    it('(grid = [[1,5],[1,5]]) -> 12"', () => {
        const grid = [
            [1, 5],
            [1, 5]];
        expect(cherryPickup(grid)).to.eql(12);
    });
    it('(grid = [[5,1],[5,1]]) -> 12"', () => {
        const grid = [
            [5, 1],
            [5, 1]];
        expect(cherryPickup(grid)).to.eql(12);
    });
    it('(grid = [[1,1,1],[1,1,1],[1,1,1]]) -> 6"', () => {
        const grid = [
            [1, 1, 1],
            [1, 1, 1],
            [1, 1, 1]];
        expect(cherryPickup(grid)).to.eql(6);
    });
    it('(grid = [[1,2,3],[4,5,6],[7,8,9]]) -> 32"', () => {
        const grid = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]];
        expect(cherryPickup(grid)).to.eql(32);
    });
    it('(grid = [70 x 70 filled with 0]) -> 0"', () => {
        const grid: number[][] = Array(70).fill(0).map(() => Array(70).fill(0));
        expect(cherryPickup(grid)).to.eql(0);
    });
    it('(grid = [70 x 70 filled with 1]) -> 140"', () => {
        const grid: number[][] = Array(70).fill(0).map(() => Array(70).fill(1));
        expect(cherryPickup(grid)).to.eql(140);
    });
});
