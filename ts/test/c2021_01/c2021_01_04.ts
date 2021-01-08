import {expect} from 'chai';

/**
 * ### Merge Two Sorted Lists
 * {@link https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3592/}
 */
function mergeTwoLists(l1: ListNode | null, l2: ListNode | null): ListNode | null {
    if (!l1 && !l2) return null;
    else if (!l1) return mergeTwoLists(l2, null);
    else if (!l2) return new ListNode(l1.val, mergeTwoLists(l1.next, null));
    else if (l1.val <= l2.val) return new ListNode(l1.val, mergeTwoLists(l1.next, l2));
    else return mergeTwoLists(l2, l1);
}

class ListNode {
    val: number;
    next: ListNode | null;

    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val);
        this.next = (next === undefined ? null : next);
    }
}

describe('c2021_01_04 Merge Two Sorted Lists', () => {
    const l = (v: number, n?: ListNode | null): ListNode | null => new ListNode(v, n);

    it('Example 1: (l1 = [1,2,4], l2 = [1,3,4]) -> [1,1,2,3,4,4]', () => {
        const l1 = l(1, l(2, l(4)));
        const l2 = l(1, l(3, l(4)));
        const e = l(1, l(1, l(2, l(3, l(4, l(4))))));

        expect(mergeTwoLists(l1, l2)).to.eql(e);
    });
    it('Example 2: (l1 = [], l2 = []) -> []', () => {
        expect(mergeTwoLists(null, null)).to.eql(null);
    });
    it('Example 3: (l1 = [], l2 = [0]) -> [0]', () => {
        expect(mergeTwoLists(null, l(0))).to.eql(l(0));
    });
});
