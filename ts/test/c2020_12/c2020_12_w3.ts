import {expect} from 'chai';


class ListNode {
    val: number;
    next: ListNode | null;

    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val);
        this.next = (next === undefined ? null : next);
    }
}


function plusOne(head: ListNode | null): ListNode | null {
    const sentinel = new ListNode(0, head);
    let lastNonNine = sentinel;

    while (head != null) {
        if (head.val < 9)
            lastNonNine = head;
        head = head.next;
    }

    lastNonNine.val++;
    head = lastNonNine.next;

    while (head != null) {
        head.val = 0;
        head = head.next;
    }

    return sentinel.val > 0 ? sentinel : sentinel.next;
}

describe('c2020_12_w3 Plus One Linked List', () => {
    const L = (v: number, n?: ListNode | null): ListNode | null => new ListNode(v, n);
    it('Example 1: (head = [1,2,3]) -> [1,2,4]', () => {
        const head = L(1, L(2, L(3)));
        const expected = L(1, L(2, L(4)));

        expect(plusOne(head)).to.eql(expected);
    });
    it('Example 2: (head = [0]) -> [1]', () => {
        expect(plusOne(L(0))).to.eql(L(1));
    });

    it('(head = [1,2,9]) -> [1,3,0]', () => {
        const head = L(1, L(2, L(9)));
        const expected = L(1, L(3, L(0)));

        expect(plusOne(head)).to.eql(expected);
    });
    it('(head = [9,9,9]) -> [1,0,0,0]', () => {
        const head = L(9, L(9, L(9)));
        const expected = L(1, L(0, L(0, L(0))));

        expect(plusOne(head)).to.eql(expected);
    });
});