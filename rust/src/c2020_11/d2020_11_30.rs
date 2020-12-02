// @formatter:off
struct Solution;
// @formatter:on
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::get_skyline_range(&buildings)
    }

    fn get_skyline_range(bs: &[Vec<i32>]) -> Vec<Vec<i32>> {
        if bs.is_empty() {
            vec![]
        } else if bs.len() == 1 {
            vec![vec![bs[0][0], bs[0][2]], vec![bs[0][1], 0]]
        } else {
            let (l, r) = bs.split_at(bs.len() / 2);
            Self::merge(Self::get_skyline_range(l), Self::get_skyline_range(r))
        }
    }

    fn merge(left: Vec<Vec<i32>>, right: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(left.len() + right.len());

        let mut li = 0;
        let mut ri = 0;
        let mut ly = 0;
        let mut ry = 0;

        while li < left.len() && ri < right.len() {
            let lx = left[li][0];
            let rx = right[ri][0];
            let x = if lx < rx {
                ly = left[li][1];
                li += 1;
                lx
            } else {
                ry = right[ri][1];
                ri += 1;
                rx
            };

            let y = ly.max(ry);
            Self::update_result(&mut result, x, y);
        }

        for p in &left[li..] { Self::update_result(&mut result, p[0], p[1]); }
        for p in &right[ri..] { Self::update_result(&mut result, p[0], p[1]); }

        result
    }

    fn update_result(result: &mut Vec<Vec<i32>>, x: i32, y: i32) {
        let default = vec![0, 0];
        let last = result.last().unwrap_or(&default);
        if last[1] != y {
            if last[0] == x {
                result.pop();
            }
            result.push(vec![x, y])
        }
    }
}
// @formatter:off

mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn example1_5buildings_produce_8points() {
        let buildings =
            vec![vec![2,9,10],vec![3,7,15],vec![5,12,12],vec![15,20,10],vec![19,24,8]];
        let expected  =
            vec![vec![2,10],vec![3,15],vec![7,12],vec![12,0],vec![15,10],vec![20,8],vec![24,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn example2_8buildings_produce_9points() {
        let buildings = vec![vec![1,5,11],vec![2,7,6],vec![3,9,13],vec![12,16,7],
            vec![14,25,3],vec![19,22,18],vec![23,29,13],vec![24,28,4]];
        let expected = vec![vec![1,11],vec![3,13],vec![9,0],vec![12,7],vec![16,3],
            vec![19,18],vec![22,3],vec![23,13],vec![29,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn test27_2buildings_produce_2points() {
        let buildings = vec![vec![0,2,3],vec![2,5,3]];
        let expected = vec![vec![0,3],vec![5,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }

    #[test]
    fn no_buildings_produce_no_points() {
        let buildings = vec![];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn one_building_produces_2_points() {
        let buildings = vec![vec![1,2,3]];
        let expected: Vec<Vec<i32>> = vec![vec![1,3],vec![2,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn test_b1_5_2b2_4_3_is_p1_2p2_3p4_2p5_0() {
        let buildings = vec![vec![1,5,2],vec![2,4,3]];
        let expected = vec![vec![1,2],vec![2,3],vec![4,2],vec![5,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn test_b1_3_1b2_4_1_is_p() {
        let buildings = vec![vec![1,3,1],vec![2,4,1]];
        let expected = vec![vec![1,1],vec![4,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn test_b1_2_1b2_3_1_is_p() {
        let buildings = vec![vec![1,2,1],vec![2,3,1]];
        let expected = vec![vec![1,1],vec![3,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn test_b1_2_1b3_4_1_is_p() {
        let buildings = vec![vec![1,2,1],vec![3,4,1]];
        let expected = vec![vec![1,1],vec![2,0],vec![3,1],vec![4,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn test_b1_3_1b1_2_2_is_p() {
        let buildings = vec![vec![1,3,1],vec![1,2,2]];
        let expected = vec![vec![1,2],vec![2,1],vec![3,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn test_b1_3_1b2_3_2_is_p() {
        let buildings = vec![vec![1,3,1],vec![2,3,2]];
        let expected = vec![vec![1,1],vec![2,2],vec![3,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn test_b1_4_2b2_3_1_is_p() {
        let buildings = vec![vec![1,4,2],vec![2,3,1]];
        let expected = vec![vec![1,2],vec![4,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn test_b1_2_1b1_2_1_is_p() {
        let buildings = vec![vec![1,2,1],vec![1,2,1]];
        let expected = vec![vec![1,1],vec![2,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn test_b1_2_5b1_3_4b1_4_3b1_5_2b1_6_1_is_p() {
        let buildings = vec![vec![1,2,5],vec![1,3,4],vec![1,4,3],vec![1,5,2],vec![1,6,1]];
        let expected = vec![vec![1,5],vec![2,4],vec![3,3],vec![4,2],vec![5,1],vec![6,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }
    #[test]
    fn test_b1_6_1b1_5_2b1_4_3b1_3_4b1_2_5_is_p() {
        let buildings = vec![vec![1,6,1],vec![1,5,2],vec![1,4,3],vec![1,3,4],vec![1,2,5]];
        let expected = vec![vec![1,5],vec![2,4],vec![3,3],vec![4,2],vec![5,1],vec![6,0]];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }

    #[test]
    fn test_10000_buildings_produce_some_points() {
        let mut buildings = Vec::new();
        let mut rng = rand::thread_rng();
        for li in 1..=10000 {
            let ri = rng.gen_range(li + 1, std::i32::MAX);
            let hi = rng.gen_range(1, std::i32::MAX);
            buildings.push(vec![li, ri, hi]);
        }

        assert!(Solution::get_skyline(buildings).len() >= 2);
    }
}