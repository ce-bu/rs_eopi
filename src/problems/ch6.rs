use std::ptr;

///
/// 6.1 THE DUTCH NATIONAL FLAG PROBLEM
///
/// Write a program that takes an array A and an index i into A, and rearranges the elements such that all elements less than A[i] (the "pivot") appear first, followed by elements equal to the pivot, followed by elements greater than the pivot.
///

pub fn dutch_flag_partition<T: Ord>(arr: &mut [T]) {
    if arr.len() == 0 {
        return;
    }
    let pivot = &arr[0] as *const T;
    let mut start = 1;
    let mut end = arr.len();
    while start < end {
        let x = unsafe { &*pivot as &T };
        if &arr[start] <= x {
            start = start + 1;
        } else {
            end = end - 1;
            unsafe {
                ptr::swap(&mut arr[start], &mut arr[end]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dutch_flag_partition() {
        let mut arr1 = vec![
            14, 30, 4, 3, 19, 16, 9, 20, 2, 7, 12, 8, 13, 1, 10, 11, 18, 6, 15, 14, 5,
        ];
        dutch_flag_partition(&mut arr1);
        println!("{:?}", arr1);
    }
}
