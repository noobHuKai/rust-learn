//! # 单链表

use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode<T> {
    pub data: T,
    pub next: Option<Box<ListNode<T>>>,
}
impl<T> ListNode<T> {
    #[inline]
    fn new(data: T) -> ListNode<T> {
        ListNode { next: None, data }
    }

    fn get_last_node<'a>(&'a mut self) -> &'a mut Self {
        if let Some(ref mut b) = self.next {
            return b.get_last_node();
        }
        self
    }
}
#[derive(PartialEq, Eq, Clone, Debug)]
struct List<T> {
    pub head: Option<Box<ListNode<T>>>,
}
impl<T: Debug> List<T> {
    #[inline]
    fn new() -> Self {
        List { head: None }
    }
    fn push_front(&mut self, data: T) {
        let mut new_node = ListNode::new(data);
        new_node.next = self.head.take();
        self.head = Some(Box::new(new_node));
    }
    fn push_end(&mut self, data: T) {
        let new_node = Some(Box::new(ListNode::new(data)));
        if let Some(ref mut node) = self.head {
            let mut last_node = node.get_last_node();
            last_node.next = new_node;
        } else {
            self.head = new_node;
        }
    }
    fn pop_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let head = self.head.take().unwrap();
        self.head = head.next;
        Some(head.data)
    }
    fn pop_end(&mut self) -> Option<T> {
        let  head = &self.head;
        // if head.is_none() {
        //     return None;
        // }
        // while let Some(mut pre_node) = &head {
        //     if let Some(end_node) = pre_node.next {
        //         if end_node.next.is_none() {
        //             pre_node.next = None;
        //             return Some(end_node.data);
        //         }
        //     }
        //     head = &pre_node.next;
        // }

        return None;
    }
}
impl<T: Debug> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(node) = &self.head {
            let mut a = Some(node);
            while let Some(b) = a {
                write!(f, "{:?} => ", b.data).unwrap();
                a = b.next.as_ref();
            }
        }
        write!(f, "None")
    }
}

#[test]
fn test_linked_list() {
    let mut list = List::new();
    list.push_front(9);
    list.push_front(13);
    list.push_front(15);
    list.push_end(8);
    list.push_front(10);
    list.push_end(777);
    println!("{}", list);

    assert_eq!(list.pop_front(), Some(10));
    println!("{}", list);
    list.pop_end();
}
