//! # 双链表
use std::{
    cell::{RefCell},
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
    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} => ", self.data)?;

        if let Some(node) = &self.next {
            return node.borrow().display(f);
        }
        write!(f, "")
    }
}
fn get_index_node<T>(
    node: Rc<RefCell<ListNode<T>>>,
    cur: usize,
    index: usize,
) -> Rc<RefCell<ListNode<T>>> {
    if cur >= index {
        return node;
    }
    if let Some(node1) = &node.borrow_mut().next {
        let a = get_index_node(node1.clone(), cur + 1, index);
        return a.clone();
    }
    return node;
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct List<T> {
    pub head: Option<Rc<RefCell<ListNode<T>>>>,
    pub length: usize,
}

impl<T: Debug + Copy> List<T> {
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
    // 插入
    fn insert(&mut self, index: usize, data: T) {
        let mut new_node = ListNode::new(data);
        self.length += 1;
        if let Some(ref mut head) = self.head {
            if index == 0 {
                let head = self.head.take().unwrap();
                new_node.next = Some(head.clone());
                let rc = Rc::new(RefCell::new(new_node));
                head.borrow_mut().prev = Some(rc.clone());
                self.head = Some(rc);
            } else {
                let prev_node = get_index_node(head.clone(), 0, index - 1);
                new_node.next = prev_node.borrow_mut().next.take();
                prev_node.clone().borrow_mut().next = Some(Rc::new(RefCell::new(new_node)));
            }
        } else {
            self.head = Some(Rc::new(RefCell::new(new_node)))
        }
    }

    // 删除
    fn delete(&mut self, index: usize) {
        if let Some(ref mut head) = self.head {
            self.length -= 1;
            if index == 0 {
                self.head = head.clone().borrow_mut().next.take();
            } else if index >= self.length {
                let prev_node = get_index_node(head.clone(), 0, self.length - 1);
                prev_node.borrow_mut().next.take();
            } else {
                let prev_node = get_index_node(head.clone(), 0, index - 1);
                let mut next_node = prev_node.borrow_mut().next.take();
                prev_node.borrow_mut().next = next_node.as_mut().unwrap().borrow_mut().next.take();
            }
        }
    }
    // 修改
    fn change(&mut self, mut index: usize, data: T) {
        if let Some(ref mut head) = self.head {
            if index >= self.length {
                index = self.length;
            }
            get_index_node(head.clone(), 0, index).borrow_mut().data = data;
        }
    }
    //  查询
    fn search(&mut self, index: usize) -> Option<T> {
        if let Some(ref mut head) = self.head {
            if index >= self.length {
                return None;
            }
            let data = get_index_node(head.clone(), 0, index).borrow().data;
            return Some(data);
        } else {
            return None;
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
    list.push_back(20);
    list.push_front(100);
    list.push_front(10);
    list.push_back(21);
    list.push_back(200);
    list.insert(0, 50);
    list.insert(list.length, 40);
    list.insert(list.length + 1, 41);
    println!("{}", list);

    list.delete(0);
    list.delete(list.length);
    list.delete(2);
    println!("{}", list);

    list.change(0, 1);
    list.change(list.length, 1);
    list.change(2, 1);
    println!("{}", list);

    println!("{:?}", list.search(0));
    println!("{:?}", list.search(list.length));
    println!("{:?}", list.search(1));
}
