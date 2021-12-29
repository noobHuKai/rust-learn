//! # 双链表
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::Rc,
};

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode<T> {
    pub data: T,
    pub next: Option<Rc<RefCell<T>>>,
    pub prev: Option<Rc<RefCell<T>>>,
}
impl<T> ListNode<T> {
    #[inline]
    fn new(data: T) -> ListNode<T> {
        ListNode {
            data,
            prev: None,
            next: None,
        }
    }
}
