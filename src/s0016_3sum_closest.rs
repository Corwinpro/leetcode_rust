use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut best = nums[0] + nums[1] + nums[2] - target;
        for (i, num) in nums.iter().enumerate() {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let current = num + nums[left] + nums[right] - target;
                if current.abs() < best.abs() {
                    best = current;
                }
                match current.cmp(&0) {
                    Ordering::Greater => right -= 1,
                    Ordering::Equal => return target,
                    Ordering::Less => left += 1,
                }
            }
        }
        return best + target;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let vec = vec![-1, 2, 1, -4];
        assert_eq!(Solution::three_sum_closest(vec, 1), 2);
    }
    #[test]
    fn all_zeroes() {
        let vec = vec![0, 0, 0];
        assert_eq!(Solution::three_sum_closest(vec, 1), 0);
    }
}
