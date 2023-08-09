pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let length = nums.len();
        let mut count: usize = length;
        let mut left = 0;
        for j in 0..length {
            if nums[j] != val {
                nums[left] = nums[j];
                left += 1;
            } else {
                count -= 1;
            }
        }

        return count as i32;
    }

    pub fn with_retain(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&n| n != val);
        nums.len() as i32
    }
}
