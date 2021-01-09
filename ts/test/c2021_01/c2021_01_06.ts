import {expect} from 'chai';

/**
 * ### Kth Missing Positive Number
 * {@link https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3594/}
 */
function findKthPositive(arr: number[], k: number): number {
    let l = 0;
    let r = arr.length - 1;
    while (l <= r) {
        const mid = l + Math.floor((r - l) / 2);
        if (arr[mid] - mid - 1 < k) {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    return k + l;
}

// noinspection JSUnusedLocalSymbols
function findKthPositiveLinear(arr: number[], k: number): number {
    let i = 0;
    while (i < arr.length && arr[i] - i - 1 < k) i++;
    return k + i;
}

describe('c2021_01_06 Kth Missing Positive Number', () => {
    it('Example 1: (arr = [2,3,4,7,11], k = 5) -> 9', () => {
        expect(findKthPositive([2, 3, 4, 7, 11], 5)).to.eql(9);
        // Explanation: The missing positive integers are [1,5,6,8,9,10,12,13,...]. The 5th missing positive integer is 9.
    });
    it('Example 2: (arr = [1,2,3,4], k = 2) -> 6', () => {
        expect(findKthPositive([1, 2, 3, 4], 2)).to.eql(6);
        // Explanation: The missing positive integers are [5,6,7,...]. The 2nd missing positive integer is 6.
    });

    it('(arr = [1001..=2000], k = 1000) -> 1000', () => {
        const arr = Array(1000).fill(0).map((_, i) => i + 1001);
        expect(findKthPositive(arr, 1000)).to.eql(1000);
    });
    it('(arr = [1..=1000], k = 1000) -> 2000', () => {
        const arr = Array(1000).fill(0).map((_, i) => i + 1);
        expect(findKthPositive(arr, 1000)).to.eql(2000);
    });
});