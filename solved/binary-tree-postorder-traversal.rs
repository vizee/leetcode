impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        pub fn postorder(v: &mut Vec<i32>, root: &Rc<RefCell<TreeNode>>) {
            if let Some(ref left) = root.borrow().left {
                postorder(v, left);
            }
            if let Some(ref right) = root.borrow().right {
                postorder(v, right);
            }
            v.push(root.borrow().val);
        }
        let mut v = Vec::new();
        if let Some(root) = root {
            postorder(&mut v, &root);
        }
        v
    }
}
