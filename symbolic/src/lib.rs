use bit_set::BitSet;
use nanoid::nanoid;

#[derive(Debug, Hash)]
pub struct EventPossibilities {
    possibilities: Bitset,
    id: String,
}

/// immutability gang
/// has lifetime 's, which is the lifetime of the situation. every probablitiy will be owned by the
/// situation it occupies.
#[derive(Debug, Hash)]
pub enum Probability<'s> {
    UNIT(EventPossibilities),
    COND(&'s Probability<'s>, &'s Probability<'s>),
}

#[derive(Debug)]
pub enum Situation {

}
