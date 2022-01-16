//! # 二叉查找树或者说二叉搜索树(Binary Search Tree)
use std::{cmp::PartialOrd, fmt::Display};

#[derive(Debug)]
struct TreeNode<T> {
    pub data: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}
impl<T> TreeNode<T>
where
    T: PartialOrd + Display + Copy,
{
    #[inline]
    fn new(data: T) -> Self {
        TreeNode {
            data,
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, data: T) {
        if data < self.data {
            if let Some(left) = &mut self.left {
                left.insert(data);
            } else {
                self.left = Some(Box::new(TreeNode::new(data)));
            }
        } else if data > self.data {
            if let Some(right) = &mut self.right {
                right.insert(data);
            } else {
                self.right = Some(Box::new(TreeNode::new(data)));
            }
        } else { // 等于不做修改
        }
    }
    fn find(&mut self, find_data: T) -> Option<&mut Self> {
        if find_data < self.data {
            if let Some(left) = &mut self.left {
                return left.find(find_data);
            } else {
                // not found
                return None;
            }
        } else if find_data > self.data {
            if let Some(right) = &mut self.right {
                return right.find(find_data);
            } else {
                // not found
                return None;
            }
        } else {
            return Some(self);
        }
    }
    fn find_min(&mut self) -> Option<&mut Self> {
        if self.left.is_some() {
            return self.left.as_mut().unwrap().find_min();
        } else {
            return Some(self);
        }
    }
    fn find_max(&mut self) -> Option<&mut Self> {
        if self.right.is_some() {
            return self.right.as_mut().unwrap().find_max();
        } else {
            return Some(self);
        }
    }
    // fn delete(&mut self, data: T) {
    //     if data < self.data {
    //         if let Some(left) = &mut self.left {
    //             left.delete(data);
    //         } else {
    //             // not found
    //         }
    //     } else if data > self.data {
    //         if let Some(right) = &mut self.right {
    //             right.delete(data);
    //         } else {
    //             // not found
    //         }
    //     } else {
    //         if self.left.is_some() && self.right.is_some() {
    //             // 左右子节点都有
    //             let temp = self.right.as_mut().unwrap().find_min();
    //             self.data = temp.unwrap().data;
    //             self.right.as_mut().unwrap().delete(self.data);
    //         } else {
    //             if self.left.is_some() {
    //                 if let Some(right) = &mut self.right {
    //                     self.data = right.data;
    //                     self.left = right.left;
    //                     self.right = right.right;
    //                 }
    //             } else if self.right.is_some() {
    //             } else {
    //             }
    //         }
    //     }
    // }

    pub fn get_node_num(&self) -> usize {
        let left = match &self.left {
            None => 0,
            Some(node) => node.get_node_num(),
        };
        let right = match &self.right {
            None => 0,
            Some(node) => node.get_node_num(),
        };
        return 1 + left + right;
    }
    pub fn get_depth(&self) -> usize {
        let left = match &self.left {
            None => 0,
            Some(node) => node.get_depth(),
        };
        let right = match &self.right {
            None => 0,
            Some(node) => node.get_depth(),
        };
        if left > right {
            return left + 1;
        } else {
            return right + 1;
        }
    }
    // 前序遍历
    fn pre_order(&self) {
        println!("{}", self.data);
        if let Some(left) = &self.left {
            left.pre_order()
        }
        if let Some(right) = &self.right {
            right.pre_order()
        }
    }
    // 中序遍历
    // 输出的结果是排好序的
    fn infix_order(&self) {
        if let Some(left) = &self.left {
            left.infix_order()
        }
        println!("{}", self.data);
        if let Some(right) = &self.right {
            right.infix_order()
        }
    }

    // 后序遍历
    fn after_order(&self) {
        if let Some(left) = &self.left {
            left.after_order()
        }
        if let Some(right) = &self.right {
            right.after_order()
        }
        println!("{}", self.data);
    }
    // 层序遍历
    fn level_order(&self) {
        let mut queue = Vec::new(); //初始化队列
        queue.push(self); //根结点入队列
        while !queue.is_empty() {
            let a = queue.remove(0);
            println!("{}", a.data);
            if let Some(b) = &a.left {
                queue.push(b.as_ref());
            }
            if let Some(b) = &a.right {
                queue.push(b.as_ref());
            }
        }
    }

    fn rightmost_child(&mut self) -> Option<Box<TreeNode<T>>> {
        match self.right {
            Some(ref mut right) => {
                if let Some(t) = right.rightmost_child() {
                    Some(t)
                } else {
                    let mut r = self.right.take();
                    if let Some(ref mut r) = r {
                        self.right = std::mem::replace(&mut r.left, None);
                    }
                    r
                }
            }
            None => None,
        }
    }
}

fn delete<T: PartialOrd+Display+Copy>(mut this: Box<TreeNode<T>>, target: &T) -> Option<Box<TreeNode<T>>> {
    if target < &this.data {
        if let Some(left) = this.left.take() {
            this.left = delete(left, target);
        }
        return Some(this);
    }

    if target > &this.data {
        if let Some(right) = this.right.take() {
            this.left = delete(right, target);
        }
        return Some(this);
    }

    if target != &this.data {
        println!("not found");
        return None;
    }
    match (this.left.take(), this.right.take()) {
        (None, None) => None,
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),

        (Some(mut left), Some(right)) => {
            if let Some(mut rightmost) = left.rightmost_child() {
                rightmost.left = Some(left);
                rightmost.right = Some(right);
                Some(rightmost)
            } else {
                left.right = Some(right);
                Some(left)
            }
        }
    }
}

#[test]
fn test_bst() {
    let mut tree = TreeNode::new(100);

    tree.insert(50);
    tree.insert(200);
    tree.insert(150);
    tree.insert(210);
    tree.insert(170);
    tree.insert(40);
    tree.insert(60);
    tree.insert(10);

    println!(
        "***********************************  prev  order  ***********************************"
    );
    tree.pre_order();

    println!(
        "***********************************  infix order  ***********************************"
    );
    tree.infix_order();

    println!(
        "***********************************  after order  ***********************************"
    );
    tree.after_order();

    println!(
        "***********************************  level order  ***********************************"
    );
    tree.level_order();

    println!("min value : {:?}", tree.find_min());
    println!("max value : {:?}", tree.find_max());

    println!(
        "***********************************  delete data  ***********************************"
    );
    // tree.delete(data);
   let mut a= delete(Box::new(tree), &170);
    println!(
        "***********************************  level order  ***********************************"
    );
    a.as_mut().unwrap().level_order();
}
