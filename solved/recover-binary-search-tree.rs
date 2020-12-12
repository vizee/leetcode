impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn collect(v: &mut Vec<i32>, root: &Rc<RefCell<TreeNode>>) {
            if let Some(ref left) = root.borrow().left {
                collect(v, left);
            }
            v.push(root.borrow().val);
            if let Some(ref right) = root.borrow().right {
                collect(v, right);
            }
        }
        fn recover(v: &Vec<i32>, mut i: usize, root: &Rc<RefCell<TreeNode>>) -> usize {
            if let Some(ref left) = root.borrow().left {
                i = recover(v, i, left);
            }
            root.borrow_mut().val = v[i];
            i += 1;
            if let Some(ref right) = root.borrow().right {
                i = recover(v, i, right);
            }
            i
        }
        if let Some(root) = root {
            let mut v = Vec::new();
            collect(&mut v, root);
            v.sort();
            recover(&v, 0, root);
        }
    }
}
