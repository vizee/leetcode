use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

pub struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut s = String::new();
        s.push('[');
        let mut nonnull = if root.is_some() { 1 } else { 0 };
        let mut q = VecDeque::new();
        q.push_back(root);
        while let Some(p) = q.pop_front() {
            if nonnull <= 0 {
                break;
            }
            if s.len() > 1 {
                s.push(',');
            }
            if let Some(p) = p {
                nonnull -= 1;
                let mut p = p.borrow_mut();
                s.push_str(p.val.to_string().as_str());
                let left = p.left.take();
                if left.is_some() {
                    nonnull += 1;
                }
                q.push_back(left);
                let right = p.right.take();
                if right.is_some() {
                    nonnull += 1;
                }
                q.push_back(right);
            } else {
                s.push_str("null");
            }
        }
        s.push(']');
        s
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let s = &data[1..data.len() - 1];
        if s.is_empty() {
            return None;
        }
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut root = None;
        let mut it = s
            .split(',')
            .map(|t| t.parse::<i32>().ok())
            .into_iter();
        while let Some(lval) = it.next() {
            if let Some(cur) = q.pop_front() {
                let rval = it.next();
                if let Some(v) = lval {
                    let n = Rc::new(RefCell::new(TreeNode::new(v)));
                    cur.borrow_mut().left = Some(n.clone());
                    q.push_back(n);
                }
                if let Some(Some(v)) = rval {
                    let n = Rc::new(RefCell::new(TreeNode::new(v)));
                    cur.borrow_mut().right = Some(n.clone());
                    q.push_back(n);
                }
            } else {
                let x = Rc::new(RefCell::new(TreeNode::new(lval.unwrap())));
                root = Some(x.clone());
                q.push_back(x);
            }
        }
        root
    }
}
