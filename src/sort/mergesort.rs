pub fn mergesort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        _mergesort(arr, 0, len - 1);
    }
}

fn _mergesort<T: Ord + Copy>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let mid = lo + (hi - lo) / 2;
        _mergesort(arr, lo, mid);
        _mergesort(arr, mid + 1, hi);
        _merge(arr, lo, mid, hi);
    }
}

fn _merge<T: Ord + Copy>(arr: &mut [T], lo: usize, mid: usize, hi: usize) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for v in arr.iter().take(mid + 1).skip(lo) {
        // NOTE: take the first `mid + 1` elements and skip the first `lo` elements.
        // In other words, iterate over elements lo, lo + 1, ..., mid.
        left.push(*v);
    }
    for v in arr.iter().take(hi + 1).skip(mid + 1) {
        // NOTE: take the first `hi + 1` elements and skip the first `mid + 1` elements.
        // In other words, iterate over elements mid + 1, mid + 2, ..., hi.
        right.push(*v);
    }

    let lsize = left.len();
    let rsize = right.len();

    let mut l = 0;
    let mut r = 0;
    let mut a = lo;

    while l < lsize && r < rsize {
        if left[l] < right[r] {
            arr[a] = left[l];
            l += 1;
        } else {
            arr[a] = right[r];
            r += 1;
        }
        a += 1;
    }

    while l < lsize {
        arr[a] = left[l];
        l += 1;
        a += 1;
    }
    while r < rsize {
        arr[a] = right[r];
        r += 1;
        a += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::utils::check_sorted;
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        mergesort(&mut res);
        check_sorted(&res);
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        mergesort(&mut res);
        check_sorted(&res);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        mergesort(&mut res);
        check_sorted(&res);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        mergesort(&mut res);
        check_sorted(&res);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        mergesort(&mut res);
        check_sorted(&res);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        mergesort(&mut res);
        check_sorted(&res);
    }
}
