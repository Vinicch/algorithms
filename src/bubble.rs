pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[test]
fn bubble() {
    let mut arr = [3, 2, 5, 78, 56, 14, 32, 8, 5, 6];

    bubble_sort(&mut arr);

    assert_eq!(arr, [2, 3, 5, 5, 6, 8, 14, 32, 56, 78])
}
