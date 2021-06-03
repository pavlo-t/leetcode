struct Solution;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 || arr[0] > arr[1] {
            return false;
        }

        let mut seen_the_top = false;
        let (head, tail) = arr.split_at(1);
        let mut prev = head[0];

        for &curr in tail {
            if prev == curr || (seen_the_top && prev < curr) {
                return false;
            }
            if !seen_the_top && prev > curr {
                seen_the_top = true;
            }
            prev = curr
        }

        seen_the_top
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_arr2_1_is_false() {
        assert!(!Solution::valid_mountain_array(vec![2, 1]))
    }

    #[test]
    fn example2_arr3_5_5_is_false() {
        assert!(!Solution::valid_mountain_array(vec![3, 5, 5]))
    }

    #[test]
    fn example3_arr0_3_2_1_is_true() {
        assert!(Solution::valid_mountain_array(vec![0, 3, 2, 1]))
    }

    #[test]
    fn arr1to10000_is_false() {
        assert!(!Solution::valid_mountain_array((1..=10000).collect()));
    }

    #[test]
    fn arr10000to1_is_false() {
        assert!(!Solution::valid_mountain_array((1..=10000).rev().collect()));
    }

    #[test]
    fn arr1to9999_9998_is_true() {
        let mut arr: Vec<i32> = (1..=9999).collect();
        arr.push(9998);
        assert!(Solution::valid_mountain_array(arr))
    }
}
