#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution {}

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        // Traverse the chain until we get to the first node to be rotated.
        //
        // Create a placeholder node so that the `head` element itself can be
        // treated in a similar way as non-`head` ones.
        //
        // The `before` node is the one that stands _before_ those nodes that
        // will be reversed.
        let mut pre_head = Box::new(ListNode { val: 0, next: head });
        let mut before = &mut pre_head;
        for _ in 1..left {
            before = before.next.as_mut()?;
        }

        // For all relevant nodes, change the `next` to the previous one
        // Ignore possible "overflows" as it is promised that the number of
        // nodes is consistent with the left / right values.
        let mut first_node = before.next.take();
        let mut second_node = {
            let first_node_ref = first_node.as_mut();
            first_node_ref?.next.take()
        };

        for _ in left..right {
            let third_node = {
                let second_node_ref = second_node.as_mut()?;
                second_node_ref.next.take()
            };
            second_node.as_mut()?.next = first_node;
            first_node = second_node;
            second_node = third_node;
        }

        // Traverse "back" (in the original direction order) to the first node
        // which was changed; That would be the one that `before` was pointing to.
        let mut reverse = first_node.as_mut()?;
        for _ in left..right {
            reverse = reverse.next.as_mut()?;
        }
        // And point it to the node that is the first one not being changed.
        // This joins the chain with the tail.
        reverse.next = second_node;
        // Point the `before` node to the last one changed. This joins the
        // chain with the head.
        before.next = first_node;

        // Finally, return the origina `head` that is owned by the pre-head placeholder
        return pre_head.next;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let mut list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        println!("{:?}", list);

        list = Solution::reverse_between(list, 2, 4);
        println!("{:?}", list);
    }

    #[test]
    fn example_2() {
        let mut list = Some(Box::new(ListNode { val: 1, next: None }));
        println!("{:?}", list);
        list = Solution::reverse_between(list, 1, 1);
        println!("{:?}", list);
    }
}
