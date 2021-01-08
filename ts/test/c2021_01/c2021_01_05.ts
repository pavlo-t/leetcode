import {expect} from 'chai';

class ListNode {
    val: number;
    next: ListNode | null;

    constructor(val?: number, next?: ListNode | null)
    constructor(val?: number[], next?: ListNode | null)

    constructor(val?: number | number[], next?: ListNode | null) {
        if (Array.isArray(val)) {
            if (val.length < 2) {
                this.val = val.length ? val[0] : 0;
                this.next = null;
            } else {
                this.val = val[0];
                this.next = null;

                let curr: ListNode = this;
                for (let i = 1; i < val.length; i++) {
                    const n = new ListNode(val[i]);
                    curr.next = n;
                    curr = n;
                }
            }
        } else {
            this.val = (val === undefined ? 0 : val);
            this.next = (next === undefined ? null : next);
        }
    }
}

/**
 * ### Remove Duplicates from Sorted List II
 * {@link https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3593/}
 */
function deleteDuplicates(head: ListNode | null): ListNode | null {
    const sentinel = new ListNode();
    let newCurr: ListNode = sentinel;
    let curr: ListNode | null = head;

    while (curr) {
        if (curr.next && curr.val == curr.next.val) {
            while (curr.next && curr.val == curr.next.val) {
                curr = curr.next;
            }
            curr = curr.next;
        } else {
            newCurr.next = new ListNode(curr.val);
            newCurr = newCurr.next;
            curr = curr.next;
        }
    }

    return sentinel.next;
}

describe('c2021_01_05 Remove Duplicates from Sorted List II', () => {
    it('Example 1: (head = [1,2,3,3,4,4,5]) -> [1,2,5]', () => {
        const head = new ListNode([1, 2, 3, 3, 4, 4, 5]);
        const e = new ListNode([1, 2, 5]);
        expect(deleteDuplicates(head)).to.eql(e);
    });
    it('Example 2: (head = [1,1,1,2,3]) -> [2,3]', () => {
        const head = new ListNode([1, 1, 1, 2, 3]);
        const e = new ListNode([2, 3]);
        expect(deleteDuplicates(head)).to.eql(e);
    });

    it('(head = [1,2,3]) -> [1,2,3]', () => {
        const head = new ListNode([1, 2, 3]);
        const e = new ListNode([1, 2, 3]);
        expect(deleteDuplicates(head)).to.eql(e);
    });
    it('(head = [1,1,2,3,3]) -> [2]', () => {
        const head = new ListNode([1, 1, 2, 3, 3]);
        const e = new ListNode([2]);
        expect(deleteDuplicates(head)).to.eql(e);
    });
    it('(head = [1,1,1,2,3,3,3]) -> [2]', () => {
        const head = new ListNode([1, 1, 1, 2, 3, 3, 3]);
        const e = new ListNode([2]);
        expect(deleteDuplicates(head)).to.eql(e);
    });
});
