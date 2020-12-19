import {expect} from 'chai';

function increasingTriplet(nums: number[]): boolean {
    let min = nums[0];
    let mid = Infinity;
    for (let i = 1; i < nums.length; i++) {
        if (nums[i] <= min) min = nums[i];
        else if (nums[i] <= mid) mid = nums[i];
        else return true;
    }
    return false;
}

describe('c2020_12_18 Increasing Triplet Subsequence', () => {
    it('Example 1: (nums = [1,2,3,4,5]) -> true', () => {
        expect(increasingTriplet([1, 2, 3, 4, 5])).to.eql(true);
        //Explanation: Any triplet where i < j < k is valid.
    });
    it('Example 2: (nums = [5,4,3,2,1]) -> false', () => {
        expect(increasingTriplet([5, 4, 3, 2, 1])).to.eql(false);
        //Explanation: No triplet exists.
    });
    it('Example 3: (nums = [2,1,5,0,4,6]) -> true', () => {
        expect(increasingTriplet([2, 1, 5, 0, 4, 6])).to.eql(true);
        //Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] == 4 < nums[5] == 6.
    });

    it('(nums = [1,1,1]) -> false', () => {
        expect(increasingTriplet([1, 1, 1])).to.eql(false);
    });
    it('(nums = [2,1,3]) -> false', () => {
        expect(increasingTriplet([2, 1, 3])).to.eql(false);
    });
    it('(nums = [2,2,5,5,4,4]) -> false', () => {
        expect(increasingTriplet([2, 2, 5, 5, 4, 4])).to.eql(false);
    });
    it('(nums = [2,5,4,5,4]) -> true', () => {
        expect(increasingTriplet([2, 5, 4, 5, 4])).to.eql(true);
    });

    it('(nums = [100_000 elements]) -> false', () => {
        const nums = Array(100_000).fill(1);
        expect(increasingTriplet(nums)).to.eql(false);
    });
    it('(nums = [100_000 elements]) -> true', () => {
        const nums = Array(99998).fill(1);
        nums.push(2);
        nums.push(3);
        expect(increasingTriplet(nums)).to.eql(true);
    });
});
