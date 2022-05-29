pub use std::cell::RefCell;
pub use std::rc::Rc;

#[macro_export]
macro_rules! D {
    ($x: expr) => {
        println!("{} = {:?}", stringify!($x), $x)
    };

    ($tag: expr, $x: expr) => {
        println!("{} = {:?}", $tag, $x)
    };
}

#[macro_export]
macro_rules! point {
    ($x: expr, $y: expr) => {
        Point { x: $x, y: $y }
    };
}

#[macro_export]
macro_rules! tree_node {
    ($val: expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            val: $val,
            left: None,
            right: None,
        })))
    };
    ($val: expr, $left: expr, $right: expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            val: $val,
            left: $left,
            right: $right,
        })))
    };
}

#[macro_export]
macro_rules! list_node {
    ($val: expr, $next: expr) => {
        Some(Box::new(ListNode {
            val: $val,
            next: $next,
        }))
    };
}

#[macro_export]
macro_rules! own {
    ($s: expr) => {
        $s.to_owned()
    };
}

#[macro_export]
macro_rules! T {
    ($s: expr) => {
        ($s).to_string()
    };
}

#[macro_export]
macro_rules! u8str {
    ($s: expr) => {
        String::from_utf8(($s).to_vec()).unwrap()
    };
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Definition for an interval.
#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

impl From<Vec<i32>> for Interval {
    fn from(v: Vec<i32>) -> Self {
        Self::new(v[0], v[1])
    }
}

// Definition for a point.
#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
