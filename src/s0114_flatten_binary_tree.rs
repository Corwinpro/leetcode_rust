use std::cell::RefCell;
use std::rc::Rc;

type Leaf = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Leaf,
    pub right: Leaf,
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
    pub fn flatten(root: &mut Leaf) {
        if let Some(node) = root {
            let mut left = node.borrow_mut().left.take();
            Solution::flatten(&mut left);

            let mut right = node.borrow_mut().right.take();
            Solution::flatten(&mut right);

            node.borrow_mut().right = left;

            fn append_to_right_most(tree_node: &mut Leaf, element: Leaf) {
                *tree_node = match tree_node.take() {
                    None => element,
                    Some(current) => {
                        append_to_right_most(&mut current.borrow_mut().right, element);
                        Some(current)
                    }
                }
            }

            append_to_right_most(root, right);
        }
    }
}
