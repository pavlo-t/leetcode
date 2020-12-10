import {expect} from "chai"

function findMissingRanges(nums: number[], lower: number, upper: number): string[] {
    const rangeToStr = (lower: number, upper: number): string => {
        if (lower == upper) return `${lower}`
        else return `${lower}->${upper}`
    }
    if (nums.length == 0) {
        return [rangeToStr(lower, upper)]
    }

    const result: string[] = []

    if (lower < nums[0]) {
        result.push(rangeToStr(lower, nums[0] - 1))
    }

    for (let j = 1; j < nums.length; j++) {
        const i = j - 1
        if (nums[i] + 1 < nums[j]) {
            result.push(rangeToStr(nums[i] + 1, nums[j] - 1))
        }
    }

    if (nums[nums.length - 1] < upper) {
        result.push(rangeToStr(nums[nums.length - 1] + 1, upper))
    }

    return result
}

describe("c2020_12_w2 Missing Ranges", () => {
    it(`Example 1: (nums = [0,1,3,50,75], lower = 0, upper = 99) -> ["2","4->49","51->74","76->99"]`, () => {
        expect(findMissingRanges([0, 1, 3, 50, 75], 0, 99)).to.eql(["2", "4->49", "51->74", "76->99"])
        //Explanation: The ranges are:
        //[2,2] --> "2"
        //[4,49] --> "4->49"
        //[51,74] --> "51->74"
        //[76,99] --> "76->99"
    })
    it(`Example 2: (nums = [], lower = 1, upper = 1) -> ["1"]`, () => {
        expect(findMissingRanges([], 1, 1)).to.eql(["1"])
        //Explanation: The only missing range is [1,1], which becomes "1".
    })
    it(`Example 3: (nums = [], lower = -3, upper = -1) -> ["-3->-1"]`, () => {
        expect(findMissingRanges([], -3, -1)).to.eql(["-3->-1"])
        //Explanation: The only missing range is [-3,-1], which becomes "-3->-1".
    })
    it(`Example 4: (nums = [-1], lower = -1, upper = -1) -> []`, () => {
        expect(findMissingRanges([-1], -1, -1)).to.eql([])
        //Explanation: There are no missing ranges since there are no missing numbers.
    })
    it(`Example 5: (nums = [-1], lower = -2, upper = -1) -> ["-2"]`, () => {
        expect(findMissingRanges([-1], -2, -1)).to.eql(["-2"])
    })

    it(`(nums=[1..100],lower=-1000000000,upper=1000000000) -> ["-1000000000->0","101->1000000000"]`, () => {
        const nums = Array(100).fill(0).map((_, i) => i + 1)
        const expected = ["-1000000000->0", "101->1000000000"]
        expect(findMissingRanges(nums, -1000000000, 1000000000)).to.eql(expected)
    })
})
