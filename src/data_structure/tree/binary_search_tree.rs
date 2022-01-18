//! # 二叉查找树或者说二叉搜索树(Binary Search Tree)
use std::{cmp::PartialOrd, fmt::Display};

#[derive(Debug, Clone)]
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
    fn find(&mut self, target: T) -> Option<&mut Self> {
        if target < self.data {
            if let Some(left) = &mut self.left {
                return left.find(target);
            } else {
                // not found
                return None;
            }
        } else if target > self.data {
            if let Some(right) = &mut self.right {
                return right.find(target);
            } else {
                // not found
                return None;
            }
        } else {
            return Some(self);
        }
    }
    fn find_min(&self) -> Option<&Self> {
        if self.left.is_some() {
            return self.left.as_ref().unwrap().find_min();
        } else {
            return Some(self);
        }
    }
    fn find_max(&self) -> Option<&Self> {
        if self.right.is_some() {
            return self.right.as_ref().unwrap().find_max();
        } else {
            return Some(self);
        }
    }
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
    // fn delete(&mut self, target: T) -> Option<Box<TreeNode<T>>> {
    //     if target < self.data {
    //         if let Some(left) = &mut self.left.take() {
    //             self.left = left.delete(target);
    //         }
    //     }
    //     if target > self.data {
    //         if let Some(right) = &mut self.right.take() {
    //             self.right = right.delete(target);
    //         }
    //     }
    //     if target != self.data {
    //         return None;
    //     }

    //     match (self.left.take(), self.right.take()) {
    //         (None, None) => None,
    //         (None, Some(right)) => Some(right),
    //         (Some(left), None) => Some(left),
    //         (Some(mut left), Some(mut right)) => {
    //             if let Some(mut rightmost) = left.find_max() {
    //                 rightmost.left = Some(left);
    //                 rightmost.right = Some(right);
    //                 return Some(Box::new(*rightmost));
    //             }
    //             None
    //         }
    //     }
    // }
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
    // tree.delete(170);
    // println!(
    //     "***********************************  level order  ***********************************"
    // );
    // tree.level_order();
}
