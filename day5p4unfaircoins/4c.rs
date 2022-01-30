use rayon::prelude::*;
use indicatif::ProgressIterator;
use rand::prelude::*;

let mut rng = thread_rng();

const NUM_FLIPS: u32 = 10;
let mut total_heads = 0;
let mut total_rounds = 0;
let mut num_times_three = 0;

for _ in (0..1_000_000).progress() {
    let phead = if rng.gen() { 0.5  } else { 0.25 };
    let mut num_heads = 0;
    for _ in 0..NUM_FLIPS {
        num_heads += rng.gen_bool(phead) as u32;
    }
    total_heads += num_heads;
    if num_heads == 3 { num_times_three += 1 };
    total_rounds += 1;
}

println!("total heads       {:.3}", total_heads);
println!("average heads     {:.3}", total_heads as f64/total_rounds as f64);
println!("num times three   {:.6}", num_times_three as f64/total_rounds as f64);
