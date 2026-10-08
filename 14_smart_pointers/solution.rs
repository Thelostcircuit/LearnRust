// Lesson 14 — solutions.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum Tree {
    Leaf,
    Node(Box<Tree>, i32, Box<Tree>),
}

impl Tree {
    pub fn node(left: Tree, value: i32, right: Tree) -> Tree {
        Tree::Node(Box::new(left), value, Box::new(right))
    }

    pub fn sum(&self) -> i32 {
        match self {
            Tree::Leaf => 0,
            Tree::Node(left, value, right) => left.sum() + value + right.sum(),
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            Tree::Leaf => 0,
            Tree::Node(left, _, right) => 1 + left.depth().max(right.depth()),
        }
    }

    pub fn insert(&mut self, value: i32) {
        match self {
            Tree::Leaf => *self = Tree::node(Tree::Leaf, value, Tree::Leaf),
            Tree::Node(left, v, right) => {
                if value < *v {
                    left.insert(value);
                } else {
                    right.insert(value);
                }
            }
        }
    }

    pub fn to_sorted_vec(&self) -> Vec<i32> {
        match self {
            Tree::Leaf => Vec::new(),
            Tree::Node(left, value, right) => {
                let mut out = left.to_sorted_vec();
                out.push(*value);
                out.extend(right.to_sorted_vec());
                out
            }
        }
    }
}

#[derive(Debug)]
pub struct Account {
    pub balance: i64,
}

pub struct Card {
    pub account: Rc<RefCell<Account>>,
}

impl Card {
    pub fn spend(&self, amount: i64) -> bool {
        let mut account = self.account.borrow_mut();
        if account.balance >= amount {
            account.balance -= amount;
            true
        } else {
            false
        }
    }
}

pub fn issue_cards(balance: i64, n: usize) -> (Rc<RefCell<Account>>, Vec<Card>) {
    let account = Rc::new(RefCell::new(Account { balance }));
    let cards = (0..n).map(|_| Card { account: Rc::clone(&account) }).collect();
    (account, cards)
}
