import {expect} from 'chai';

/**
 * ### Largest Rectangle in Histogram
 *
 * {@link https://leetcode.com/explore/featured/card/december-leetcoding-challenge/573/week-5-december-29th-december-31st/3587/}
 */
function largestRectangleArea(heights: number[]): number {
    const stack = [-1];
    let maxArea = 0;

    for (let i = 0; i < heights.length; i++) {
        while (stack.length > 1 && heights[stack[stack.length - 1]] >= heights[i]) {
            const h = heights[stack.pop()!];
            const w = i - stack[stack.length - 1] - 1;
            maxArea = Math.max(maxArea, h * w);
        }
        stack.push(i);
    }
    while (stack.length > 1) {
        const h = heights[stack.pop()!];
        const w = heights.length - stack[stack.length - 1] - 1;
        maxArea = Math.max(maxArea, h * w);
    }

    return maxArea;
}

describe('c2020_12_31 Largest Rectangle in Histogram', () => {
    it('Example: ([2,1,5,6,2,3]) -> 10', () => {
        expect(largestRectangleArea([2, 1, 5, 6, 2, 3])).to.eql(10);
    });
    it('([]) -> 0', () => {
        expect(largestRectangleArea([])).to.eql(0);
    });
    it('([1]) -> 1', () => {
        expect(largestRectangleArea([1])).to.eql(1);
    });
    it('([2,1,5,12,2,3]) -> 12', () => {
        expect(largestRectangleArea([2, 1, 5, 12, 2, 3])).to.eql(12);
    });
});
