mod bst;
mod hashmap;
mod linked_list;

use crate::bst::BST;
use crate::hashmap::HashMap;
fn main() {
    let mut bst: BST<i32> = BST::new();
    let inserts: Vec<i32> = vec![7, 2, 5, 1, 4];
    for test_value in inserts.iter() {
        bst.put(*test_value);
    }
    bst.in_order();
    let mut hm: HashMap<i32, i32> = HashMap::new(10);
    hm.put(1, 100);
}
