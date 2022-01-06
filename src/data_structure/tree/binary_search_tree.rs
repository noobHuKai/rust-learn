//! # 二叉查找树或者说二叉搜索树(Binary Search Tree)
use std::{cmp::PartialOrd, fmt::Debug};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: PartialOrd + Copy,
{
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    pub fn insert(&mut self, val: T) {
        if val < self.val {
            if let Some(node) = &mut self.left {
                node.insert(val);
            } else {
                self.left = Some(Box::new(TreeNode::new(val)))
            }
        } else if val > self.val {
            if let Some(node) = &mut self.right {
                node.insert(val);
            } else {
                self.right = Some(Box::new(TreeNode::new(val)))
            }
        }
        // 等于不做任何处理，等于也许可以作为更新
    }
    pub fn search(&self, key: T) -> Option<Box<TreeNode<T>>> {
        if key < self.val {
            if let Some(node) = &mut self.left {
                return node.search(key);
            } else {
                return None;
            }
        } else if key > self.val {
            if let Some(node) = &mut self.left {
                return node.search(key);
            } else {
                return None;
            }
        } else {
            let data = Box::new(self.clone().);
            return Some(data);
        }
    }
}
// 前序遍历
fn pre_order<T: Debug>(node: &TreeNode<T>) {
    println!("{:?}", node.val);
    if let Some(left) = &node.left {
        pre_order(&left)
    }
    if let Some(right) = &node.right {
        pre_order(&right)
    }
}
// 中序遍历
// 输出的结果是排好序的
fn infix_order<T: Debug>(node: &TreeNode<T>) {
    if let Some(left) = &node.left {
        infix_order(&left)
    }
    println!("{:?}", node.val);
    if let Some(right) = &node.right {
        infix_order(&right)
    }
}

// 后序遍历
fn after_order<T: Debug>(node: &TreeNode<T>) {
    if let Some(left) = &node.left {
        after_order(&left)
    }
    if let Some(right) = &node.right {
        after_order(&right)
    }
    println!("{:?}", node.val);
}
// 层序遍历
fn level_order<T: Debug>(node: &TreeNode<T>) {
    let mut queue = Vec::new(); //初始化队列
    queue.push(node); //根结点入队列
    while !queue.is_empty() {
        let a = queue.remove(0);
        println!("{:?}", a.val);
        if let Some(b) = &a.left {
            queue.push(b.as_ref());
        }
        if let Some(b) = &a.right {
            queue.push(b.as_ref());
        }
    }
}
#[test]
fn test_binary_tree() {
    let mut tree = TreeNode::new(23);
    tree.insert(16);
    tree.insert(3);
    tree.insert(22);
    tree.insert(45);
    tree.insert(37);
    tree.insert(99);
    tree.insert(35);
    tree.insert(40);

    println!(
        "***********************************  prev  order  ***********************************"
    );
    pre_order(&tree);
    println!(
        "***********************************  infix order  ***********************************"
    );

    infix_order(&tree);
    println!(
        "***********************************  after order  ***********************************"
    );

    after_order(&tree);
    println!(
        "***********************************  level order  ***********************************"
    );

    level_order(&tree);
}
