import {expect} from 'chai'

function sortedSquares(nums: number[]): number[] {
    if (nums.length == 1 || nums[0] >= 0) {
        return nums.map(n => n * n);
    } else {
        let l = 0;
        let r = nums.length - 1;
        const result: number[] = Array(nums.length).fill(0);

        while (l <= r) {
            if (-nums[l] > nums[r]) {
                result[r - l] = nums[l] * nums[l];
                l++;
            } else {
                result[r - l] = nums[r] * nums[r];
                r--;
            }
        }

        return result;
    }
}

// noinspection JSUnusedLocalSymbols
function sortedSquaresMapSort(nums: number[]): number[] {
    nums = nums.map(n => n * n);
    nums.sort((a, b) => a - b);

    return nums;
}

describe('c2020_12_15 Squares of a Sorted Array', () => {
    it('Example 1: (nums = [-4,-1,0,3,10]) -> [0,1,9,16,100]', () => {
        expect(sortedSquares([-4, -1, 0, 3, 10])).to.eql([0, 1, 9, 16, 100]);
        //Explanation: After squaring, the array becomes [16,1,0,9,100].
        //After sorting, it becomes [0,1,9,16,100].
    })
    it('Example 2: (nums = [-7,-3,2,3,11]) -> [4,9,9,49,121]', () => {
        expect(sortedSquares([-7, -3, 2, 3, 11])).to.eql([4, 9, 9, 49, 121]);
    })

    it('(nums = [-10000,-9998,...,-2,1,3,...,9997,9999]) -> [1,4,9,...,100_000_000]', () => {
        const nums: number[] = Array(10000).fill(0)
            .map((_, i) => (i % 2 != 0) ? -(i + 1) : i + 1);
        nums.sort((a, b) => a - b);
        const expected = Array<number>(10000).fill(0).map((_, i) => (i + 1) * (i + 1));

        expect(sortedSquares(nums)).to.eql(expected);
    })

    it('(nums = [-10000,-9998,...,-2,1,3,...,9997,9999]*100) -> [1,4,9,...,100_000_000]*100', () => {
        const repeat = 10;

        let nums: number[] = Array(10000).fill(0)
            .map((_, i) => (i % 2 != 0) ? -(i + 1) : i + 1);
        nums.sort((a, b) => a - b);
        nums = nums.flatMap(n => Array(repeat).fill(n));

        const expected =
            Array(10000).fill(0)
                .map((_, i) => (i + 1) * (i + 1))
                .flatMap((n) => Array(repeat).fill(n))

        expect(sortedSquares(nums)).to.eql(expected);
    })
})