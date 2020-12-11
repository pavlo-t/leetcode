import {expect} from 'chai'


function removeDuplicates(nums: number[]): number {
    if (nums.length < 2) {
        return nums.length
    } else {
        let l = 0
        for (let r = 2; r < nums.length; r++) {
            if (nums[l] != nums[r]) {
                nums[l + 2] = nums[r]
                l++
            }
        }
        return l + 2
    }
}

// noinspection JSUnusedLocalSymbols
function removeDuplicatesSplice(nums: number[]): number {
    if (nums.length < 2) {
        return nums.length
    } else {
        let l = 0
        let r = 2
        while (r < nums.length) {
            if (nums[l] == nums[r]) {
                nums.splice(r, 1)
            } else {
                l++
                r++
            }
        }
        return l + 2
    }
}

describe('c2020_12_11 Remove Duplicates from Sorted Array II', () => {
    it('Example 1: (nums = [1,1,1,2,2,3]) -> Output: 5, nums = [1,1,2,2,3]', () => {
        const nums = [1, 1, 1, 2, 2, 3]

        const size = removeDuplicates(nums)

        expect(size).to.eql(5)
        expect(nums.slice(0, size)).to.eql([1, 1, 2, 2, 3])
        //Explanation:
        // Your function should return length = 5,
        // with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
        // It doesn't matter what you leave beyond the returned length.
    })
    it('Example 2: (nums = [0,0,1,1,1,1,2,3,3]) -> Output: 7, nums = [0,0,1,1,2,3,3]', () => {
        const nums = [0, 0, 1, 1, 1, 1, 2, 3, 3]

        const size = removeDuplicates(nums)

        expect(size).to.eql(7)
        expect(nums.slice(0, size)).to.eql([0, 0, 1, 1, 2, 3, 3])
        //Explanation:
        // Your function should return length = 7,
        // with the first seven elements of nums being modified to 0, 0, 1, 1, 2, 3 and 3 respectively.
        // It doesn't matter what values are set beyond the returned length.
    })
    it('(nums = []) -> Output: 0, nums = []', () => {
        const nums: number[] = []

        const size = removeDuplicates(nums)

        expect(size).to.eql(0)
        expect(nums.slice(0, size)).to.eql([])
    })
    it('(nums = Array(30_000).fill(1)) -> Output: 2, nums = [1,1]', () => {
        const nums = Array(30000).fill(1)

        const size = removeDuplicates(nums)

        expect(size).to.eql(2)
        expect(nums.slice(0, size)).to.eql([1, 1])
    })
})
