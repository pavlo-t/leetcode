import {expect} from "chai"

function validMountainArray(arr: number[]): boolean {
    if (arr.length < 3 || arr[0] >= arr[1]) {
        return false
    }

    let [prev, ...tail] = arr
    let seenThePeak = false

    for (const curr of tail) {
        if (prev == curr || (seenThePeak && prev < curr)) {
            return false
        } else if (!seenThePeak && prev > curr) {
            seenThePeak = true
        }
        prev = curr
    }

    return seenThePeak
}

describe("c2020_12_10 Valid Mountain Array", () => {
    it("Example 1: (arr = [2,1]) -> false", () => {
        expect(validMountainArray([2, 1])).to.eql(false)
    })
    it("Example 2: (arr = [3,5,5]) -> false", () => {
        expect(validMountainArray([3, 5, 5])).to.eql(false)
    })
    it("Example 3: (arr = [0,3,2,1]) -> true", () => {
        expect(validMountainArray([0, 3, 2, 1])).to.eql(true)
    })

    it("(arr = [1..10_000]) -> false", () => {
        const arr = [...Array(10000).fill(0).map((_, i) => 1 + i)]
        expect(validMountainArray(arr)).to.eql(false)
    })
    it("(arr = [10_000..1]) -> false", () => {
        const arr = [...Array(10000).fill(0).map((_, i) => 10000 - i)]
        expect(validMountainArray(arr)).to.eql(false)
    })
    it("(arr = [1..9999,9998]) -> true", () => {
        const arr = [...Array(10000).fill(0).map((_, i) => 1 + i)]
        arr[9999] = 9998
        expect(validMountainArray(arr)).to.eql(true)
    })
})
