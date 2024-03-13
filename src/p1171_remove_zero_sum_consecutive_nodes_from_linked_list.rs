#![allow(dead_code)]

use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;

    let mut prefix_sum_to_node = HashMap::new();
    prefix_sum_to_node.insert(0, &dummy);

    let mut sum = 0;
    let mut cur = dummy.next.as_ref();
    while let Some(node) = cur {
        sum += node.val;
        cur = node.next.as_ref();
        prefix_sum_to_node.insert(sum, node);
    }

    let mut result = Box::new(ListNode::new(0));
    let mut cur = Some(&mut result);
    let mut sum = 0;

    while let Some(cur_node) = cur {
        sum += cur_node.val;
        if let Some(node) = prefix_sum_to_node.get(&sum) {
            if let Some(next_node) = node.next.as_ref() {
                cur_node.next = Some(Box::new(ListNode::new(next_node.val)));
            }
        }
        cur = cur_node.next.as_mut();
    }

    result.next
}

#[cfg(test)]
mod tests {
    use crate::p1171_remove_zero_sum_consecutive_nodes_from_linked_list::{ListNode, remove_zero_sum_sublists};

    #[test]
    fn test_one() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: -3,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode
                        {
                            val: 1,
                            next: None,
                        })),
                    })),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 1,
                next: None,
            })),
        }));
        let output = remove_zero_sum_sublists(list);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: -3,
                        next: Some(Box::new(ListNode
                        {
                            val: 4,
                            next: None,
                        })),
                    })),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                })),
            })),
        }));
        let output = remove_zero_sum_sublists(list);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: -3,
                        next: Some(Box::new(ListNode
                        {
                            val: -2,
                            next: None,
                        })),
                    })),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: None,
        }));
        let output = remove_zero_sum_sublists(list);
        assert_eq!(expected, output);
    }
}