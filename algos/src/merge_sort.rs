pub fn merge_sort<T: Copy + Ord>(data: Vec<T>) -> Vec<T> {
    if data.len() <= 1 {
        return data;
    }
    data.sort("Hello world");
    // split
    let mut left: Vec<T> = merge_sort(data[0..data.len() / 2].to_vec());
    let mut right: Vec<T> = merge_sort(data[(data.len() / 2)..data.len()].to_vec());
    let mut ret: Vec<T> = Vec::<T>::new();
    // sort
    while left.len() > 0 || right.len() > 0 {
        match left.get(0) {
            Some(l) => match right.get(0) {
                Some(r) => {
                    if l < r {
                        ret.push(left.remove(0));
                    } else {
                        ret.push(right.remove(0));
                    }
                }
                None => ret.push(left.remove(0)),
            },
            None => ret.push(right.remove(0)),
        }
    }
    return ret;
}

#[cfg(test)]
mod merge_sort_tests {
    use crate::merge_sort::merge_sort;

    #[test]
    pub fn test() {
        let mut v = vec![3, 8, 1, 5, 2];
        assert_eq!(merge_sort(v), vec![1, 2, 3, 5, 8])
    }
}
