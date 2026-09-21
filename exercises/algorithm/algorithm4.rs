/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}
fn node_add<T: Ord>(v: T, n: &mut Option<Box<TreeNode<T>>>) -> Option<Box<TreeNode<T>>> {
    match n.take() {
        Some(mut b) => {
            if v > b.value {
                b.right = node_add(v, &mut b.right);
            } else if v < b.value {
                b.left = node_add(v, &mut b.left);
            }
            Some(b)
        },
        None => {
            Some(Box::new(TreeNode::new(v)))
        },
    }
}

fn node_search<T: Ord>(v: &T, n: &Option<Box<TreeNode<T>>>) -> bool {
    match n {
        Some(x) => {
            if x.value == *v {
                true
            } else {
                node_search(v, &x.left) || node_search(v, &x.right)
            }
        },
        None => false,
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        self.root = node_add(value, &mut self.root);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        node_search(&value, &self.root)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


