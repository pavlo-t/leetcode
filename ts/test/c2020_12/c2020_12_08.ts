import {expect} from "chai";

function numPairsDivisibleBy60(time: number[]): number {
    let result = 0
    const remainders = Array<number>(60).fill(0)

    for (const t of time) {
        const r = t % 60
        result += remainders[r == 0 ? 0 : (60 - r)]
        remainders[r] += 1
    }

    return result
}

// noinspection JSUnusedLocalSymbols
function numPairsDivisibleBy60_BruteForce(time: number[]): number {
    let result = 0
    for (let i = 0; i < time.length - 1; i++) {
        for (let j = i + 1; j < time.length; j++) {
            if ((time[i] + time[j]) % 60 == 0)
                result++
        }
    }
    return result
}

describe("c2020_12_08 Pairs of Songs With Total Durations Divisible by 60", () => {
    it("Example 1: (time = [30,20,150,100,40]) -> 3", () => {
        const time = [30, 20, 150, 100, 40]
        expect(numPairsDivisibleBy60(time)).to.eql(3)
        //Explanation: Three pairs have a total duration divisible by 60:
        // (time[0] = 30, time[2] = 150): total duration 180
        // (time[1] = 20, time[3] = 100): total duration 120
        // (time[1] = 20, time[4] = 40): total duration 60
    })
    it("Example 2: (time = ) -> 3", () => {
        const time = [60, 60, 60]
        expect(numPairsDivisibleBy60(time)).to.eql(3)
        //Explanation: All three pairs have a total duration of 120, which is divisible by 60.
    })
    it("(time = [10]) -> 0", () => {
        expect(numPairsDivisibleBy60([10])).to.eql(0)
    })
    it("(time = [60000 elements]) -> 0", () => {
        const time = Array<number>(60000).fill(10)
        expect(numPairsDivisibleBy60(time)).to.eql(0)
    })
    it("(time = [60000 elements]) -> 1,799,970,000", () => {
        const time = Array<number>(60000).fill(60)
        expect(numPairsDivisibleBy60(time)).to.eql(1_799_970_000)
    })
    it("(time = [1 to 60000]) -> 29,999,000", () => {
        const time: number[] = []
        for (let i = 1; i <= 60000; i++) {
            time.push(i)
        }
        expect(numPairsDivisibleBy60(time)).to.eql(29_999_000)
    })
})
