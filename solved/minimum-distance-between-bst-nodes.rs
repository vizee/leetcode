impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn walk_bst(min: &mut i32, prev: Option<i32>, root: &Rc<RefCell<TreeNode>>) -> i32 {
            if let Some(ref left) = root.borrow().left {
                let closest = walk_bst(min, prev, left);
                *min = (*min).min(root.borrow().val - closest);
            } else if let Some(prev) = prev {
                *min = (*min).min(root.borrow().val - prev);
            }
            if let Some(ref right) = root.borrow().right {
                *min = (*min).min(right.borrow().val - root.borrow().val);
                walk_bst(min, Some(root.borrow().val), right)
            } else {
                root.borrow().val
            }
        }
        let mut min = i32::max_value();
        if let Some(root) = root {
            walk_bst(&mut min, None, &root);
        }
        min
    }
}
