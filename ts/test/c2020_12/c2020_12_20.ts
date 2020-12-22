import {expect} from 'chai';

function decodeAtIndex(S: string, K: number): string {
    let size = 0;
    let i = 0;

    while (size < K) {
        const d = parseInt(S[i], 10);
        if (!isNaN(d)) {
            size *= d;
        } else {
            size++;
            if (size == K) {
                return S[i];
            }
        }
        i++;
    }

    while (i > 0) {
        K %= size;
        const d = parseInt(S[--i], 10);
        if (!isNaN(d)) {
            size /= d;
        } else if (K == 0) {
            return S[i];
        } else {
            size--;
        }
    }

    throw new Error('Bad input: ' + S);
}

describe('c2020_12_20 Decoded String at Index', () => {
    it('example1: (S="leet2code3",K=10) -> "o"', () => {
        expect(decodeAtIndex('leet2code3', 10)).to.eql('o');
        // Explanation:
        // The decoded string is "leetleetcodeleetleetcodeleetleetcode".
        // The 10th letter in the string is "o".
    });
    it('Example 2: (S = "ha22", K = 5) -> "h"', () => {
        expect(decodeAtIndex('ha22', 5)).to.eql('h');
        // Explanation:
        // The decoded string is "hahahaha".  The 5th letter is "h".
    });
    it('Example 3: (S = "a2345678999999999999999", K = 1) -> "a"', () => {
        expect(decodeAtIndex('a2345678999999999999999', 1)).to.eql('a');
        // Explanation:
        // The decoded string is "a" repeated 8301530446056247680 times.  The 1st letter is "a".
    });

    it('Test 11: (S = "a23", K = 6) -> "a"', () => {
        expect(decodeAtIndex('a23', 6)).to.eql('a');
    });

    it('(S = "a2345678999999999999999", K = 1_000_000_000) -> "a"', () => {
        expect(decodeAtIndex('a2345678999999999999999', 1_000_000_000)).to.eql('a');
    });
});
