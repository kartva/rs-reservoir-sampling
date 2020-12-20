use reservoir_sampling::unweighted::{l, /*r*/};

fn main () {
    let arr = (0u32..100).collect::<Vec<u32>>();

    // Working with borrows is necessary as r does not expect T to implement Copy.
    let mut sampled_arr = vec![&0u32; 10];
    l(arr.iter(), &mut sampled_arr[0..]);
    println!("With Algorithm L: {:?}", sampled_arr);
/*
    r(arr.iter(), &mut sampled_arr[0..]);
    println!("With Algorithm R: {:?}", sampled_arr);
*/
}