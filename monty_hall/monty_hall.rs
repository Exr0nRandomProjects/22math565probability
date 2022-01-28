use rand::prelude::*;
use indicatif::ProgressIterator;

const NUM_ITERS: u64 = 5_000_000;

let mut rng = thread_rng();

let mut num_a_wrongblood = 0;
let mut num_b_wrongblood = 0;
let mut num_a_guilty = 0;
let mut num_b_bloodtype = 0;
let mut total = 0;

//for _ in 1..=NUM_ITERS {
for _ in (0..5_000_000).progress() {
    let a_was_guilty: bool = rng.gen();

    let a_has_bloodtype = rng.gen_bool(0.1);
    let b_has_bloodtype = rng.gen_bool(0.1);

    if ! a_has_bloodtype { num_a_wrongblood += 1; continue; }
    if (!a_was_guilty) && (!b_has_bloodtype) { num_b_wrongblood += 1; continue; }

    total += 1;
    if a_was_guilty { num_a_guilty += 1; }
    if b_has_bloodtype { num_b_bloodtype += 1; }
}

println!("num_a_wrongblood  {:.3} {:8}", num_a_wrongblood as f64 / NUM_ITERS as f64, num_a_wrongblood);
println!("num_b_wrongblood  {:.3} {:8}", num_b_wrongblood as f64 / NUM_ITERS as f64, num_b_wrongblood);
println!("successful trials {:.3} {:8}", total            as f64 / NUM_ITERS as f64, total);
println!("num_a_guilty      {:.3} {:8}", num_a_guilty     as f64 / total     as f64, num_a_guilty);
println!("num_b_bloodtype   {:.3} {:8}", num_b_bloodtype  as f64 / total     as f64, num_b_bloodtype);

