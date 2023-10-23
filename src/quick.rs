pub fn quick_sort(arr: &mut [i32]) {
    qs(arr, 0, arr.len() - 1);
}

fn qs(arr: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let pivot_index = partition(arr, low, high);

    qs(arr, low, pivot_index - 1);
    qs(arr, pivot_index + 1, high);
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut index = low - 1;

    for i in 0..high {
        if arr[i] <= pivot {
            index += 1;
            arr.swap(i, index);
        }
    }

    index += 1;
    arr[high] = arr[index];
    arr[index] = pivot;

    index
}
