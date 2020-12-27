import {expect} from 'chai';

class ListNode {
    val: number;
    next: ListNode | null;

    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val);
        this.next = (next === undefined ? null : next);
    }
}

/**
 * ### Swap Nodes in Pairs
 *
 * {@link https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3579/}
 */
function swapPairs(head: ListNode | null): ListNode | null {
    if (!head || !head.next) {
        return head;
    }

    const guard = new ListNode(-1);
    let prev = guard;

    while (head && head.next) {
        const n1: ListNode = head;
        const n2: ListNode = head.next;

        prev.next = n2;
        n1.next = n2.next;
        n2.next = n1;

        prev = n1;
        head = n1.next;
    }

    return guard.next;
}

describe('c2020_12_24 Swap Nodes in Pairs', () => {
    const L = (v: number, n?: ListNode | null) => new ListNode(v, n);

    it('Example 1: (head = [1,2,3,4]) -> [2,1,4,3]', () => {
        const head = L(1, L(2, L(3, L(4))));
        const expected = L(2, L(1, L(4, L(3))));

        expect(swapPairs(head)).to.eql(expected);
    });
    it('Example 2: (head = []) -> []', () => {
        expect(swapPairs(null)).to.be.null;
    });
    it('Example 3: (head = [1]) -> [1]', () => {
        expect(swapPairs(L(1))).to.eql(L(1));
    });

    it('(head = [1,2,3]) -> [2,1,3]', () => {
        const head = L(1, L(2, L(3)));
        const expected = L(2, L(1, L(3)));

        expect(swapPairs(head)).to.eql(expected);
    });
});
