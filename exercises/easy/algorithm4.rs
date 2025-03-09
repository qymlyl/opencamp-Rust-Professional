/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::fmt::Pointer;


// 子树左边的叶子小于子树的根和右边的叶子
#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord + std::fmt::Display,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord + std::fmt::Display,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + std::fmt::Display,
{
    // 构造函数
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + std::fmt::Display,
{
    // 构造函数
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 向BST中插入一个值
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut root) => root.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    fn search(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |root| root.search(value))
    }

    fn in_order_traversal(&self) {
        if let Some(ref root) = self.root {
            root.in_order_traversal()
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord + std::fmt::Display,
{
    fn insert(&mut self, value: T) {
        self.insert_node(TreeNode::new(value))
    }

    fn insert_node(&mut self, new_node: TreeNode<T>) {
        if new_node.value < self.value {
            // 小于当前value，将值插入左子树
            match self.left {
                Some(ref mut left) => left.insert_node(new_node),
                None => self.left = Some(Box::new(new_node)),
            }
        } else if new_node.value > self.value {
            // 大于当前值，插入右子树
            match self.right {
                Some(ref mut right) => right.insert_node(new_node),
                None => self.right = Some(Box::new(new_node)),
            }
        }

        // 如果值相同，则不做任何处理
    }

    // 查询方法
    fn search(&self, value: T) -> bool {
        if value == self.value {
            true
        } else if value < self.value {
            // 左子树查询
            self.left.as_ref().map_or(false, |left| left.search(value))
        } else {
            // 右子树查询
            self.right
                .as_ref()
                .map_or(false, |right| right.search(value))
        }
    }

    // 中序遍历打印树中的元素
    fn in_order_traversal(&self) {
        if let Some(ref left) = self.left {
            left.in_order_traversal();
        }

        // 打印
        println!("{}", self.value);

        if let Some(ref right) = self.right {
            right.in_order_traversal();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        bst.in_order_traversal();

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
