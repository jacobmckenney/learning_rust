type BSTLink<T: PartialOrd + Copy> = Option<Box<BSTNode<T>>>;

pub struct BST<T: PartialOrd + Copy> {
    root: BSTLink<T>,
    size: usize,
}

struct BSTNode<T: PartialOrd + Copy> {
    left: BSTLink<T>,
    right: BSTLink<T>,
    value: T,
}

impl<T: PartialOrd + Copy> BST<T> {
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

    pub fn put(&mut self, value: T) {
        let new_node: BSTNode<T> = BSTNode {
            left: None,
            right: None,
            value: value,
        };

        let mut prev: &BSTLink<T> = &None;
        let mut curr: &BSTLink<T> = &self.root;
        while curr.is_some() {
            prev = curr;
            let currVal: T = curr.as_ref().unwrap().value;
            if (value > currVal) {
                curr = &curr.as_ref().unwrap().right;
            } else {
                // value < currVal
                curr = &curr.as_ref().unwrap().left;
            }
        }
        if prev.is_none() {
            // Tree was empty
            self.root = Some(Box::new(new_node));
        }
        if prev.unwrap().value > value {
        } else if prev.unwrap().value < value {
        }
    }
}
