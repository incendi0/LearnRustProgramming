use std::cmp::Ordering;

#[derive(Debug)]
pub struct BstNode {
    val: i32,
    left: Option<Box<BstNode>>,
    right: Option<Box<BstNode>>,
}

#[derive(Debug)]
pub struct Bst {
    root: Option<Box<BstNode>>,
}

impl Bst {
    pub fn new() -> Self {
        Bst { root: None }
    }

    pub fn from_vec(xs: &Vec<i32>) -> Self {
        let mut bst = Self::new();
        for &x in xs {
            bst.insert(x);
        }
        bst
    }

    pub fn search(&self, x: i32) -> bool {
        let mut curr = &self.root;
        while let Some(node) = curr {
            if node.val == x {
                return true;
            } else if node.val > x {
                curr = &node.left;
            } else {
                curr = &node.right;
            }
        }
        false
    }

    pub fn minimum(&self) -> Option<i32> {
        let mut curr = &self.root;
        let mut ret = None;
        while let Some(node) = curr {
            ret = Some(node.val);
            curr = &node.left;
        }
        ret
    }

    pub fn maximum(&self) -> Option<i32> {
        let mut curr = &self.root;
        let mut ret = None;
        while let Some(node) = curr {
            ret = Some(node.val);
            curr = &node.right;
        }
        ret
    }

    pub fn insert(&mut self, x: i32) {
        self.root = Self::insert_helper(self.root.take(), x);
    }

    fn insert_helper(root: Option<Box<BstNode>>, val: i32) -> Option<Box<BstNode>> {
        match root {
            None => Some(Box::new(BstNode {
                val,
                left: None,
                right: None,
            })),
            Some(mut node) => {
                if node.val > val {
                    node.left = Self::insert_helper(node.left, val);
                } else {
                    node.right = Self::insert_helper(node.right, val);
                }
                Some(node)
            }
        }
    }

    pub fn delete(&mut self, val: i32) {
        self.root = Self::delete_helper(self.root.take(), val);
    }

    fn delete_helper(root: Option<Box<BstNode>>, val: i32) -> Option<Box<BstNode>> {
        if root.is_none() {
            return None;
        }
        let mut root = root;
        match root.as_mut().unwrap().val.cmp(&val) {
            Ordering::Greater => {
                let left = root.as_mut().unwrap().left.take();
                root.as_mut().unwrap().left = Self::delete_helper(left, val);
                root
            }
            Ordering::Less => {
                let right = root.as_mut().unwrap().right.take();
                root.as_mut().unwrap().right = Self::delete_helper(right, val);
                root
            }
            Ordering::Equal => {
                if root.as_mut().unwrap().right.is_none() {
                    return root.as_mut().unwrap().left.take();
                }
                if root.as_mut().unwrap().left.is_none() {
                    return root.as_mut().unwrap().right.take();
                }
                let mut tmp = root;
                let (new_right, mut new_head) =
                    Self::delete_min_helper(tmp.as_mut().unwrap().right.take());
                new_head.as_mut().unwrap().right = new_right;
                new_head.as_mut().unwrap().left = tmp.as_mut().unwrap().left.take();
                new_head
            }
        }
    }

    pub fn delete_min(&mut self) {
        if self.root.is_some() {
            self.root = Self::delete_min_helper(self.root.take()).0;
        }
    }

    // return (root, min)
    fn delete_min_helper(
        root: Option<Box<BstNode>>,
    ) -> (Option<Box<BstNode>>, Option<Box<BstNode>>) {
        let mut root = root;
        match root.as_mut().unwrap().left.take() {
            None => (root.as_mut().unwrap().right.take(), root),
            l @ Some(_) => {
                let (lhs, min) = Self::delete_min_helper(l);
                root.as_mut().unwrap().left = lhs;
                (root, min)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bst_new_works() {
        let bst = Bst::new();
        assert!(bst.root.is_none());
    }

    #[test]
    fn bst_from_vec_works() {
        let bst = Bst::from_vec(&vec![3, 5, 2, 4, 1]);
        assert_eq!(bst.root.as_ref().unwrap().val, 3);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().val, 2);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().val, 5);
        assert_eq!(
            bst.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .val,
            1
        );
        assert_eq!(
            bst.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .val,
            4
        );
    }

    #[test]
    fn bst_minimum_maximum_works() {
        let bst = Bst::new();
        assert_eq!(bst.minimum(), None);
        assert_eq!(bst.maximum(), None);

        let bst = Bst::from_vec(&vec![3, 5, 2, 4, 1]);
        assert_eq!(bst.minimum(), Some(1));
        assert_eq!(bst.maximum(), Some(5));
    }

    #[test]
    fn bst_delete_works() {
        let mut bst = Bst::from_vec(&vec![3, 5, 2, 4, 1, 6]);
        // delete order 2,3,5,1,4,6
        bst.delete(2);
        assert_eq!(bst.root.as_ref().unwrap().val, 3);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().val, 1);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().val, 5);
        assert_eq!(
            bst.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .val,
            4
        );
        assert_eq!(
            bst.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .val,
            6
        );

        bst.delete(3);
        assert_eq!(bst.root.as_ref().unwrap().val, 4);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().val, 1);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().val, 5);
        assert_eq!(
            bst.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .val,
            6
        );

        bst.delete(5);
        assert_eq!(bst.root.as_ref().unwrap().val, 4);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().val, 1);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().val, 6);

        bst.delete(1);
        assert_eq!(bst.root.as_ref().unwrap().val, 4);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().val, 6);

        bst.delete(4);
        assert_eq!(bst.root.as_ref().unwrap().val, 6);

        bst.delete(6);
        assert!(bst.root.is_none());
    }

    #[test]
    fn bst_delete_min_works() {
        let mut bst = Bst::from_vec(&vec![3, 5, 2, 4, 1]);
        bst.delete_min();
        assert_eq!(bst.root.as_ref().unwrap().val, 3);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().val, 2);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().val, 5);
        assert!(bst
            .root
            .as_ref()
            .unwrap()
            .left
            .as_ref()
            .unwrap()
            .left
            .is_none());
        assert_eq!(
            bst.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .val,
            4
        );
    }

    #[test]
    fn bst_delete_min_works_2() {
        let mut bst = Bst::from_vec(&vec![3, 5, 6]);
        bst.delete_min();
        assert_eq!(bst.root.as_ref().unwrap().val, 5);
        assert!(bst.root.as_ref().unwrap().left.is_none());
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().val, 6);
    }
}
