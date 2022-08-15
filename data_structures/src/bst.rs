use std::fmt::Display;
type BSTLink<T: PartialOrd + Copy + Display> = Option<Box<BSTNode<T>>>;
#[derive(Clone)]
pub struct BST<T: PartialOrd + Copy + Display> {
    root: BSTLink<T>,
    size: usize,
}
#[derive(Clone)]
struct BSTNode<T: PartialOrd + Copy + Display> {
    left: BSTLink<T>,
    right: BSTLink<T>,
    value: T,
}

impl<T: PartialOrd + Copy + Display> BSTNode<T> {
    fn new(value: T) -> BSTNode<T> {
        BSTNode {
            left: None,
            right: None,
            value,
        }
    }
}

impl<T: PartialOrd + Copy + Display> BST<T> {
    pub fn new() -> Self {
        return Self {
            root: None,
            size: 0,
        };
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }
    pub fn size(&self) -> usize {
        return self.size;
    }
    fn in_order_print(node: &BSTLink<T>) {
        match node {
            Some(n) => {
                BST::in_order_print(&n.left);
                println!("{} ", n.value);
                BST::in_order_print(&n.right);
            }
            None => (),
        }
    }

    pub fn in_order(self) {
        BST::in_order_print(&self.root);
    }

    pub fn contains(&self, value: T) -> bool {
        let mut curr: &Option<Box<BSTNode<T>>> = &self.root;
        loop {
            match curr {
                Some(node) => {
                    if value == node.value {
                        return true;
                    } else if value < node.value {
                        curr = &node.left;
                    } else {
                        curr = &node.right;
                    }
                }
                None => return false,
            }
        }
    }

    pub fn put(&mut self, value: T) {
        let new_node: Option<Box<BSTNode<T>>> = Some(Box::new(BSTNode::new(value)));
        let put_node: &mut Option<Box<BSTNode<T>>> = self.find_put_node(value);
        match put_node {
            Some(node) => {
                if value < node.value {
                    node.left = new_node;
                } else {
                    node.right = new_node;
                }
            }
            None => *put_node = new_node,
        }
        self.size += 1;
    }

    fn find_put_node(&mut self, value: T) -> &mut BSTLink<T> {
        let mut curr: &mut Option<Box<BSTNode<T>>> = &mut self.root;
        loop {
            match curr {
                Some(node) => {
                    if value < node.value {
                        curr = &mut node.left;
                    } else {
                        curr = &mut node.right;
                    }
                }
                None => {
                    return curr;
                }
            }
        }
    }
}

#[cfg(test)]
mod bst_tests {
    use crate::bst::BST;

    #[test]
    fn new() {
        let mut bst: BST<i32> = BST::new();
        assert_eq!(bst.size(), 0);
    }

    #[test]
    fn put() {
        let mut bst: BST<i32> = BST::new();
        let value: i32 = 123;
        bst.put(value);
        assert_eq!(bst.size(), 1);
        assert!(bst.contains(value));
        assert!(!bst.contains(value + 1));
    }
}
