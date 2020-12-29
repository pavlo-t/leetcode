import {expect} from 'chai';

/**
 * ### Jump Game IV
 *
 * {@link https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3582/}
 */
function minJumps(arr: number[]): number {
    const map = arr.reduce((map, n, i) => {
        const is = map.get(n);
        is ? is.push(i) : map.set(n, [i]);
        return map;
    }, new Map<number, number[]>());

    const queue = new dequeue.Queue(0, undefined);
    const seen: boolean[] = Array(arr.length).fill(false);
    seen[0] = true;

    let steps = 0;

    while (true) {
        const i = queue.dequeue();

        if (i === undefined) {
            steps++;
            queue.enqueue(undefined);
        } else {
            if (i === arr.length - 1) {
                return steps;
            }

            map.get(arr[i])!.forEach((j) => {
                if (!seen[j]) {
                    seen[j] = true;
                    queue.enqueue(j);
                }
            });
            map.set(arr[i], []);

            if (i > 0 && !seen[i - 1]) {
                seen[i - 1] = true;
                queue.enqueue(i - 1);
            }
            if (!seen[i + 1]) {
                seen[i + 1] = true;
                queue.enqueue(i + 1);
            }
        }
    }
}

module dequeue {
    class Node<T> {
        val: T;
        next: Node<T> | undefined;

        constructor(val: T) {
            this.val = val;
        }
    }

    export class Queue<T> {
        private head: Node<T> | undefined;
        private last: Node<T> | undefined;

        constructor(x: T, ...xs: T[]) {
            this.head = new Node(x);
            let curr = this.head;
            xs.forEach(x => {
                const n = new Node(x);
                curr.next = n;
                curr = n;
            });
            this.last = curr;
        }

        enqueue(e: T): void {
            const n = new Node(e);
            if (this.last) {
                this.last.next = n;
            } else {
                this.head = n;
                this.last = n;
            }
            this.last = n;
        }

        dequeue(): T | undefined {
            if (this.head) {
                const val = this.head.val;
                this.head = this.head.next;
                return val;
            } else {
                return undefined;
            }
        }
    }
}

// noinspection JSUnusedLocalSymbols
function minJumpsQueueWithNulls_34_38_ms(arr: number[]): number {
    const map = arr.reduce((map, n, i) => {
        const is = map.get(n);
        is ? is.push(i) : map.set(n, [i]);
        return map;
    }, new Map<number, number[]>());

    const queue: (number | undefined)[] = [0, undefined];
    const seen: boolean[] = Array(arr.length).fill(false);
    seen[0] = true;

    let steps = 0;

    let k = 0;
    while (k < queue.length) {
        const i = queue[k];

        if (i === undefined) {
            steps++;
            queue.length++;
        } else {
            if (i === arr.length - 1) {
                return steps;
            }

            map.get(arr[i])!.forEach((j) => {
                if (!seen[j]) {
                    seen[j] = true;
                    queue.push(j);
                }
            });
            map.set(arr[i], []);

            if (i > 0 && !seen[i - 1]) {
                seen[i - 1] = true;
                queue.push(i - 1);
            }
            if (!seen[i + 1]) {
                seen[i + 1] = true;
                queue.push(i + 1);
            }
        }

        k++;
    }

    throw new Error('Illegal state');
}

// noinspection JSUnusedLocalSymbols
function minJumpsQueueOfTuples_58_60_ms(arr: number[]): number {
    const map = arr.reduce((map, n, i) => {
        const is = map.get(n);
        is ? is.push(i) : map.set(n, [i]);
        return map;
    }, new Map<number, number[]>());

    const queue: [number, number][] = [[0, 0]];
    const seen: boolean[] = Array(arr.length).fill(false);
    seen[0] = true;

    let k = 0;

    while (k < queue.length) {
        const [i, s] = queue[k];

        if (i === arr.length - 1) {
            return s;
        }

        map.get(arr[i])!.forEach((j) => {
            if (!seen[j]) {
                seen[j] = true;
                queue.push([j, s + 1]);
            }
        });
        map.set(arr[i], []);

        if (i > 0 && !seen[i - 1]) {
            seen[i - 1] = true;
            queue.push([i - 1, s + 1]);
        }
        if (!seen[i + 1]) {
            seen[i + 1] = true;
            queue.push([i + 1, s + 1]);
        }

        k++;
    }

    throw new Error('Illegal state');
}

describe('c2020_12_27 Jump Game IV', () => {
    it('Example 1: (arr = [100,-23,-23,404,100,23,23,23,3,404]) -> 3', () => {
        const arr = [100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
        expect(minJumps(arr)).to.eql(3);
        // Explanation:
        // You need three jumps from index 0 --> 4 --> 3 --> 9.
        // Note that index 9 is the last index of the array.
    });
    it('Example 2: (arr = [7]) -> 0', () => {
        const arr = [7];
        expect(minJumps(arr)).to.eql(0);
        // Explanation: Start index is the last index. You don't need to jump.
    });
    it('Example 3: (arr = [7,6,9,6,9,6,9,7]) -> 1', () => {
        const arr = [7, 6, 9, 6, 9, 6, 9, 7];
        expect(minJumps(arr)).to.eql(1);
        // Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
    });
    it('Example 4: (arr = [6,1,9]) -> 2', () => {
        const arr = [6, 1, 9];
        expect(minJumps(arr)).to.eql(2);
    });
    it('Example 5: (arr = [11,22,7,7,7,7,7,7,7,22,13]) -> 3', () => {
        const arr = [11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13];
        expect(minJumps(arr)).to.eql(3);
    });

    it('(arr = 50000 1s) -> 1', () => {
        const arr = Array(50000).fill(1);
        expect(minJumps(arr)).to.eql(1);
    });
    it('(arr = 0..50000) -> 49999', () => {
        const arr = Array(50000).fill(0).map((_, i) => i);
        expect(minJumps(arr)).to.eql(49999);
    });
});