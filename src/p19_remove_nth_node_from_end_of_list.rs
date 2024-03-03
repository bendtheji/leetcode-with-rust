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

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut length = 0;
    let mut cur_ptr = &head;
    while let Some(node) = cur_ptr {
        length += 1;
        cur_ptr = &node.next;
    }

    let mut prev = Some(Box::new(ListNode { val: -1, next: head }));
    let mut ptr = prev.as_mut();

    for _ in 0..length - n {
        ptr = ptr.unwrap().next.as_mut();
    }

    ptr.unwrap().next = ptr.as_mut().unwrap().next.as_mut().unwrap().next.take();
    prev.unwrap().next
}

// get length of list
// remove the (length-n)th element in the list

#[cfg(test)]
mod tests {
    use crate::p19_remove_nth_node_from_end_of_list::{ListNode, remove_nth_from_end};

    #[test]
    fn test_one() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next:
            Some(Box::new(ListNode {
                val: 2,
                next:
                Some(Box::new(ListNode {
                    val: 3,
                    next:
                    Some(Box::new(ListNode {
                        val: 4,
                        next:
                        Some(Box::new(ListNode::new(5))),
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
                    val: 3,
                    next:
                    Some(Box::new(ListNode::new(5))),
                })),
            })),
        }));
        let output = remove_nth_from_end(list, 2);
        assert_eq!(expected, output);
    }
}