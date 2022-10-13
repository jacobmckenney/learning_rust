mod merge_sort;
mod quick_sort;

use crate::merge_sort::merge_sort;
use crate::quick_sort::quick_sort;
fn main() {
    let mut test = vec![3, 8, 2, 5, 1];
    // let mut ms = merge_sort(test);
    quick_sort(&mut test, 0, 4);
    println!("{:?}", test);
}
