import {expect} from 'chai';

/**
 * ### 1697. Checking Existence of Edge Length Limited Paths
 *
 * {@link https://leetcode.com/contest/weekly-contest-220/problems/checking-existence-of-edge-length-limited-paths/}
 */
function distanceLimitedPathsExist(n: number, edgeList: number[][], queries: number[][]): boolean[] {
    edgeList = [...edgeList].sort((es1, es2) => es1[2] - es2[2]);
    queries = queries.map((qs, i) => [...qs, i]).sort((qs1, qs2) => qs1[2] - qs2[2]);
    const ds = new DisjointSet(n);

    const result: boolean[] = Array(queries.length).fill(false);
    let i = 0;
    for (const [p, q, lim, j] of queries) {
        while (i < edgeList.length && edgeList[i][2] < lim) {
            const [u, v] = edgeList[i];
            ds.union(u, v);
            i++;
        }
        result[j] = ds.find(p) == ds.find(q);
    }

    return result;
}

class DisjointSet {
    private readonly parents: number[];
    private readonly ranks: number[];

    constructor(n: number) {
        this.parents = Array(n).fill(0).map((_, i) => i);
        this.ranks = Array(n).fill(0);
    }

    find(n: number): number {
        if (this.parents[n] != n) {
            this.parents[n] = this.find(this.parents[n]);
        }
        return this.parents[n];
    }

    union(x: number, y: number): void {
        x = this.find(x);
        y = this.find(y);
        if (x != y) {
            if (this.ranks[x] < this.ranks[y]) {
                [x, y] = [y, x];
            }
            this.parents[y] = x;
            if (this.ranks[x] == this.ranks[y]) {
                this.ranks[x]++;
            }
        }
    }
}

describe('w220_4 Checking Existence of Edge Length Limited Paths', () => {
    it('Example 1: (n = 3, edgeList = [[0,1,2],[1,2,4],[2,0,8],[1,0,16]], queries = [[0,1,2],[0,2,5]]) -> ' +
        '[false,true]', () => {
        const n = 3;
        const edgeList = [[0, 1, 2], [1, 2, 4], [2, 0, 8], [1, 0, 16]];
        const queries = [[0, 1, 2], [0, 2, 5]];
        expect(distanceLimitedPathsExist(n, edgeList, queries)).to.eql([false, true]);
        // console.log(edgeList);
        // console.log(queries);
        // expect(distanceLimitedPathsExist(n, edgeList, queries)).to.eql([]);
        // console.log(edgeList);
        // console.log(queries);

        // Explanation: Note that there are two overlapping edges between 0 and 1 with distances 2 and 16.
        //
        // For the first query, between 0 and 1 there is no path where each distance is less than 2,
        // thus we return false for this query.
        //
        // For the second query, there is a path (0 -> 1 -> 2) of two edges with distances less than 5,
        // thus we return true for this query.
    });
    it('Example 2: (n = 5, edgeList = [[0,1,10],[1,2,5],[2,3,9],[3,4,13]], queries = [[0,4,14],[1,4,13]]) -> ' +
        '[true,false]', () => {
        const n = 5;
        const edgeList = [[0, 1, 10], [1, 2, 5], [2, 3, 9], [3, 4, 13]];
        const queries = [[0, 4, 14], [1, 4, 13]];
        expect(distanceLimitedPathsExist(n, edgeList, queries)).to.eql([true, false]);
        // console.log(edgeList);
        // console.log(queries);
        // expect(distanceLimitedPathsExist(n, edgeList, queries)).to.eql([]);
        // console.log(edgeList);
        // console.log(queries);
    });
});
