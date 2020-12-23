pub fn mean(arr: &[usize]) -> usize {
    arr.iter().copied().fold(0, |acc, curr| curr + acc) / arr.len()
}

pub fn sd(arr: &[usize]) -> usize {
    let arr_mean = mean(arr);

    let mut variance: Vec<usize> = arr.iter().copied().map(|n| n - arr_mean).map(|n| n * n).collect();
    mean(&mut variance[..])
}


pub mod unweighted {
    use super::mean;
    use std::ops::Range;

    /// TODO: Implement _real_ tests for randomness
    pub fn run (max_tests: usize, f: fn(Range<usize>, &mut [usize]) ) {
        let mut sampled_arr = vec![0usize; 10];
        let mut results_vec = Vec::with_capacity(max_tests);

        f(0usize..100, sampled_arr.as_mut_slice());
        results_vec.push(mean(sampled_arr.as_slice()));

        for _ in 0..max_tests {
            f(0usize..100, sampled_arr.as_mut_slice());
            println!("{:?}", sampled_arr.as_slice());
        }

        todo!("Implement tests for randomness, \
               currently panic so human can see \
               results and verify that results \
               look sort-of random")
    }
}