use bit_set::BitSet;
use nanoid::nanoid;

#[derive(Debug, Hash)]
pub struct EventPossibilities {
    possibilities: BitSet,
    id: String,
}

/// immutability gang
/// has lifetime 's, which is the lifetime of the situation. every probablitiy will be owned by the
/// situation it occupies.
#[derive(Debug, Hash)]
pub enum Probability<'s> {
    UNIT(EventPossibilities),                           // P(X)
    COND(&'s Probability<'s>, &'s Probability<'s>),     // P(A | B)
    OR  (&'s Probability<'s>, &'s Probability<'s>),     // P(A u B)
    AND (&'s Probability<'s>, &'s Probability<'s>),     // P(A n B)     not MUL: doesn't assume independence
    ADD (&'s Probability<'s>, &'s Probability<'s>),     // P(A) + P(B)  not OR: doesn't assume disjointness
    MUL (&'s Probability<'s>, &'s Probability<'s>),     // P(A) * P(B)  only for algebraic purposes
    DIV (&'s Probability<'s>, &'s Probability<'s>),     // P(A) / P(B)  only for algebraic purposes
    SUB (&'s Probability<'s>, &'s Probability<'s>),     // P(A) - P(B)  only for algebraic purposes
}

#[derive(Debug)]
pub enum Situation {

}
