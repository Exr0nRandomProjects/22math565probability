use rand::prelude::*;
use rayon::prelude::*;
use indicatif::ProgressIterator;

const NUM_ITERS: u32 = 100_000;

//fn get_expected_parallel(give_up_predicate: &dyn Fn(u64, u64) -> bool,
//                win_predicate: &dyn Fn(u64, u64) -> bool) -> (f64, f64) {
//    let (num_win, tot_score) = (0..NUM_ITERS)
//        .into_par_iter()
//        //.map(|v| { println!("on number {v}"); v })
//        //.map(|v| { println!("o"); v })
//        .map(|id| {
//            let mut tot: u64 = 0;
//            let mut win: u64 = 0;
//            let mut rng = thread_rng();
//            loop {
//                tot += 1;
//                if rng.gen::<bool>() { win += 1; }
//                //if win > (tot - win) + 2 { println!("won {id} after {tot}!"); break win as f64 / tot as f64 }
//                //if win >= (tot - win) + delta { break win as f64 / tot as f64 }
//                if give_up_predicate(tot, win) { break (0u64, win as f64 / tot as f64) }
//                if win_predicate(tot, win) { break (1u64, win as f64 / tot as f64) }
//            }
//        })
//        .reduce(|| (0u64, 0f64), |a, b| (a.0 + b.0, a.1 + b.1));
//    (num_win as f64 / NUM_ITERS as f64, tot_score / NUM_ITERS as f64)
//}

fn get_expected(give_up_predicate: &dyn Fn(u64, u64) -> bool,
                win_predicate: &dyn Fn(u64, u64) -> bool) -> (f64, f64) {
    let (num_win, tot_score) = (0..NUM_ITERS).progress()
        //.into_par_iter()
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
                //if win >= (tot - win) + delta { break win as f64 / tot as f64 }
                if give_up_predicate(tot, win) { break (0u64, (win as f64 / tot as f64).max(0.5)) }
                if win_predicate(tot, win) { break (1u64, win as f64 / tot as f64) }
            }
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1)).expect("no runs!");
    (num_win as f64 / NUM_ITERS as f64, tot_score / NUM_ITERS as f64)
}

// main
//for d in 0..=5 {
//    let (num_success, avg_score) = get_expected(
//        {
//            &|t, _| t >= 1_00_000
//        },
//        {
//            //let d = d.clone();
//            &|t, w| w >= (t-w) + d
//        });
//    println!("delta {:5} | average win: {:.5} with win rate {:.5}", avg_score, num_success, d);
//}

fn huxleys_thing(thresh: u64, anneal: &dyn Fn(u64) -> f64) -> (f64, f64) {
    get_expected(
    {
        &|t, _| t >= thresh
    },
    {
        //let d = d.clone();
        &|t, w| if t > thresh { false } else { anneal(t) <= (w as f64/t as f64) }
    })
}

let thresh = 1_000_000;
let c = 0.57;    // up to 1
let (num_success, avg_score) = huxleys_thing(thresh, &|x|
                 ((1f64 + c*(-(x as f64)/(thresh as f64)+1f64).sqrt())/2f64));
println!("huxey plan | average win: {:.5} with win rate {:.5} with thresh {} and c {}", avg_score, num_success, thresh, c);
