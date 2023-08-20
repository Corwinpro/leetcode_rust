use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Solution {}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => return true,
            (Some(p_), Some(q_)) => {
                if p_.borrow().val != q_.borrow().val {
                    return false;
                }
                return Solution::is_same_tree(p_.borrow().left.clone(), q_.borrow().left.clone())
                    && Solution::is_same_tree(
                        p_.borrow().right.clone(),
                        q_.borrow().right.clone(),
                    );
            }
            _ => return false,
        }
    }
}
