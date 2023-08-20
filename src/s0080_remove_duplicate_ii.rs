pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        let mut left = 0;
        let mut count = 1;
        for right in 1..len {
            if nums[left] == nums[right] {
                if count < 2 {
                    left += 1;
                    nums.swap(left, right);
                }
                count += 1;
            } else {
                left += 1;
                nums.swap(left, right);
                count = 1;
            }
        }
        return left as i32 + 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exampe_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![1, 1, 2, 2, 3, 1]);
    }

    #[test]
    fn exampe_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 7);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 3, 3, 1, 1]);
    }
}
