use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => return 0,
            Some(value) => {
                return 1 + std::cmp::max(
                    Solution::max_depth(value.borrow().left.clone()),
                    Solution::max_depth(value.borrow().right.clone()),
                )
            }
        }
    }
}
