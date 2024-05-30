use super::customers::CustomerOfProperty;

fn partition(arr: &mut [CustomerOfProperty], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize].budget >= arr[pivot].budget {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot);
    i + 1
}

pub fn quicksort(arr: &mut [CustomerOfProperty], low: isize, high: isize) {
    if low < high {
        let pi = partition(arr, low, high);

        quicksort(arr, low, pi - 1);
        quicksort(arr, pi + 1, high);
    }
}
