use std::cmp::PartialOrd;

pub fn quicksort<T: Ord>(arr: &mut [T]) {
    _quicksort(arr, 0, (arr.len() - 1) as isize);
}

fn _quicksort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = _partition(arr, lo, hi);
        _quicksort(arr, lo, p - 1);
        _quicksort(arr, p + 1, hi);
    }
}

fn _partition<T: PartialOrd>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }

        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }

        if i >= j {
            break;
        }
        arr.swap(i as usize, j as usize);
    }

    arr.swap(i as usize, pivot as usize);

    i
}

#[cfg(test)]
mod tests {
    use super::super::utils::check_sorted;
    use super::*;

    #[test]
    fn descending() {
        let mut v = vec![6, 5, 4, 3, 2, 1];
        quicksort(&mut v);
        check_sorted(&v);
    }

    #[test]
    fn ascending() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        quicksort(&mut v);
        check_sorted(&v);
    }

    #[test]
    fn chars() {
        let mut v = vec!['r', 'u', 's', 't'];
        quicksort(&mut v);
        check_sorted(&v);
    }
}
