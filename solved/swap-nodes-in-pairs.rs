impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        use std::mem::swap;

        let mut h = head;
        let mut next = &mut h;
        let mut v = None;
        while let Some(x) = next {
            match v {
                None => v = Some(&mut x.val as *mut i32),
                Some(p) => {
                    swap(&mut x.val, unsafe {&mut *p});
                    v = None;
                }
            }
            next = &mut x.next;
        }
        h
    }
}
