import {expect} from 'chai'

function maxCoins(nums: number[]): number {
    if (nums.length == 0) return 0

    let dp = Array<Array<number>>(nums.length)
    for (let i = 0; i < nums.length; i++) {
        dp[i] = Array<number>(nums.length).fill(0)
    }

    for (let s = 1; s <= nums.length; s++) {
        for (let li = 0; li <= (nums.length - s); li++) {
            const ri = li + s - 1
            for (let last = li; last <= ri; last++) {
                const le = li == 0 ? 1 : nums[li - 1]
                const re = ri == nums.length - 1 ? 1 : nums[ri + 1]

                const lSum = last == li ? 0 : dp[li][last - 1]
                const rSum = last == ri ? 0 : dp[last + 1][ri]

                dp[li][ri] = Math.max(dp[li][ri], lSum + le * nums[last] * re + rSum)
            }
        }
    }

    return dp[0][nums.length - 1]
}

describe('c2020_12_13 Burst Balloons', () => {
    it('Example: ([3,1,5,8]) -> 167', () => {
        expect(maxCoins([3, 1, 5, 8])).to.eql(167)
        //Output: 167
        //Explanation: nums = [3,1,5,8] --> [3,5,8] -->   [3,8]   -->  [8]  --> []
        //             coins =  3*1*5      +  3*5*8    +  1*3*8      + 1*8*1   = 167
    })
    it('([]) -> 0', () => {
        expect(maxCoins([])).to.eql(0)
    })
    it('([1 to 5]) -> 110', () => {
        expect(maxCoins([1, 2, 3, 4, 5])).to.eql(110)
    })
    it('([1 to 10]) -> 2420', () => {
        expect(maxCoins([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])).to.eql(2420)
    })
    it('([10 to 1]) -> 2420', () => {
        expect(maxCoins([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])).to.eql(2420)
    })

    it('([1 to 500]) -> 20,708,501,000', () => {
        let nums = Array<number>(500).fill(0)
        for (let i = 0; i < 500; i++) {
            nums[i] = i + 1
        }
        expect(maxCoins(nums)).to.eql(20708501000)
    })
})
