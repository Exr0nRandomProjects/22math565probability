#![feature(format_args_capture)]
use rand::prelude::*;

fn simulate(a: u8, b: u8, num: u32) -> f64 {
    // heads = 1, tails = 0
    let mut num_a_wins: u32 = 0;
    for _ in 0..num {
        let mut x: u8 = 0;
        for _ in 0..3 {
            x = (x << 1) | (rand::random::<u8>() & 1);
        }
        num_a_wins += loop { 
            if (x ^ a) & 0b111 == 0 { break 1; }
            if (x ^ b) & 0b111 == 0 { break 0; }
            x = (x << 1) | (rand::random::<u8>() & 1);
        }
    }
    (num_a_wins as f64) / (num as f64)
}

fn main() {
    for a in 0..0b1000 {
        let mut best_b = 0;
        let mut best_b_wins = 0.;
        let mut worst_a_wins = 0.;
        for b in 0..0b1000 {
            let num_wins = simulate(a, b, 10_000);
            let a_wins = num_wins * 100.;
            let b_wins = 100. - a_wins;
            if b_wins > best_b_wins { best_b = b; best_b_wins = b_wins; worst_a_wins = a_wins; }
        }
        println!("{a:03b} {best_b:03b} = {worst_a_wins:.5}, {best_b_wins:.5}");
    }
}
