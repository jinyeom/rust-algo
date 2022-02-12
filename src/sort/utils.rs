pub fn check_sorted<T: Ord>(arr: &[T]) {
    if arr.len() > 0 {
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
