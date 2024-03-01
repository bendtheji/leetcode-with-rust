#![allow(dead_code)]

use std::mem;

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

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut placeholder = Some(Box::new(ListNode { val: -1, next: None }));
    let mut prev = &mut placeholder;
    let mut curr = &mut head;
    while curr.is_some() {
        if curr.as_ref().unwrap().next.is_some() &&
            curr.as_ref().unwrap().val == curr.as_ref().unwrap().next.as_ref().unwrap().val {
            while curr.as_ref().unwrap().next.is_some() && curr.as_ref().unwrap().val == curr.as_ref().unwrap().next.as_ref().unwrap().val {
                curr = &mut curr.as_mut().unwrap().next;
            }
            curr = &mut curr.as_mut().unwrap().next;
        } else {
            mem::swap(&mut prev.as_mut().unwrap().next, curr);
            prev = &mut prev.as_mut().unwrap().next;
            mem::swap(curr, &mut prev.as_mut().unwrap().next);
        }
    }

    placeholder.unwrap().next
}

#[cfg(test)]
mod tests {
    use crate::p82_remove_duplicates_from_sorted_list_ii::{delete_duplicates, ListNode};

    #[test]
    fn test_one() {
        let list = Some(Box::new(
            ListNode {
                val: 1,
                next: Some(Box::new(
                    ListNode {
                        val: 2,
                        next: Some(Box::new(
                            ListNode {
                                val: 3,
                                next: Some(Box::new(
                                    ListNode {
                                        val: 3,
                                        next: Some(Box::new(
                                            ListNode {
                                                val: 4,
                                                next: Some(Box::new(
                                                    ListNode {
                                                        val: 4,
                                                        next: Some(Box::new(
                                                            ListNode::new(5))),
                                                    })),
                                            })),
                                    })),
                            })),
                    })),
            }));

        let expected = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode::new(5))) })) }));
        let output = delete_duplicates(list);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let list = Some(Box::new(
            ListNode {
                val: 1,
                next: Some(Box::new(
                    ListNode {
                        val: 1,
                        next: Some(Box::new(
                            ListNode {
                                val: 1,
                                next: Some(Box::new(
                                    ListNode {
                                        val: 2,
                                        next: Some(Box::new(
                                            ListNode {
                                                val: 3,
                                                next: None,
                                            })),
                                    })),
                            })),
                    })),
            }));

        let expected = Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode::new(3))) }));
        let output = delete_duplicates(list);
        assert_eq!(expected, output);
    }
}