import {expect} from 'chai'

function partition(s: string): string[][] {
    let result = [] as string[][];

    const isPalindrome = (l: number, r: number) => {
        while (l < r) {
            if (s[l++] !== s[r--]) return false;
        }
        return true;
    };

    const dfs = (l: number, curr: string[]) => {
        if (l === s.length) {
            result.push(curr);
        }
        for (let r = l; r < s.length; r++) {
            if (isPalindrome(l, r)) {
                dfs(r + 1, [...curr, s.slice(l, r + 1)]);
            }
        }
    };
    dfs(0, []);

    return result;
}

// noinspection JSUnusedLocalSymbols
function partitionDp(s: string): string[][] {
    const dp: boolean[][] = Array(s.length).fill(0).map((_) => Array(s.length).fill(false))
    const result: string[][] = [];

    const dfs = (l: number, curr: string[]) => {
        if (l == s.length) result.push([...curr])
        for (let r = l; r < s.length; r++) {
            if (s[l] == s[r] && (r - l <= 2 || dp[l + 1][r - 1])) {
                dp[l][r] = true
                curr.push(s.substring(l, r + 1))
                dfs(r + 1, curr)
                curr.pop()
            }
        }
    }
    dfs(0, [])

    return result
}

describe('c2020_12_14 Palindrome Partitioning', () => {
    it('Example 1: (s = "aab") -> [["a","a","b"],["aa","b"]]', () => {
        expect(partition("aab")).to.eql([["a", "a", "b"], ["aa", "b"]])
    })
    it('Example 2: (s = "a") -> [["a"]]', () => {
        expect(partition("a")).to.eql([["a"]])
    })

    it('("aaa") -> [["a","a","a"],["aa","a"],["a","aa"],["aaa"]]', () => {
        expect(partition("aaa")).to.eql([["a", "a", "a"], ["a", "aa"], ["aa", "a"], ["aaa"]])
    })
    it('("a" * 16).size -> 2^15', () => {
        expect(partition("a".repeat(16)).length).to.eql(32768)
    })
})
