pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if *nums.last().unwrap() < target {
            return nums.len() as i32;
        }

        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = (left + right) / 2;
            let mid_value = nums[mid];
            if mid_value > target {
                right = mid;
            } else if mid_value < target {
                left = mid + 1;
            } else {
                return mid as i32;
            }
        }
        return right as i32;
    }
}
#[cfg(test)]
mod tests {
    use crate::s0035_search_insert::Solution;

    #[test]
    fn does_not_exist() {
        let vec = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(vec, 2), 1);
    }

    #[test]
    fn does_not_exist_2() {
        let vec = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(vec, 0), 0);
    }

    #[test]
    fn does_not_exist_3() {
        let vec = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(vec, 7), 4);
    }

    #[test]
    fn exists() {
        let vec = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(vec, 5), 2);
    }

    #[test]
    fn exists_2() {
        let vec = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(vec, 6), 3);
    }

    #[test]
    fn exists_3() {
        let vec = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(vec, 1), 0);
    }
}
