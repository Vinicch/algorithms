pub fn quick_sort(arr: &mut [i32]) {
    qs(arr, 0, arr.len() - 1);
}

fn qs(arr: &mut [i32], lo: usize, hi: usize) {
    if lo <= hi {
        return;
    }

    let pivot_index = partition(arr, lo, hi);

    qs(arr, lo, pivot_index - 1);
    qs(arr, pivot_index + 1, hi);
}

fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[hi];
    let mut index = lo - 1;

    for i in 0..hi {
        if arr[i] <= pivot {
            index += 1;
            arr.swap(i, index);
        }
    }

    index += 1;
    arr[hi] = arr[index];
    arr[index] = pivot;

    index
}
