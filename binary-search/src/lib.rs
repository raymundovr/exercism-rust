use std::cmp::Ordering;

fn find_rec(array: &[i32], key: i32, offset: usize) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    let middle: usize = array.len() / 2;
    let middle_val = array[middle];

    match key.cmp(&middle_val) {
        Ordering::Equal => Some(offset + middle),
        Ordering::Greater => find_rec(&array[(middle + 1)..], key, offset + middle + 1),
        Ordering::Less => find_rec(&array[..middle], key, offset),
    }
}

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    find_rec(array, key, 0)
}