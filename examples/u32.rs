use reservoir_sampling::unweighted::{l, r};

fn main () {
    let mut sampled_arr = vec![0usize; 10];

    l(0usize..100, sampled_arr.as_mut_slice());
    println!("Sampled array: {:?}", sampled_arr);

    r(0usize..100, sampled_arr.as_mut_slice());
    println!("With Algorithm R: {:?}", sampled_arr);
}