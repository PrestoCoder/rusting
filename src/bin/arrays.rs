#![allow(unused)]

fn print_slice(arr: &[u8]) {
    println!("arr's 0th element: {}", arr[0]);
}

fn main() {
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[2] = {}", arr[2]);

    let mut arr: [u8; 3] = [1, 2, 3];
    arr[0] = 2;
    println!("arr's length = {}", arr.len());

    // Array slices — different ways to slice
    let arr_slice_0: &[u8; 3] = &arr;          // ref to full fixed-size array
    let arr_slice_1: &[u8] = &arr;             // unsized slice of full array
    let arr_slice_2: &[u8] = &arr[0..1];       // first element only
    let arr_slice_3: &[u8] = &arr[0..2];       // first two
    let arr_slice_4: &[u8] = &arr[0..3];       // all three
    let arr_slice_5: &[u8] = &arr[..1];        // same as 0..1

    println!("full slice length = {}", arr_slice_1.len());
    println!("partial slice length = {}", arr_slice_2.len());

    print_slice(&arr);
}
