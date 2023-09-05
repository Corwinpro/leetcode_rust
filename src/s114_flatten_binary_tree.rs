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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut left = node.borrow_mut().left.take();
            Solution::flatten(&mut left);

            let mut right = node.borrow_mut().right.take();
            Solution::flatten(&mut right);

            node.borrow_mut().right = left;

            fn append_to_right_most(
                tree_node: &mut Option<Rc<RefCell<TreeNode>>>,
                element: Option<Rc<RefCell<TreeNode>>>,
            ) {
                *tree_node = match tree_node.take() {
                    None => element,
                    Some(current) => {
                        {
                            let mut current_borrow = current.borrow_mut();
                            append_to_right_most(&mut current_borrow.right, element);
                        }
                        Some(current)
                    }
                }
            }

            append_to_right_most(root, right);
        }
    }
}
