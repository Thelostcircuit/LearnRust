// Lesson 14 exercises. Replace each `todo!()` and run `cargo test`.
#![allow(dead_code, unused_variables, unused_imports)]

use std::cell::RefCell;
use std::rc::Rc;

// -----------------------------------------------------------------------------
// 1. A binary tree using Box.

#[derive(Debug)]
pub enum Tree {
    Leaf,
    Node(Box<Tree>, i32, Box<Tree>), // left, value, right
}

impl Tree {
    /// Helper to build a node.
    pub fn node(left: Tree, value: i32, right: Tree) -> Tree {
        Tree::Node(Box::new(left), value, Box::new(right))
    }

    /// Sum of all values in the tree. (Recursion!)
    pub fn sum(&self) -> i32 {
        todo!()
    }

    /// Height of the tree: Leaf = 0, a node = 1 + the taller of its children.
    pub fn depth(&self) -> usize {
        todo!()
    }

    /// Insert a value as a binary search tree: smaller goes left, otherwise right.
    /// Hint: `match self { Tree::Leaf => *self = ..., Tree::Node(left, v, right) => ... }`
    pub fn insert(&mut self, value: i32) {
        todo!()
    }

    /// Return all values in sorted (in-order) order: left subtree, value, right subtree.
    pub fn to_sorted_vec(&self) -> Vec<i32> {
        todo!()
    }
}

// -----------------------------------------------------------------------------
// 2. Shared mutable state with Rc<RefCell<T>>.

/// A bank account that several "cards" share.
#[derive(Debug)]
pub struct Account {
    pub balance: i64,
}

pub struct Card {
    pub account: Rc<RefCell<Account>>,
}

impl Card {
    /// Spend from the shared account if there's enough money. Return whether it worked.
    pub fn spend(&self, amount: i64) -> bool {
        todo!()
    }
}

/// Create an account with `balance`, and `n` cards all sharing it.
pub fn issue_cards(balance: i64, n: usize) -> (Rc<RefCell<Account>>, Vec<Card>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Tree {
        //      5
        //    /   \
        //   3     8
        //  /
        // 1
        Tree::node(
            Tree::node(Tree::node(Tree::Leaf, 1, Tree::Leaf), 3, Tree::Leaf),
            5,
            Tree::node(Tree::Leaf, 8, Tree::Leaf),
        )
    }

    #[test]
    fn test_sum() {
        assert_eq!(sample().sum(), 17);
        assert_eq!(Tree::Leaf.sum(), 0);
    }

    #[test]
    fn test_depth() {
        assert_eq!(sample().depth(), 3);
        assert_eq!(Tree::Leaf.depth(), 0);
    }

    #[test]
    fn test_insert_and_sort() {
        let mut t = Tree::Leaf;
        for v in [5, 2, 8, 1, 9, 3] {
            t.insert(v);
        }
        assert_eq!(t.to_sorted_vec(), vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_shared_account() {
        let (account, cards) = issue_cards(100, 3);
        assert_eq!(cards.len(), 3);
        assert!(cards[0].spend(40));
        assert!(cards[1].spend(40));
        assert!(!cards[2].spend(40)); // only 20 left
        assert_eq!(account.borrow().balance, 20);
        assert_eq!(Rc::strong_count(&account), 4); // the original + 3 cards
    }
}
