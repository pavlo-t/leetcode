import {expect} from 'chai';

function fourSumCount(A: number[], B: number[], C: number[], D: number[]): number {
    const AB = new Map<number, number>();
    for (let i = 0; i < A.length; i++) {
        for (let j = 0; j < B.length; j++) {
            const sum = A[i] + B[j];
            AB.set(sum, (AB.get(sum) || 0) + 1);
        }
    }

    let result = 0;
    for (let k = 0; k < C.length; k++) {
        for (let l = 0; l < D.length; l++) {
            result += AB.get(-C[k] - D[l]) || 0;
        }
    }
    return result;
}

describe('c2020_12_17 4Sum II', () => {
    it('Example: (A=[1,2],B=[-2,-1],C=[-1,2],D=[0,2]) -> 2', () => {
        const A = [1, 2];
        const B = [-2, -1];
        const C = [-1, 2];
        const D = [0, 2];

        expect(fourSumCount(A, B, C, D)).to.eql(2);
        //Explanation:
        //The two tuples are:
        //1. (0, 0, 0, 1) -> A[0] + B[0] + C[0] + D[1] = 1 + (-2) + (-1) + 2 = 0
        //2. (1, 1, 0, 0) -> A[1] + B[1] + C[0] + D[0] = 2 + (-1) + (-1) + 0 = 0
    });
    it('Test 6: ([-1,-1],[-1,1],[-1,1],[1,-1]) -> 6', () => {
        const A = [-1, -1];
        const B = [-1, 1];
        const C = [-1, 1];
        const D = [1, -1];
        expect(fourSumCount(A, B, C, D)).to.eql(6);
    });
    it('(A=[0],B=[0],C=[0],D=[0]) -> 1', () => {
        const a = [0];
        expect(fourSumCount(a, a, a, a)).to.eql(1);
    });
    it('(A=[1 to 500],B=[1 to 500],C=[1 to 500],D=[1 to 500]) -> 0', () => {
        const a = Array(500).fill(0).map((_, i) => i + 1);
        expect(fourSumCount(a, a, a, a)).to.eql(0);
    });

    it('test array vs object vs map', () => {
        const arr = [];
        arr[3] = 4;
        console.log(arr);
        for (const i of arr) {
            console.log('item: ' + i);
        }

        const obj: { [k: number]: number } = {};
        obj[5] = 6;
        console.log(obj);
        for (const i of Object.entries(obj)) {
            console.log('entry: ' + i);
        }

        const map = new Map<number, number>();
        map.set(12, 14);
        console.log(map);
        for (const i of map) {
            console.log('kv: ' + i);
        }
    });
});
