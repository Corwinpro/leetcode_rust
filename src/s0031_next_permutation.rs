pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        if len < 3 {
            nums.reverse();
            return;
        }
        let mut i_mid = len - 1;
        loop {
            if i_mid == 0 {
                nums.reverse();
                return;
            }
            if nums[i_mid] > nums[i_mid - 1] {
                break;
            }
            i_mid -= 1;
        }
        let mut swap_i = i_mid;
        for i in i_mid..len {
            if nums[i] > nums[i_mid - 1] {
                if nums[i] <= nums[swap_i] {
                    swap_i = i;
                }
            } else {
                break;
            }
        }
        nums.swap(i_mid - 1, swap_i);
        nums[i_mid..len].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_two_three() {
        let mut vec = vec![1, 2, 3];
        Solution::next_permutation(&mut vec);
        assert_eq!(vec, vec![1, 3, 2]);
    }

    #[test]
    fn six() {
        let mut vec = vec![5, 2, 3, 4, 6, 1];
        Solution::next_permutation(&mut vec);
        assert_eq!(vec, vec![5, 2, 3, 6, 1, 4]);
    }

    #[test]
    fn reversed() {
        let mut vec = vec![4, 3, 2, 1];
        Solution::next_permutation(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);
    }

    #[test]
    fn with_duplicate() {
        let mut vec = vec![2, 3, 1, 3, 3];
        Solution::next_permutation(&mut vec);
        assert_eq!(vec, vec![2, 3, 3, 1, 3]);
    }
}
