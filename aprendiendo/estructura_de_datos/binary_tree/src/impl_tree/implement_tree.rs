use std::fmt::Debug;

use crate::models::tree::{BinaryTree, Node};

impl<T: Ord + Debug> BinaryTree<T> {
    pub fn new_binary() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, val: T) {
        if let Some(ref mut root) = self.root {
            root.insert(val);
        } else {
            self.root = Some(Box::new(Node {
                value: val,
                left: None,
                right: None,
            }));
        }
    }
    // Método de ayuda para imprimir el árbol en orden
    pub fn inorder_traversal(node: &Option<Box<Node<T>>>) {
        if let Some(node) = node {
            BinaryTree::inorder_traversal(&node.left);
            println!("{:?}", node.value);
            BinaryTree::inorder_traversal(&node.right);
        }
    }

    // Método para imprimir el árbol en orden
    pub fn print_inorder(&self) {
        BinaryTree::inorder_traversal(&self.root);
    }
}

impl<T: Ord> Node<T> {
    // Método para insertar un valor en el árbol a partir de este nodo
    pub fn insert(&mut self, val: T) {
        if val < self.value {
            if let Some(ref mut left) = self.left {
                left.insert(val);
            } else {
                self.left = Some(Box::new(Node {
                    value: val,
                    left: None,
                    right: None,
                }));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(val);
            } else {
                self.right = Some(Box::new(Node {
                    value: val,
                    left: None,
                    right: None,
                }));
            }
        }
    }
}
