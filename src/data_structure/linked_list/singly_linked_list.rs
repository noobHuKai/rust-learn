use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode<T> {
    pub data: T,
    pub next: Option<Box<ListNode<T>>>,
}
impl<T> ListNode<T> {
    // 新建一个节点
    #[inline]
    fn new(data: T) -> ListNode<T> {
        ListNode { next: None, data }
    }
    // 获取最后的节点
    fn get_last_node(&mut self) -> &mut Self {
        if let Some(ref mut node) = self.next {
            return node.get_last_node();
        }
        self
    }
    // 根据索引查找节点
    fn get_index_node(&mut self, cur: usize, index: usize) -> &mut Self {
        if cur >= index {
            return self;
        }
        if let Some(ref mut node) = self.next {
            return node.get_index_node(cur + 1, index);
        }
        self
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct List<T> {
    pub head: Option<Box<ListNode<T>>>,
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
    // 头插
    fn push_front(&mut self, data: T) {
        self.insert(0, data);
    }
    // 尾插
    fn push_back(&mut self, data: T) {
        self.insert(self.length, data);
    }
    // 插入
    fn insert(&mut self, index: usize, data: T) {
        let mut new_node = ListNode::new(data);
        if let Some(ref mut head) = self.head {
            if index == 0 {
                let head = self.head.take();
                new_node.next = head;
                self.head = Some(Box::new(new_node));
            } else {
                let mut prev_node = head.get_index_node(0, index - 1);
                let next_node = prev_node.next.take();
                new_node.next = next_node;
                prev_node.next = Some(Box::new(new_node));
            }
        } else {
            self.head = Some(Box::new(new_node));
        }
        self.length += 1
    }
    // 删除
    fn delete(&mut self, index: usize) {
        if let Some(ref mut head) = self.head {
            self.length -= 1;
            if index == 0 {
                self.head = head.next.take();
            } else if index >= self.length {
                let prev_node = head.get_index_node(0, self.length - 1);
                prev_node.next.take();
            } else {
                let mut prev_node = head.get_index_node(0, index - 1);
                let mut next_node = prev_node.next.take();
                prev_node.next = next_node.as_mut().unwrap().next.take();
            }
        }
    }
    // 修改
    fn change(&mut self, mut index: usize, data: T) {
        if let Some(ref mut head) = self.head {
            if index >= self.length {
                index = self.length;
            }
            let mut node = head.get_index_node(0, index);
            node.data = data;
        }
    }
    //  查询
    fn search(&mut self, index: usize) -> Option<T> {
        if let Some(ref mut head) = self.head {
            if index >= self.length {
                return None;
            }
            let node = head.get_index_node(0, index);
            let data = node.data;
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
            let mut head = Some(head);
            while let Some(node) = head {
                write!(f, "{:?} => ", node.data).unwrap();
                head = node.next.as_ref();
            }
        }
        write!(f, "None")
    }
}

#[test]
fn test_singly_linked_list() {
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
