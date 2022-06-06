// noinspection DuplicatedCode,JSUnusedLocalSymbols

import {expect} from "chai";

// Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null

    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
    }
}

/**
 * \#160. Intersection of Two Linked Lists
 * =======================================
 *
 * Given the heads of two singly linked-lists `headA` and `headB`,
 * return _the node at which the two lists intersect_.
 * If the two linked lists have no intersection at all, return `null`.
 *
 * For example, the following two linked lists begin to intersect at node `c1`:
 *
 * ```
 *       a1 - a2
 *              \
 *               c1 - c2 - c3
 *              /
 *  b1 - b2 - b3
 * ```
 *
 * The test cases are generated such that there are no cycles anywhere in the entire linked structure.
 *
 * __Note__ that the linked lists must __retain their original structure__ after the function returns.
 *
 * __Custom Judge:__
 *
 * The inputs to the __judge__ are given as follows (your program is __not__ given these inputs):
 *
 * - `intersectVal` - The value of the node where the intersection occurs.
 *                    This is `0` if there is no intersected node.
 * - `listA` - The first linked list.
 * - `listB` - The second linked list.
 * - `skipA` - The number of nodes to skip ahead in `listA` (starting from the head) to get to the intersected node.
 * - `skipB` - The number of nodes to skip ahead in `listB` (starting from the head) to get to the intersected node.
 *
 * The judge will then create the linked structure based on these inputs and pass the two heads,
 * `headA` and `headB` to your program.
 * If you correctly return the intersected node, then your solution will be __accepted__.
 *
 * __Constraints:__
 *
 * - The number of nodes of `listA` is in the `m`.
 * - The number of nodes of `listB` is in the `n`.
 * - `1 <= m, n <= 30_000`
 * - `1 <= Node.val <= 100_000`
 * - `0 <= skipA < m`
 * - `0 <= skipB < n`
 * - `intersectVal` is `0` if `listA` and `listB` do not intersect.
 * - `intersectVal == listA[skipA] == listB[skipB]` if `listA` and `listB` intersect.
 *
 *
 * __Follow up:__ Could you write a solution that runs in `O(m + n)` time and use only `O(1)` memory?
 *
 * <https://leetcode.com/problems/intersection-of-two-linked-lists>
 */
function getIntersectionNodeMy(headA: ListNode | null, headB: ListNode | null): ListNode | null {
    const len = (n: ListNode | null): number => {
        let len = 0
        let curr = n
        while (curr !== null) {
            len++
            curr = curr.next
        }
        return len
    }
    const lenToSkip = (lenA: number, lenB: number): [number, number] => {
        if (lenA > lenB) {
            return [lenA - lenB, 0]
        } else if (lenA < lenB) {
            return [0, lenB - lenA]
        } else {
            return [0, 0]
        }
    }
    const skip = (n: ListNode | null, len: number): ListNode | null => {
        let curr = n
        while (len > 0) {
            len--
            curr = curr!.next
        }
        return curr
    }

    const [skipA, skipB] = lenToSkip(len(headA), len(headB))
    let currA = skip(headA, skipA)
    let currB = skip(headB, skipB)

    while (currA !== null && currA !== currB) {
        currA = currA.next
        currB = currB!.next
    }

    return currA
}

/** From <https://leetcode.com/problems/intersection-of-two-linked-lists/solution/> */
function getIntersectionNode(headA: ListNode | null, headB: ListNode | null): ListNode | null {
    let pA = headA
    let pB = headB
    while (pA !== pB) {
        pA = pA === null ? headB : pA.next
        pB = pB === null ? headA : pB.next
    }
    return pA
}


describe('160. Intersection of Two Linked Lists', () => {
    const L = (arr: number[], tail: ListNode | null = null) => {
        const head = new ListNode
        let curr = head
        for (const val of arr) {
            curr.next = new ListNode(val)
            curr = curr.next
        }
        curr.next = tail
        return head.next
    }
    it('intersectVal = 8, listA = [4,1,8,4,5], listB = [5,6,1,8,4,5], skipA = 2, skipB = 3', () => {
        //     4 - 1
        //          \
        //           8 - 4 - 5
        //          /
        // 5 - 6 - 1
        const C = L([8, 4, 5])
        const A = L([4, 1], C)
        const B = L([5, 6, 1], C)
        expect(getIntersectionNode(A, B)).to.equal(C)
        // Explanation: The intersected node's value is 8 (note that this must not be 0 if the two lists intersect).
        // From the head of A, it reads as [4,1,8,4,5].
        // From the head of B, it reads as [5,6,1,8,4,5].
        // There are 2 nodes before the intersected node in A;
        // There are 3 nodes before the intersected node in B.
    })
    it('intersectVal = 2, listA = [1,9,1,2,4], listB = [3,2,4], skipA = 3, skipB = 1', () => {
        // 1 - 9 - 1
        //          \
        //           2 - 4
        //          /
        //         3
        const C = L([2, 4])
        const A = L([1, 9, 1], C)
        const B = L([3], C)
        expect(getIntersectionNode(A, B)).to.equal(C)
        // Explanation: The intersected node's value is 2 (note that this must not be 0 if the two lists intersect).
        // From the head of A, it reads as [1,9,1,2,4].
        // From the head of B, it reads as [3,2,4].
        // There are 3 nodes before the intersected node in A;
        // There are 1 node before the intersected node in B.
    })
    it('intersectVal = 0, listA = [2,6,4], listB = [1,5], skipA = 3, skipB = 2', () => {
        // 2 - 6 - 4
        //
        //     1 - 5
        const A = L([2, 6, 4])
        const B = L([1, 5])
        expect(getIntersectionNode(A, B)).to.equal(null)
        // Output: No intersection
        // Explanation: From the head of A, it reads as [2,6,4].
        // From the head of B, it reads as [1,5].
        // Since the two lists do not intersect, intersectVal must be 0, while skipA and skipB can be arbitrary values.
        // Explanation: The two lists do not intersect, so return null.
    })
})