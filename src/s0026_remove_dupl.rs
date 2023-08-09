pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let length = nums.len();
        if length == 0 {
            return 0 as i32;
        }

        let mut left = 0;
        for i in 1..length {
            if nums[i] != nums[i - 1] {
                nums[left + 1] = nums[i];
                left += 1;
            }
        }
        nums.truncate(left + 1);
        return (left + 1) as i32;
    }
}

#[cfg(test)]
mod tests {
    use crate::s0026_remove_dupl::Solution;

    #[test]
    fn all_unique() {
        let mut input_v = vec![1, 2, 3];
        let result = Solution::remove_duplicates(&mut input_v);
        assert_eq!(result, 3);
        assert_eq!(input_v, vec![1, 2, 3]);
    }
    #[test]
    fn all_same() {
        let mut input_v = vec![1, 1, 1];
        let result = Solution::remove_duplicates(&mut input_v);
        assert_eq!(result, 1);
        assert_eq!(input_v, vec![1]);
    }
}
