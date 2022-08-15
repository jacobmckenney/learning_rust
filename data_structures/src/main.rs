mod bst;
mod linked_list;
use crate::bst::BST;
fn main() {
    let mut bst: BST<i32> = BST::new();
    let inserts: Vec<i32> = vec![7, 2, 5, 1, 4];
    for test_value in inserts.iter() {
        bst.put(*test_value);
    }
    bst.in_order();
}
