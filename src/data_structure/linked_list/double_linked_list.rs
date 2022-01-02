//! # 双链表
use std::{
    cell::{RefCell, RefMut},
    fmt::{Debug, Display},
    rc::Rc,
};

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode<T> {
    pub data: T,
    pub next: Option<Rc<RefCell<ListNode<T>>>>,
    pub prev: Option<Rc<RefCell<ListNode<T>>>>,
}
impl<T: Debug> ListNode<T> {
    #[inline]
    fn new(data: T) -> ListNode<T> {
        ListNode {
            data,
            prev: None,
            next: None,
        }
    }
    // fn get_index_node<'a>(&'a mut self, cur: usize, index: usize) -> Rc<RefCell<Self>> {
    //     if cur >= index {
    //         return Rc::new(RefCell::new(self));
    //     }
    //     if let Some(ref mut node) = self.next {
    //        let a = node.borrow_mut().get_index_node(cur + 1, index);
    //        return a
    //     }
    //     self
    // }
    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} => ", self.data)?;

        if let Some(node) = &self.next {
            return node.borrow().display(f);
        }
        write!(f, "")
    }
}
#[derive(PartialEq, Eq, Clone, Debug)]
struct List<T> {
    pub head: Option<Rc<RefCell<ListNode<T>>>>,
    pub length: usize,
}

impl<T: Debug> List<T> {
    #[inline]
    fn new() -> Self {
        List {
            head: None,
            length: 0,
        }
    }
    fn push_front(&mut self, data: T) {
        self.insert(0, data);
    }
    fn push_back(&mut self, data: T) {
        self.insert(self.length, data);
    }

    fn insert(&mut self, index: usize, data: T) {
        let mut new_node = ListNode::new(data);
        self.length += 1;
        match self.head.take() {
            Some(head) => {
                if index == 0 {
                    new_node.next = Some(head.clone());
                    let rc = Rc::new(RefCell::new(new_node));
                    head.borrow_mut().prev = Some(rc.clone());
                    self.head = Some(rc);
                }else{

                }
            }
            None => self.head = Some(Rc::new(RefCell::new(new_node))),
        }
    }
}
impl<T> Display for List<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(head) = &self.head {
            head.borrow().display(f)?
        }
        write!(f, "None")
    }
}
#[test]
fn test_double_linked_list() {
    let mut list = List::new();
    // list.push_back(20);
    list.push_front(100);
    list.push_front(10);
    // list.push_back(21);
    // list.push_back(200);
    // list.insert(0, 50);
    // list.insert(list.length, 40);
    // list.insert(list.length + 1, 41);
    println!("{}", list);
}
