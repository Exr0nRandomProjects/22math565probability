use rayon::prelude::*;
use indicatif::ProgressIterator;
use rand::prelude::*;

const NUM_SIMS: u32 = 100_000_000;
const NUM_FLIPS: u32 = 10;
const NUM_VALS: usize = 3;
let mut total_heads = 0;
let mut total_rounds = 0;
let mut num_times_three = 0;

// reduce
let vals = (0..NUM_SIMS)
    .into_par_iter()
    .map(|_: u32| -> [u32; NUM_VALS] {
        let mut rng = thread_rng();
        let phead = if rng.gen() { 0.5 } else { 0.25 };
        //let phead = 0.25;
        let mut num_heads = 0;
        let mut first_two = 0;
        for i in 0..NUM_FLIPS {
            num_heads += rng.gen_bool(phead) as u32;
            if i == 1 { first_two = num_heads }
        }
        //(num_heads, (num_heads == 3) as u32)
        [num_heads, (num_heads == 3) as u32, (first_two == 2) as u32]
    })
    .reduce(|| [0u32; NUM_VALS], |a, b| [a[0] + b[0], a[1] + b[1], a[2] + b[2]] );
    //.reduce(|| (0u32, 0u32), |a, b| (a.0 as u32 + b.0 as u32, a.1 as u32 + b.1 as u32) );


// fold reduce (slower, because it can't combine and has to do them sequentially)
//let vals = (0..10_000_000)
//    .into_par_iter()
//    .map(|_: u32| -> (u8, bool) {
//        let mut rng = thread_rng();
//        let phead = if rng.gen() { 0.5  } else { 0.25 };
//        let mut num_heads = 0;
//        for _ in 0..NUM_FLIPS {
//            num_heads += rng.gen_bool(phead) as u8;
//        }
//        (num_heads, num_heads == 3)
//    })
//    .fold_with((0u32, 0u32), |a, c| (a.0 + c.0 as u32, a.1 + c.1 as u32))
//    .reduce(|| (0u32, 0u32), |a, b| (a.0 as u32 + b.0 as u32, a.1 as u32 + b.1 as u32) );

//println!("vals {:?}", vals);
println!("average heads     {:.3}", vals[0] as f64/NUM_SIMS as f64);
println!("num times three   {:.6}", vals[1] as f64/NUM_SIMS as f64);
println!("num double head   {:.6}", vals[2] as f64/NUM_SIMS as f64);

//println!("total heads       {:.3}", total_heads);
//println!("average heads     {:.3}", total_heads as f64/total_rounds as f64);
//println!("num times three   {:.6}", num_times_three as f64/total_rounds as f64);
