#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        enum Direction { R, D, L, U }

        use Direction::*;

        let n = n as usize;
        let m = n - 1;

        (1..=(n * n) as i32)
            .fold((vec![vec![0; n]; n], R, 0, 0),
                  |(mut acc, d, r, c), v| {
                      acc[r][c] = v;
                      match d {
                          R if c < m && acc[r][c + 1] == 0 => (acc, R, r, c + 1),
                          D if r < m && acc[r + 1][c] == 0 => (acc, D, r + 1, c),
                          L if c > 0 && acc[r][c - 1] == 0 => (acc, L, r, c - 1),
                          U if r > 0 && acc[r - 1][c] == 0 => (acc, U, r - 1, c),
                          R => (acc, D, r + 1, c),
                          D => (acc, L, r, c - 1),
                          L => (acc, U, r - 1, c),
                          U => (acc, R, r, c + 1),
                      }
                  }).0
    }

    pub fn generate_matrix_iteration(n: i32) -> Vec<Vec<i32>> {
        enum Direction { Right, Down, Left, Up }
        use Direction::*;

        let n = n as usize;
        let m = n - 1;
        let mut result = vec![vec![0; n]; n];

        let mut direction = Right;
        let mut r = 0;
        let mut c = 0;


        for v in 1..=(n * n) {
            result[r][c] = v as i32;
            match direction {
                Right => if c < m && result[r][c + 1] == 0 {
                    c += 1;
                } else {
                    direction = Down;
                    r += 1;
                },
                Down => if r < m && result[r + 1][c] == 0 {
                    r += 1;
                } else {
                    direction = Left;
                    c -= 1;
                },
                Left => if c > 0 && result[r][c - 1] == 0 {
                    c -= 1;
                } else {
                    direction = Up;
                    r -= 1;
                },
                Up => if r > 0 && result[r - 1][c] == 0 {
                    r -= 1;
                } else {
                    direction = Right;
                    c += 1;
                },
            };
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example2_n1() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }

    #[test]
    fn example1_n3() {
        let expected =
            vec![vec![1, 2, 3],
                 vec![8, 9, 4],
                 vec![7, 6, 5]];
        assert_eq!(Solution::generate_matrix(3), expected);
    }

    #[test]
    fn test_n2() {
        let expected =
            vec![vec![1, 2],
                 vec![4, 3]];
        assert_eq!(Solution::generate_matrix(2), expected);
    }

    #[test]
    fn test_n4() {
        let expected =
            vec![vec![1, 2, 3, 4],
                 vec![12, 13, 14, 5],
                 vec![11, 16, 15, 6],
                 vec![10, 9, 8, 7]];
        assert_eq!(Solution::generate_matrix(4), expected);
    }

    #[test]
    fn test_n20() {
        let expected =
            vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
                vec![76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 21],
                vec![75, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 95, 22],
                vec![74, 143, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 161, 96, 23],
                vec![73, 142, 203, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 219, 162, 97, 24],
                vec![72, 141, 202, 255, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 269, 220, 163, 98, 25],
                vec![71, 140, 201, 254, 299, 336, 337, 338, 339, 340, 341, 342, 343, 344, 311, 270, 221, 164, 99, 26],
                vec![70, 139, 200, 253, 298, 335, 364, 365, 366, 367, 368, 369, 370, 345, 312, 271, 222, 165, 100, 27],
                vec![69, 138, 199, 252, 297, 334, 363, 384, 385, 386, 387, 388, 371, 346, 313, 272, 223, 166, 101, 28],
                vec![68, 137, 198, 251, 296, 333, 362, 383, 396, 397, 398, 389, 372, 347, 314, 273, 224, 167, 102, 29],
                vec![67, 136, 197, 250, 295, 332, 361, 382, 395, 400, 399, 390, 373, 348, 315, 274, 225, 168, 103, 30],
                vec![66, 135, 196, 249, 294, 331, 360, 381, 394, 393, 392, 391, 374, 349, 316, 275, 226, 169, 104, 31],
                vec![65, 134, 195, 248, 293, 330, 359, 380, 379, 378, 377, 376, 375, 350, 317, 276, 227, 170, 105, 32],
                vec![64, 133, 194, 247, 292, 329, 358, 357, 356, 355, 354, 353, 352, 351, 318, 277, 228, 171, 106, 33],
                vec![63, 132, 193, 246, 291, 328, 327, 326, 325, 324, 323, 322, 321, 320, 319, 278, 229, 172, 107, 34],
                vec![62, 131, 192, 245, 290, 289, 288, 287, 286, 285, 284, 283, 282, 281, 280, 279, 230, 173, 108, 35],
                vec![61, 130, 191, 244, 243, 242, 241, 240, 239, 238, 237, 236, 235, 234, 233, 232, 231, 174, 109, 36],
                vec![60, 129, 190, 189, 188, 187, 186, 185, 184, 183, 182, 181, 180, 179, 178, 177, 176, 175, 110, 37],
                vec![59, 128, 127, 126, 125, 124, 123, 122, 121, 120, 119, 118, 117, 116, 115, 114, 113, 112, 111, 38],
                vec![58, 57, 56, 55, 54, 53, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39]];
        assert_eq!(Solution::generate_matrix(20), expected);
    }

    //"(n = 5) -> [[1,2,3,4,5],[16,17,18,19,6],[15,24,25,20,7],[14,23,22,21],[13,12,11,10.9]]"

    #[test]
    fn test_n1000() {
        let result = Solution::generate_matrix(1000);
        assert_eq!(result.len(), 1000);
    }
    //"(n = 1000) -> big matrix" in {
    //  val n = 1000
    //  val result = generateMatrix(n)
    //
    //  result.length shouldBe n
    //}
}
