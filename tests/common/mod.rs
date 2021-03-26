pub fn mean(arr: &[usize]) -> usize {
    arr.iter().copied().fold(0, |acc, curr| curr + acc) / arr.len()
}

/*
// Will be useful for future testing
pub fn sd(arr: &[usize]) -> usize {
    let arr_mean = mean(arr);

    let mut variance: Vec<usize> = arr.iter().copied().map(|n| n - arr_mean).map(|n| n * n).collect();
    mean(&mut variance[..])
}
*/

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
               currently panics so human can see \
               results and verify that results \
               look sort-of random")
    }
}
/*
pub mod weighted {
    use reservoir_sampling::weighted::core::WeightedItem;

    /// TODO: Implement _real_ tests for randomness
    pub fn run (max_tests: usize, f: for<'r> fn(Iterator<Item=WeightedItem<u32>>, u32) -> Vec<u32>) {
        let mut weighted_arr = Vec::new();
        for n in 0..100 {
            weighted_arr.push(WeightedItem::new(n, n as i64))
        }
        let iter = weighted_arr.iter();

        for _ in 0..max_tests {
            let result = f(iter.clone(), 10);
            println!("{:?}", result);
        }

        todo!("Implement tests for randomness, \
               currently panics so human can see \
               results and verify that results \
               look sort-of random")
    }
}
*/