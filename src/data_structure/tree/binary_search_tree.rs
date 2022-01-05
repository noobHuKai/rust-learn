//! # 二叉查找树或者说二叉搜索树(Binary Search Tree)

#[derive(Debug, PartialEq, Eq)]
struct TreeNode<T> {
    pub data: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd> TreeNode<T> {
    #[inline]
    pub fn new(data: T) -> Self {
        TreeNode {
            data,
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, data: T) {
        if data < self.data {
            if let Some(ref mut node) = self.left {
                node.insert(data);
            } else {
                self.left = Some(Box::new(TreeNode::new(data)));
            }
        }else if  data > self.data{
            if let Some(ref mut node) = self.right {
                node.insert(data);
            } else {
                self.right = Some(Box::new(TreeNode::new(data)));
            }
        }
    }
}

#[derive(Debug)]
struct Tree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T> Tree<T> {
    fn new() -> Self {
        Self { root: None }
    }
    fn insert(&mut self, data: T) {
        if let Some(ref mut root) = self.root {
            root.insert()
        } else {
            self.root = Some(Box::new(TreeNode::new(data)));
        }
    }
}

#[test]
fn test_bst() {
    let mut tree = Tree::new();
    tree.insert(100);

    println!("{:?}", tree);
}
