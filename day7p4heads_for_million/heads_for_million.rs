use rand::prelude::*;
use rayon::prelude::*;

const NUM_ITERS: u32 = 10_000;

fn get_expected(delta: u64) -> f64 {
    (0..NUM_ITERS)
        .into_par_iter()
        //.map(|v| { println!("on number {v}"); v })
        //.map(|v| { println!("o"); v })
        .map(|id| {
            let mut tot: u64 = 0;
            let mut win: u64 = 0;
            let mut rng = thread_rng();
            loop {
                tot += 1;
                if rng.gen::<bool>() { win += 1; }
                //if win > (tot - win) + 2 { println!("won {id} after {tot}!"); break win as f64 / tot as f64 }
                if win >= (tot - win) + delta { break win as f64 / tot as f64 }
            }
        })
        .reduce(|| 0f64, |a, b| a + b)
}

for d in (0..=5) {
    println!("average win: {:.5} for delta {}", get_expected(d) / NUM_ITERS as f64, d);
}
