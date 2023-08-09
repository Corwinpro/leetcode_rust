#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn _new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution {}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => return head,
            Some(mut head_) => match head_.next {
                None => return Some(head_),
                Some(mut next) => {
                    let next_next = Solution::swap_pairs(next.next);
                    head_.next = next_next;
                    next.next = Some(head_);
                    return Some(next);
                }
            },
        }
    }
}
