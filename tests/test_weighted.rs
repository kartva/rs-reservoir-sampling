use reservoir_sampling::weighted::*;
use reservoir_sampling::weighted::core::WeightedItem;

mod common;
//use common::weighted::run;

#[test]
fn test_a_res () {
        for _ in 0..10 {
            let mut weighted_arr = Vec::new();
            for n in 0..100 {
                weighted_arr.push(WeightedItem::new(n, n as i64))
            }

            let result = a_res(weighted_arr.into_iter(), 10);
            println!("{:?}", result);
        }

        todo!("Implement tests for randomness, \
                currently panics so human can see \
                results and verify that results \
                look sort-of random")
}
