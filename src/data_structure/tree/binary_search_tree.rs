//! # 二叉查找树或者说二叉搜索树(Binary Search Tree)
use std::{
    cmp::PartialOrd,
    fmt::{Debug, Display},
};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<K, V> {
    pub key: K,
    pub val: V,
    pub left: Option<Box<TreeNode<K, V>>>,
    pub right: Option<Box<TreeNode<K, V>>>,
}

impl<K, V> TreeNode<K, V>
where
    K: PartialOrd,
    V: Copy + Display,
{
    #[inline]
    pub fn new(key: K, val: V) -> Self {
        TreeNode {
            key,
            val,
            left: None,
            right: None,
        }
    }
    pub fn insert(&mut self, key: K, val: V) {
        if key < self.key {
            if let Some(node) = &mut self.left {
                node.insert(key, val);
            } else {
                self.left = Some(Box::new(TreeNode::new(key, val)))
            }
        } else if key > self.key {
            if let Some(node) = &mut self.right {
                node.insert(key, val);
            } else {
                self.right = Some(Box::new(TreeNode::new(key, val)))
            }
        } else { // as change
        }
    }
    pub fn change(&mut self, key: K, val: V) {
        if key < self.key {
            if let Some(node) = &mut self.left {
                node.change(key, val);
            } else { // not found
            }
        } else if key > self.key {
            if let Some(node) = &mut self.right {
                node.change(key, val);
            } else { //not found
            }
        } else {
            // change
            self.val = val;
        }
    }
    pub fn search(&mut self, key: K) -> Option<V> {
        if key < self.key {
            if let Some(node) = &mut self.left {
                return node.search(key);
            } else {
                // not found
                return None;
            }
        } else if key > self.key {
            if let Some(node) = &mut self.right {
                return node.search(key);
            } else {
                //not found
                return None;
            }
        } else {
            return Some(self.val);
        }
    }
    // 删除节点
    // pub fn delete(&mut self, key: K) {
    //     if key < self.key {
    //         if let Some(node) = &mut self.left {
    //             node.delete(key);
    //         } else { // not found
    //         }
    //     } else if key > self.key {
    //         if let Some(node) = &mut self.right {
    //             node.delete(key);
    //         } else { // not found
    //         }
    //     } else {
    //         match (&mut self.left, &mut self.right) {
    //             (Some(_), Some(_)) => {}
    //             (None, Some(_)) => {}
    //             (Some(_), None) => {}
    //             (None, None) => {
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
        println!("{}", self.val);
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
        println!("{}", self.val);
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
        println!("{}", self.val);
    }
    // 层序遍历
    fn level_order(&self) {
        let mut queue = Vec::new(); //初始化队列
        queue.push(self); //根结点入队列
        while !queue.is_empty() {
            let a = queue.remove(0);
            println!("{}", a.val);
            if let Some(b) = &a.left {
                queue.push(b.as_ref());
            }
            if let Some(b) = &a.right {
                queue.push(b.as_ref());
            }
        }
    }
}

#[test]
fn test_binary_tree() {
    let mut tree = TreeNode::new(0, 23);
    tree.insert(1, 16);
    tree.insert(2, 3);
    tree.insert(3, 22);
    tree.insert(4, 45);
    tree.insert(5, 37);
    tree.insert(6, 99);
    tree.insert(7, 35);
    tree.insert(8, 40);

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
    tree.insert(8, 41);
    tree.insert(9, 41);

    println!(
        "***********************************  level order  ***********************************"
    );
    tree.level_order();

    println!("node num : {}", &tree.get_node_num());
    println!("depth : {}", &tree.get_depth());
}
