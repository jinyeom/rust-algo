pub fn bubblesort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::utils::check_sorted;
    use super::*;

    #[test]
    fn descending() {
        let mut v = vec![6, 5, 4, 3, 2, 1];
        bubblesort(&mut v);
        check_sorted(&v);
    }

    #[test]
    fn ascending() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        bubblesort(&mut v);
        check_sorted(&v);
    }

    #[test]
    fn chars() {
        let mut v = vec!['r', 'u', 's', 't'];
        bubblesort(&mut v);
        check_sorted(&v);
    }
}
