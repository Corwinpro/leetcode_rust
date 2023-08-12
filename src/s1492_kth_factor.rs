use std::ops::{Div, Rem};

pub struct Solution {}

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut k = k;
        let last = (n as f32).sqrt() as i32;
        for i in 1..last + 1 {
            if n.rem(i) == 0 {
                k -= 1;
                if k == 0 {
                    return i;
                }
            }
        }
        let other_last;
        if last * last == n {
            other_last = last;
        } else {
            other_last = last + 1;
        }
        for i in (1..other_last).rev() {
            if n.rem(i) == 0 {
                k -= 1;
                if k == 0 {
                    return n.div(i);
                }
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn twelve() {
        assert_eq!(Solution::kth_factor(12, 1), 1);
        assert_eq!(Solution::kth_factor(12, 2), 2);
        assert_eq!(Solution::kth_factor(12, 3), 3);
        assert_eq!(Solution::kth_factor(12, 4), 4);
        assert_eq!(Solution::kth_factor(12, 5), 6);
        assert_eq!(Solution::kth_factor(12, 6), 12);
    }
    #[test]
    fn case_24() {
        assert_eq!(Solution::kth_factor(24, 6), 8);
    }
    #[test]
    fn case_7() {
        assert_eq!(Solution::kth_factor(7, 1), 1);
        assert_eq!(Solution::kth_factor(7, 2), 7);
    }
    #[test]
    fn case_4() {
        assert_eq!(Solution::kth_factor(4, 1), 1);
        assert_eq!(Solution::kth_factor(4, 2), 2);
        assert_eq!(Solution::kth_factor(4, 3), 4);
        assert_eq!(Solution::kth_factor(4, 4), -1);
    }
}
