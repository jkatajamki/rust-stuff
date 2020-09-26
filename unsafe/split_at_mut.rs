use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
        )
    }
}

fn main() {
    let mut slice = [1, 2, 3];
    let (one, two) = split_at_mut(&mut slice, 1);
}
