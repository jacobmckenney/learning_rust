mod merge_sort;

use crate::merge_sort::merge_sort;
fn main() {
    let mut test = vec![3, 8, 2, 5, 2];
    let mut new = merge_sort(test);
    println!("{:?}", new);
}
