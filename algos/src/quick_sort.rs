pub fn quick_sort(data: &mut Vec<i32>, lo: usize, hi: usize) {
    &&    // pick pivot - median of 3
    println!("data: {:?}", data);
    println!("lo: {}", lo);
    println!("hi: {}", hi);
    if (lo >= hi) {
        return;
    }
    let pivot = median_of_three(data, lo, hi);
    // partition
    let mut i = lo - 1;
    (lo..hi).for_each(|index| {
        if data[index] < data[pivot] {
            i = i + 1;
            data.swap(i, index);
        }
    });
    data.swap(hi, i + 1);
    quick_sort(data, lo, i);
    quick_sort(data, i + 2, hi);
}

fn median_of_three(data: &Vec<i32>, lo: usize, hi: usize) -> usize {
    let lo_val = data[lo];
    let hi_val = data[hi];
    let mid_val = data[lo + ((hi - lo) / 2)];
    if lo_val < mid_val && lo_val > hi_val {
        return lo;
    } else if mid_val < lo_val && mid_val > hi_val {
        return lo + ((hi - lo) / 2);
    } else {
        return hi;
    }
}
