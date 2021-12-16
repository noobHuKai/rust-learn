//! # 二叉树

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    pub fn insert(&mut self, val: i32) {
        if val <= self.val {
            if self.left.is_none() {
                self.left = Some(Box::new(TreeNode::new(val)));
            } else {
                self.left.as_deref_mut().unwrap().insert(val);
            }
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(TreeNode::new(val)));
            } else {
                self.right.as_deref_mut().unwrap().insert(val);
            }
        }
    }
}
// 前序遍历
fn pre_order(node: &Option<&TreeNode>) {
    if let Some(a) = node {
        println!("{}", a.val);
        if let Some(left) = &a.left {
            pre_order(&Some(left.as_ref()))
        }
        if let Some(right) = &a.right {
            pre_order(&Some(right.as_ref()))
        }
    }
}
// 中序遍历
fn infix_order(node: &Option<&TreeNode>) {
    if let Some(a) = node {
        if let Some(left) = &a.left {
            infix_order(&Some(left.as_ref()))
        }
        println!("{}", a.val);
        if let Some(right) = &a.right {
            infix_order(&Some(right.as_ref()))
        }
    }
}

// 后序遍历
fn after_order(node: &Option<&TreeNode>) {
    if let Some(a) = node {
        if let Some(left) = &a.left {
            after_order(&Some(left.as_ref()))
        }
        if let Some(right) = &a.right {
            after_order(&Some(right.as_ref()))
        }
        println!("{}", a.val);
    }
}
// 层序遍历
fn level_order(node: &TreeNode) {
    let mut queue = Vec::new(); //初始化队列
    queue.push(node); //根结点入队列
    while !queue.is_empty() {
        let a = queue.remove(0);
        println!("{}", a.val);
        if a.left.is_some() {
            queue.push(a.left.as_ref().unwrap());
        }
        if a.right.is_some() {
            queue.push(a.right.as_ref().unwrap());
        }
    }
}
#[test]
fn test_binary_tree() {
    let mut tree = TreeNode::new(50);
    tree.insert(5);
    tree.insert(70);
    tree.insert(6);
    tree.insert(9);
    tree.insert(4);
    let a = Some(&tree);
    println!("***********************************  pre order  ***********************************");
    pre_order(&a);
    println!(
        "***********************************  infix order  ***********************************"
    );
    infix_order(&a);
    println!(
        "***********************************  after order  ***********************************"
    );
    after_order(&a);
    println!(
        "***********************************  level order  ***********************************"
    );
    level_order(&tree);
}
