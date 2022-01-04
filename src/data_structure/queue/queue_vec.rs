//! # Vec实现的队列

#[derive(Debug)]
struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    #[inline]
    fn new() -> Queue<T> {
        Queue { data: Vec::new() }
    }
    fn push(&mut self, data: T) {
        self.data.push(data);
    }
    fn pop(&mut self) -> Option<T> {
        if self.data.len() == 0 {
            return None;
        } else {
            return Some(self.data.remove(0));
        }
    }
    fn length(&self) -> usize {
        self.data.len()
    }
}

#[test]
fn test_queue_vec() {
    let mut q = Queue::new();
    q.push(1);
    q.push(7);
    q.push(9);
    println!("{:?}", &q);
    q.pop();
    println!("{:?}", &q);
}
