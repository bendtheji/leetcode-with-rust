#![allow(dead_code)]

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

pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut sentinel_left = ListNode::new(10000);
    let mut sentinel_right = ListNode::new(10000);

    let mut left_ptr = &mut sentinel_left;
    let mut right_ptr = &mut sentinel_right;

    while let Some(mut node) = head {
        head = node.next.take();
        if node.val < x {
            left_ptr.next = Some(node);
            left_ptr = left_ptr.next.as_mut().unwrap();
        } else {
            right_ptr.next = Some(node);
            right_ptr = right_ptr.next.as_mut().unwrap();
        }
    }

    left_ptr.next = sentinel_right.next;
    sentinel_left.next
}

#[cfg(test)]
mod tests {
    use crate::p86_partition_list::{ListNode, partition};

    #[test]
    fn test_one() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next:
            Some(Box::new(ListNode {
                val: 4,
                next:
                Some(Box::new(ListNode {
                    val: 3,
                    next:
                    Some(Box::new(ListNode {
                        val: 2,
                        next:
                        Some(Box::new(ListNode {
                            val: 5,
                            next:
                            Some(Box::new(ListNode::new(2))),
                        })),
                    })),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next:
            Some(Box::new(ListNode {
                val: 2,
                next:
                Some(Box::new(ListNode {
                    val: 2,
                    next:
                    Some(Box::new(ListNode {
                        val: 4,
                        next:
                        Some(Box::new(ListNode {
                            val: 3,
                            next:
                            Some(Box::new(ListNode::new(5))),
                        })),
                    })),
                })),
            })),
        }));
        let output = partition(list, 3);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let list = Some(Box::new(ListNode {
            val: 2,
            next:
            Some(Box::new(ListNode {
                val: 1,
                next: None,
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next:
            Some(Box::new(ListNode {
                val: 2,
                next: None,
            })),
        }));
        let output = partition(list, 2);
        assert_eq!(expected, output);
    }
}