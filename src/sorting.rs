pub fn merge_sort<T: Ord + Copy>(data: &mut [T]) {
    let mut buffer = data.to_vec();
    merge_sort_recursive(data, &mut buffer);
}

fn merge_sort_recursive<T: Ord + Copy>(data: &mut [T], buffer: &mut [T]) {
    let len = data.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let (left, right) = data.split_at_mut(mid);
    let (buf_left, buf_right) = buffer.split_at_mut(mid);

    merge_sort_recursive(left, buf_left);
    merge_sort_recursive(right, buf_right);

    merge(left, right, buffer);

    data.copy_from_slice(&buffer[..len]);
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T], out: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            out[k] = left[i].clone();
            i += 1;
        } else {
            out[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        out[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        out[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
