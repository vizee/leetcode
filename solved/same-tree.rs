impl Solution {
    fn is_same_tree_ref(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (&Some(ref p), &Some(ref q)) =>
                p.borrow().val == q.borrow().val
                    && Self::is_same_tree_ref(&p.borrow().left, &q.borrow().left)
                    && Self::is_same_tree_ref(&p.borrow().right, &q.borrow().right),
            (&None, &None) => true,
            _ => false,
        }
    }

    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_same_tree_ref(&p, &q)
    }
}
