//! #




#[derive(Debug,Copy, Clone)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }

    fn get_last_node(&mut self) -> &mut Self {
        if let Some(ref mut node) = self.next {
            return node.get_last_node();
        }
        self
    }
}

#[derive(Debug,Copy,Clone)]
struct Queue<T> {
    data: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            data: None,
            length: 0,
        }
    }
    fn push(&mut self, data: T) {
        // push end
        if let Some(ref mut head) = self.data {
            let mut last_node = head.get_last_node();
            last_node.next = Some(Box::new(Node::new(data)));
        } else {
            self.data = Some(Box::new(Node::new(data)));
        }
        self.length += 1
    }
    fn pop(&mut self) -> Option<T> {
        if let Some(head) = &self.data {
            self.length -= 1;
            self.data = head.next.take();
            return Some(head.data);
        }
        None
    }
    fn length(&self) -> usize {
        self.length
    }
}

#[test]
fn test_queue_linked_list() {
    let mut q = Queue::new();
    q.push(1);
    q.push(7);
    q.push(9);
    println!("{:?}", &q);
    q.pop();
    println!("{:?}", &q);
}

