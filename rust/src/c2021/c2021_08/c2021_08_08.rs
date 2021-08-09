#![allow(dead_code)]
/// Rank Transform of a Matrix
/// ==========================
///
/// Given an `m x n` matrix, return
/// _a new matrix `answer` where `answer[row][col]` is the __rank__ of `matrix[row][col]`_.
///
/// The __rank__ is an __integer__ that represents how large an element is compared to other elements.
/// It is calculated using the following rules:
///
/// - The rank is an integer starting from `1`.
/// - If two elements `p` and `q` are in the __same row or column__, then:
///   - If `p < q` then `rank(p) < rank(q)`
///   - If `p == q` then `rank(p) == rank(q)`
///   - If `p > q` then `rank(p) > rank(q)`
/// - The __rank__ should be as __small__ as possible.
///
/// It is guaranteed that `answer` is unique under the given rules.
///
/// __Constraints:__
///
/// - `m == matrix.length`
/// - `n == matrix[i].length`
/// - `1 <= m, n <= 500`
/// - `-10^9 <= matrix[row][col] <= 10^9`
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/614/week-2-august-8th-august-14th/3874/
struct Solution;
impl Solution {
    /// Approach 1: Sorting + BFS
    ///
    /// https://leetcode.com/problems/rank-transform-of-a-matrix/solution/
    /// https://leetcode.libaoj.in/rank-transform-of-a-matrix.html
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

        let rs = matrix.len();
        let cs = matrix[0].len();

        // link row to col, and link col to row
        // graphs[v]: the connection graph of value v
        let mut gs: HashMap<i32, HashMap<i32, Vec<i32>>> = HashMap::new();
        for r in 0..rs {
            for c in 0..cs {
                let v = matrix[r][c];
                let (ri, ci) = (r as i32, !(c as i32));
                let g = gs.entry(v).or_default();
                g.entry(ri).or_default().push(ci);
                g.entry(ci).or_default().push(ri);
            }
        }
        println!("graph:{:?}", gs);

        // put points into `value2index` dict, grouped by connection
        // use ordered map to help us sort the key automatically
        let mut v2i: BTreeMap<i32, Vec<Vec<(usize, usize)>>> = BTreeMap::new();
        // mark whether put into `value2index` or not
        let mut seen = vec![vec![false; cs]; rs];
        for r in 0..rs {
            for c in 0..cs {
                if !seen[r][c] {
                    seen[r][c] = true;
                    let v = matrix[r][c];
                    let (ri, ci) = (r as i32, !(c as i32));
                    let g = gs.get(&v).unwrap();
                    // store visited row and col
                    let mut rcs = HashSet::new();
                    rcs.insert(ri);
                    rcs.insert(ci);
                    // start bfs
                    let mut q = VecDeque::new();
                    q.push_back(ri);
                    q.push_back(ci);
                    while let Some(i) = q.pop_front() {
                        for &j in g.get(&i).unwrap().iter() {
                            if !rcs.contains(&j) {
                                rcs.insert(j);
                                q.push_back(j);
                            }
                        }
                    }
                    // transform rowcols into points
                    let mut ps = Vec::new();
                    for &i in rcs.iter() {
                        for &j in g.get(&i).unwrap().iter() {
                            if j >= 0 {
                                ps.push((j as usize, !i as usize));
                                seen[j as usize][!i as usize] = true;
                            } else {
                                ps.push((i as usize, !j as usize));
                                seen[i as usize][!j as usize] = true;
                            }
                        }
                    }
                    v2i.entry(v).or_default().push(ps);
                }
            }
        }
        println!("v2i:{:?}", v2i);
        println!("seen:{:?}", seen);

        // the required rank matrix
        let mut result = vec![vec![0; cs]; rs];
        // rowMax[i]: the max rank in i row
        let mut r_max = vec![0; rs];
        // colMax[j]: the max rank in j col
        let mut c_max = vec![0; cs];
        for ps in v2i.values() {
            // update by connected points with same value
            for ps in ps {
                let mut rank = 1;
                for &(r, c) in ps.iter() {
                    rank = rank.max(r_max[r].max(c_max[c]) + 1);
                }
                for &(r, c) in ps.iter() {
                    result[r][c] = rank;
                    r_max[r] = r_max[r].max(rank);
                    c_max[c] = c_max[c].max(rank);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn m_p1p2_p3p4_produces_12_23() {
        let matrix = vv![[1, 2], [3, 4]];
        let e = vv![[1, 2], [2, 3]];
        assert_eq!(Solution::matrix_rank_transform(matrix), e);
        // Explanation:
        // The rank of matrix[0][0] is 1 because it is the smallest integer in its row and column.
        // The rank of matrix[0][1] is 2 because matrix[0][1] > matrix[0][0] and matrix[0][0] is rank 1.
        // The rank of matrix[1][0] is 2 because matrix[1][0] > matrix[0][0] and matrix[0][0] is rank 1.
        // The rank of matrix[1][1] is 3 because matrix[1][1] > matrix[0][1], matrix[1][1] > matrix[1][0],
        // and both matrix[0][1] and matrix[1][0] are rank 2.
    }
    #[test]
    fn m_p7p7_p7p7_produces_11_11() {
        let matrix = vv![[7, 7], [7, 7]];
        let e = vv![[1, 1], [1, 1]];
        assert_eq!(Solution::matrix_rank_transform(matrix), e);
    }
    #[test]
    fn m_p20m21p14_m19p4p19_p22m47p24_m19p4p19_produces_423_134_516_134() {
        let matrix = vv![[20, -21, 14], [-19, 4, 19], [22, -47, 24], [-19, 4, 19]];
        let e = vv![[4, 2, 3], [1, 3, 4], [5, 1, 6], [1, 3, 4]];
        assert_eq!(Solution::matrix_rank_transform(matrix), e);
    }
    #[test]
    fn m_p7p3p6_p1p4p5_p9p8p2_produces_514_123_631() {
        let matrix = vv![[7, 3, 6], [1, 4, 5], [9, 8, 2]];
        let e = vv![[5, 1, 4], [1, 2, 3], [6, 3, 1]];
        assert_eq!(Solution::matrix_rank_transform(matrix), e);
    }

    mod performance {
        use super::*;

        #[test]
        fn m_p1x500x500_produces_1x500x500() {
            let matrix = vec![vec![1; 500]; 500];
            let e = vec![vec![1; 500]; 500];
            assert_eq!(Solution::matrix_rank_transform(matrix), e);
        }
    }
}
